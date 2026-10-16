use anyhow::{bail, Context, Result};
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use std::time::Instant;
use tokio::process::Command;
use tokio::sync::Semaphore;
use uuid::Uuid;

static GS_SEMAPHORE: OnceLock<Semaphore> = OnceLock::new();
/// Cached Ghostscript binary path — resolved once at startup, never via shell lookup again.
static GS_PATH: OnceLock<PathBuf> = OnceLock::new();

fn get_semaphore() -> &'static Semaphore {
    GS_SEMAPHORE.get_or_init(|| {
        let cores = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(2);
        // Allow 2x cores concurrent passes so multi-file requests parallelise better
        Semaphore::new((cores * 2).max(2))
    })
}

fn gs_binary() -> &'static Path {
    GS_PATH.get_or_init(|| {
        // Common install locations — checked in-process, no extra fork/exec needed
        for candidate in &["/usr/bin/gs", "/usr/local/bin/gs", "/opt/homebrew/bin/gs"] {
            if Path::new(candidate).exists() {
                return PathBuf::from(candidate);
            }
        }
        PathBuf::from("gs") // fall back to PATH
    })
}

/// The base temp directory for FitKB processing. Prefers /dev/shm (RAM disk) on Linux if writable.
pub fn get_temp_dir() -> PathBuf {
    let shm = Path::new("/dev/shm/fitkb");
    if shm.exists() || std::fs::create_dir_all(shm).is_ok() {
        shm.to_path_buf()
    } else {
        let tmp = std::env::temp_dir().join("fitkb");
        let _ = std::fs::create_dir_all(&tmp);
        tmp
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionMode {
    Extreme,           // Smallest size, lower quality (~72 DPI /screen)
    Recommended,       // Balanced quality & size (~150 DPI /ebook)
    Less,              // High quality, bigger size (~300 DPI /printer)
    TargetSize(usize), // Target bytes (e.g. 100_000, 200_000, 1_048_576)
}

impl CompressionMode {
    pub fn parse(mode_str: &str, target_kb: Option<usize>) -> Self {
        match mode_str {
            "extreme" => Self::Extreme,
            "less" => Self::Less,
            "target" => {
                let kb = target_kb.unwrap_or(200);
                Self::TargetSize(kb * 1024)
            }
            _ => Self::Recommended,
        }
    }

    pub fn gs_setting(&self) -> &'static str {
        match self {
            Self::Extreme => "/screen",
            Self::Recommended => "/ebook",
            Self::Less => "/printer",
            Self::TargetSize(_) => "/ebook",
        }
    }
}

#[derive(Debug, Clone)]
pub struct ProcessedPdf {
    pub id: String,
    pub original_name: String,
    pub original_size: usize,
    pub compressed_size: usize,
    pub percent_saved: f64,
    pub file_path: PathBuf,
    pub duration_ms: u128,
    pub reached_target: bool,
    pub target_bytes: Option<usize>,
    pub message: Option<String>,
}

/// Validates PDF magic bytes, maximum size, and verifies it is not password-protected or corrupted.
pub async fn validate_pdf_file(file_path: &Path) -> Result<()> {
    use tokio::io::AsyncReadExt;

    let metadata = tokio::fs::metadata(file_path).await?;
    if metadata.len() == 0 {
        bail!("The uploaded file is empty.");
    }
    if metadata.len() > 50 * 1024 * 1024 {
        bail!("File exceeds maximum size limit of 50 MB.");
    }

    let mut file = tokio::fs::File::open(file_path).await?;
    let mut header = [0u8; 1024];
    let bytes_read = file.read(&mut header).await?;
    if bytes_read < 4 || !header[..bytes_read].starts_with(b"%PDF") {
        bail!("The uploaded file is not a valid PDF document (missing %PDF signature).");
    }

    let mut buf = Vec::new();
    file.read_to_end(&mut buf).await?;
    let content = [&header[..bytes_read], buf.as_slice()].concat();

    if content.windows(8).any(|w| w == b"/Encrypt") {
        bail!("This PDF is password-protected or encrypted. Please remove password protection before compressing.");
    }

    Ok(())
}

/// Executes Ghostscript securely with -dSAFER, speed-optimised flags, and a hard timeout.
pub async fn run_gs_pass(
    input_path: &Path,
    output_path: &Path,
    pdf_setting: &str,
    image_res: Option<u32>,
    jpeg_quality: Option<u8>,
) -> Result<bool> {
    let _permit = get_semaphore().acquire().await?;

    let mut cmd = Command::new(gs_binary());
    cmd.arg("-sDEVICE=pdfwrite")
        .arg("-dCompatibilityLevel=1.4")
        .arg("-dSAFER")
        .arg("-dNOPAUSE")
        .arg("-dQUIET")
        .arg("-dBATCH")
        // ── Speed flags ───────────────────────────────────────────────────────
        // Linearisation is an extra rewrite pass; users download immediately so skip it
        .arg("-dFastWebView=false")
        // Subset + compress embedded fonts — reduces size AND parse time
        .arg("-dCompressFonts=true")
        .arg("-dSubsetFonts=true")
        // CMYK→RGB saves ~25% on colour images in CMYK print-spec PDFs
        .arg("-dConvertCMYKImagesToRGB=true")
        // Deduplicate identical XObjects (common in scanned multi-page docs)
        .arg("-dDetectDuplicateImages=true")
        // Skip bilinear interpolation — only affects on-screen rendering, not output quality
        .arg("-dNOINTERPOLATE")
        // ─────────────────────────────────────────────────────────────────────
        .arg(format!("-dPDFSETTINGS={}", pdf_setting));

    if let Some(res) = image_res {
        cmd.arg("-dDownsampleColorImages=true")
            .arg("-dDownsampleGrayImages=true")
            .arg("-dDownsampleMonoImages=true")
            .arg(format!("-dColorImageResolution={}", res))
            .arg(format!("-dGrayImageResolution={}", res))
            .arg(format!("-dMonoImageResolution={}", res));
    }

    if let Some(q) = jpeg_quality {
        cmd.arg("-dAutoFilterColorImages=false")
            .arg("-dColorImageFilter=/DCTEncode")
            .arg(format!("-dJPEGQ={}", q));
    }

    cmd.arg(format!("-sOutputFile={}", output_path.display()))
        .arg(input_path);

    let child = cmd.spawn().context("Failed to spawn Ghostscript process")?;

    match tokio::time::timeout(std::time::Duration::from_secs(120), child.wait_with_output()).await {
        Ok(Ok(output)) => {
            if output.status.success() && output_path.exists() {
                Ok(true)
            } else {
                let err = String::from_utf8_lossy(&output.stderr);
                bail!("Ghostscript execution error: {}", err);
            }
        }
        Ok(Err(e)) => bail!("Process error: {}", e),
        Err(_) => bail!("Compression timed out after 120 seconds. The file may be extraordinarily dense or corrupt."),
    }
}

/// Compress a PDF given a mode or exact target size.
pub async fn compress_pdf_file(
    input_path: &Path,
    original_name: String,
    mode: CompressionMode,
) -> Result<ProcessedPdf> {
    let start = Instant::now();
    let original_size = tokio::fs::metadata(input_path).await?.len() as usize;

    let temp_dir = get_temp_dir();
    let id = Uuid::new_v4().to_string();
    let output_path = temp_dir.join(format!("{}.pdf", id));

    // ── Fast path: very small file — GS overhead exceeds any gain ─────────────
    if original_size < 50 * 1024 {
        let _ = tokio::fs::copy(input_path, &output_path).await;
        return Ok(ProcessedPdf {
            id,
            original_name,
            original_size,
            compressed_size: original_size,
            percent_saved: 0.0,
            file_path: output_path,
            duration_ms: start.elapsed().as_millis(),
            reached_target: true,
            target_bytes: None,
            message: Some("File is already very small. Kept original.".to_string()),
        });
    }

    let mut reached_target = true;
    let mut target_bytes = None;
    let mut message = None;

    match mode {
        CompressionMode::Extreme | CompressionMode::Recommended | CompressionMode::Less => {
            let setting = mode.gs_setting();
            let success = run_gs_pass(input_path, &output_path, setting, None, None).await?;
            if !success {
                bail!("Compression pass did not complete.");
            }
        }
        CompressionMode::TargetSize(target) => {
            target_bytes = Some(target);

            // ── Fast path: already at or below target, no GS needed ────────────
            if original_size <= target {
                let _ = tokio::fs::copy(input_path, &output_path).await;
                return Ok(ProcessedPdf {
                    id,
                    original_name,
                    original_size,
                    compressed_size: original_size,
                    percent_saved: 0.0,
                    file_path: output_path,
                    duration_ms: start.elapsed().as_millis(),
                    reached_target: true,
                    target_bytes: Some(target),
                    message: Some("File is already under the target size. Kept original.".to_string()),
                });
            }

            // Multi-tier search: moderate → screen → aggressive
            let temp_pass1 = temp_dir.join(format!("{}_p1.pdf", id));
            let _ = run_gs_pass(input_path, &temp_pass1, "/ebook", None, None).await;
            let size_p1 = tokio::fs::metadata(&temp_pass1).await.map(|m| m.len() as usize).unwrap_or(usize::MAX);

            if size_p1 <= target {
                let _ = tokio::fs::rename(&temp_pass1, &output_path).await;
            } else {
                let temp_pass2 = temp_dir.join(format!("{}_p2.pdf", id));
                let _ = run_gs_pass(input_path, &temp_pass2, "/screen", Some(72), Some(60)).await;
                let size_p2 = tokio::fs::metadata(&temp_pass2).await.map(|m| m.len() as usize).unwrap_or(usize::MAX);

                if size_p2 <= target {
                    let _ = tokio::fs::rename(&temp_pass2, &output_path).await;
                    let _ = tokio::fs::remove_file(&temp_pass1).await;
                } else {
                    let temp_pass3 = temp_dir.join(format!("{}_p3.pdf", id));
                    let _ = run_gs_pass(input_path, &temp_pass3, "/screen", Some(50), Some(40)).await;
                    let size_p3 = tokio::fs::metadata(&temp_pass3).await.map(|m| m.len() as usize).unwrap_or(usize::MAX);

                    if size_p3 <= target {
                        let _ = tokio::fs::rename(&temp_pass3, &output_path).await;
                    } else {
                        reached_target = false;
                        message = Some(format!(
                            "Could not reach {} KB without severe degradation. Showing smallest achievable result.",
                            target / 1024
                        ));
                        let best = [(&temp_pass1, size_p1), (&temp_pass2, size_p2), (&temp_pass3, size_p3)]
                            .into_iter()
                            .min_by_key(|&(_, s)| s)
                            .map(|(p, _)| p)
                            .unwrap_or(&temp_pass3);
                        let _ = tokio::fs::copy(best, &output_path).await;
                    }
                    let _ = tokio::fs::remove_file(&temp_pass1).await;
                    let _ = tokio::fs::remove_file(&temp_pass2).await;
                    let _ = tokio::fs::remove_file(&temp_pass3).await;
                }
            }
        }
    }

    let compressed_size = tokio::fs::metadata(&output_path)
        .await
        .map(|m| m.len() as usize)
        .unwrap_or(original_size);

    // Invariant: never return a file bigger than the input
    if compressed_size >= original_size {
        let _ = tokio::fs::copy(input_path, &output_path).await;
        Ok(ProcessedPdf {
            id,
            original_name,
            original_size,
            compressed_size: original_size,
            percent_saved: 0.0,
            file_path: output_path,
            duration_ms: start.elapsed().as_millis(),
            reached_target: true,
            target_bytes,
            message: Some("Your PDF is already well optimized. Kept original file.".to_string()),
        })
    } else {
        let bytes_saved = original_size - compressed_size;
        let percent_saved = (bytes_saved as f64 / original_size as f64) * 100.0;
        Ok(ProcessedPdf {
            id,
            original_name,
            original_size,
            compressed_size,
            percent_saved,
            file_path: output_path,
            duration_ms: start.elapsed().as_millis(),
            reached_target,
            target_bytes,
            message,
        })
    }
}
