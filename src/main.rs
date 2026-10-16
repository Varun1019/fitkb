mod engine;
mod templates;

use axum::{
    extract::{DefaultBodyLimit, Multipart, Path as AxumPath},
    http::{header, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Router,
};
use engine::{compress_pdf_file, get_temp_dir, validate_pdf_file, CompressionMode, ProcessedPdf};
use maud::{html, Markup};
use serde::Serialize;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use templates::{base_layout, SeoMetadata};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tower_http::compression::CompressionLayer;
use tower_http::set_header::SetResponseHeaderLayer;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;
use uuid::Uuid;

const MAX_UPLOAD_LIMIT: usize = 50 * 1024 * 1024; // 50 MB

// -------------------------------------------------------------------------------------------------
// UI Components
// -------------------------------------------------------------------------------------------------

fn render_ilovepdf_box(preselected_target_kb: Option<usize>) -> Markup {
    let target_val = preselected_target_kb.unwrap_or(200);

    html! {
        div id="compress-tool" style="max-width:880px; margin:0 auto; padding:0 1rem;" {
            // Main Tool Card Container
            div class="tool-card" {

                // 1. Initial Upload Dropzone
                div id="upload-area" {
                    input id="file-input" type="file" accept="application/pdf" multiple style="display:none;";
                    div id="dropzone" class="dropzone-container" {
                        div class="upload-icon-circle" {
                            svg width="34" height="34" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
                                path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4";
                                polyline points="17 8 12 3 7 8";
                                line x1="12" y1="3" x2="12" y2="15";
                            }
                        }
                        h2 style="font-size:1.6rem; font-weight:800; color:var(--slate-900); margin-bottom:0.5rem;" {
                            "Select PDF Files"
                        }
                        p style="font-size:1rem; color:var(--slate-500); margin-bottom:1.5rem;" {
                            "or drag and drop your PDFs here (up to 50 MB each)"
                        }
                        div {
                            button type="button" class="btn btn-primary btn-lg" style="pointer-events:none;" {
                                svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
                                    path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z";
                                    polyline points="14 2 14 8 20 8";
                                    line x1="12" y1="18" x2="12" y2="12";
                                    line x1="9" y1="15" x2="15" y2="15";
                                }
                                "Choose PDF Files"
                            }
                        }
                        p style="font-size:0.8rem; color:var(--slate-400); margin-top:1.25rem;" {
                            "Tip: Works on all phones, tablets & desktops • Paste via Ctrl+V supported"
                        }
                    }
                }

                // 2. File Queue & Compression Level Selector
                div id="file-queue-section" style="display:none;" {
                    div style="display:flex; justify-content:space-between; align-items:center; border-bottom:1px solid var(--slate-200); padding-bottom:1rem; margin-bottom:1.5rem;" {
                        h3 style="font-size:1.15rem; font-weight:700; color:var(--slate-800);" { "Selected PDF Documents" }
                        button id="add-more-btn" type="button" class="btn btn-secondary btn-sm" {
                            "+ Add More"
                        }
                    }

                    div id="file-list-container" style="margin-bottom:1.75rem;" {}

                    // Mode Selection
                    div style="margin-bottom:1.75rem;" {
                        label style="font-size:0.95rem; font-weight:700; color:var(--slate-700); display:block; margin-bottom:0.75rem;" {
                            "Choose Compression Level:"
                        }
                        div class="mode-cards-grid" {
                            div id="card-recommended" class="mode-card selected" {
                                div class="mode-card-header" {
                                    span class="mode-title" { "Recommended" }
                                    span class="badge-popular" { "Best Choice" }
                                }
                                p class="mode-desc" { "Balanced compression (150 DPI). Ideal for standard documents & resumes." }
                            }
                            div id="card-extreme" class="mode-card" {
                                div class="mode-card-header" {
                                    span class="mode-title" { "Extreme" }
                                }
                                p class="mode-desc" { "Maximum size reduction (72 DPI). Smallest achievable file size." }
                            }
                            div id="card-less" class="mode-card" {
                                div class="mode-card-header" {
                                    span class="mode-title" { "Less" }
                                }
                                p class="mode-desc" { "Gentle compression preserving print-grade clarity (300 DPI)." }
                            }
                            div id="card-target" class="mode-card" {
                                div class="mode-card-header" {
                                    span class="mode-title" { "Exact Target" }
                                }
                                p class="mode-desc" { "Compresses strictly under 100 KB, 200 KB, or 1 MB for strict portal limits." }
                            }
                        }

                        // Target Size Sub-Controls (shown when Exact Target is chosen)
                        div id="target-controls" style="display:none; margin-top:1.25rem; padding:1.25rem; background:var(--slate-50); border:1px solid var(--slate-200); border-radius:var(--radius-md);" {
                            div style="font-weight:600; font-size:0.85rem; color:var(--slate-700); margin-bottom:0.75rem;" {
                                "Target Maximum File Size:"
                            }
                            div style="display:flex; flex-wrap:wrap; gap:0.5rem; align-items:center;" {
                                button type="button" data-target="100" class=(format!("target-pill {}", if target_val == 100 { "active" } else { "" })) { "Under 100 KB" }
                                button type="button" data-target="200" class=(format!("target-pill {}", if target_val == 200 { "active" } else { "" })) { "Under 200 KB" }
                                button type="button" data-target="500" class=(format!("target-pill {}", if target_val == 500 { "active" } else { "" })) { "Under 500 KB" }
                                button type="button" data-target="1024" class=(format!("target-pill {}", if target_val == 1024 { "active" } else { "" })) { "Under 1 MB" }
                                input id="custom-target-input" type="number" placeholder="Custom KB..." style="padding:0.45rem 0.85rem; border:1px solid var(--slate-300); border-radius:var(--radius-full); font-size:0.85rem; width:130px; outline:none; transition:border 0.15s ease;";
                            }
                        }
                    }

                    // Compress Action Button
                    div style="margin-top:1.75rem;" {
                        button id="compress-btn" type="button" class="btn btn-primary btn-lg" style="width:100%;" {
                            "Compress PDF →"
                        }
                    }
                }

                // 3. Realistic Progress Stage
                div id="progress-stage" class="progress-card" style="display:none;" {
                    div class="progress-spinner" {}
                    div id="progress-percent" class="progress-headline" {
                        "0%"
                    }
                    div id="stage-label" class="progress-subtext" {
                        "Establishing secure in-memory connection..."
                    }
                    div class="progress-bar-outer" {
                        div id="progress-bar" class="progress-bar-fill" {}
                    }
                    div class="progress-meta" {
                        span id="progress-byte-meta" { "Connecting..." }
                        span id="progress-speed-meta" { "Stage: Upload" }
                    }
                }

                // 4. In-Place Result Screen (Next to action, zero scrolling required)
                div id="result-stage" style="display:none; padding:0.5rem 0;" {
                    div style="text-align:center; padding-bottom:1.5rem; border-bottom:1px solid var(--slate-200); margin-bottom:1.5rem;" {
                        div class="savings-stat-badge" style="margin-bottom:0.75rem;" {
                            "✓ Compression Complete"
                        }
                        h2 id="result-headline" style="font-size:2.5rem; font-weight:800; color:var(--slate-900); margin-bottom:0.4rem;" {
                            "70% Smaller"
                        }
                        p id="result-subtitle" style="font-size:1rem; font-weight:600; color:var(--slate-600);" {
                            "Your PDF was successfully reduced without losing readability."
                        }
                    }

                    div id="result-list" style="margin-bottom:1.75rem;" {}

                    div style="display:flex; flex-wrap:wrap; justify-content:center; align-items:center; gap:0.85rem;" {
                        a id="single-download-btn" href="#" class="btn btn-success btn-lg" style="min-width:240px;" {
                            svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
                                path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4";
                                polyline points="7 10 12 15 17 10";
                                line x1="12" y1="15" x2="12" y2="3";
                            }
                            "Download Compressed PDF"
                        }
                        a id="zip-download-btn" href="#" class="btn btn-success btn-lg" style="display:none; min-width:240px;" {
                            svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
                                path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4";
                                polyline points="7 10 12 15 17 10";
                                line x1="12" y1="15" x2="12" y2="3";
                            }
                            "Download All as ZIP"
                        }
                        button id="restart-btn" type="button" class="btn btn-secondary btn-lg" {
                            "Compress Another File"
                        }
                    }
                }

                // Error Notice
                div id="error-notice" style="display:none; background:#FEF2F2; border:1px solid #FCA5A5; color:#991B1B; padding:1rem 1.25rem; border-radius:var(--radius-md); margin-top:1.5rem; font-size:0.9rem; font-weight:500;" {
                    span id="error-message" {}
                }
            }

            // Trust Badges Grid
            div style="display:grid; grid-template-columns:repeat(auto-fit, minmax(210px, 1fr)); gap:1rem;" {
                div style="display:flex; align-items:flex-start; gap:0.75rem; padding:1.15rem; background:#ffffff; border:1px solid var(--slate-200); border-radius:var(--radius-lg); box-shadow:var(--shadow-sm);" {
                    div style="width:38px; height:38px; background:#EFF6FF; border-radius:var(--radius-md); display:flex; align-items:center; justify-content:center; color:var(--primary); font-size:1.1rem; flex-shrink:0;" {
                        "🔒"
                    }
                    div {
                        h4 style="font-size:0.92rem; font-weight:700; color:var(--slate-800); margin-bottom:0.2rem;" { "100% In-Memory" }
                        p style="font-size:0.78rem; color:var(--slate-500); line-height:1.4;" { "Processed securely in volatile RAM and deleted immediately upon download." }
                    }
                }
                div style="display:flex; align-items:flex-start; gap:0.75rem; padding:1.15rem; background:#ffffff; border:1px solid var(--slate-200); border-radius:var(--radius-lg); box-shadow:var(--shadow-sm);" {
                    div style="width:38px; height:38px; background:#ECFDF5; border-radius:var(--radius-md); display:flex; align-items:center; justify-content:center; color:var(--success); font-size:1.1rem; flex-shrink:0;" {
                        "⚡"
                    }
                    div {
                        h4 style="font-size:0.92rem; font-weight:700; color:var(--slate-800); margin-bottom:0.2rem;" { "Fast Rust Core" }
                        p style="font-size:0.78rem; color:var(--slate-500); line-height:1.4;" { "Native parallel multi-file processing with zero queue wait times." }
                    }
                }
                div style="display:flex; align-items:flex-start; gap:0.75rem; padding:1.15rem; background:#ffffff; border:1px solid var(--slate-200); border-radius:var(--radius-lg); box-shadow:var(--shadow-sm);" {
                    div style="width:38px; height:38px; background:#FEF3C7; border-radius:var(--radius-md); display:flex; align-items:center; justify-content:center; color:#D97706; font-size:1.1rem; flex-shrink:0;" {
                        "✨"
                    }
                    div {
                        h4 style="font-size:0.92rem; font-weight:700; color:var(--slate-800); margin-bottom:0.2rem;" { "Free & Unlimited" }
                        p style="font-size:0.78rem; color:var(--slate-500); line-height:1.4;" { "No sign-up, no hidden subscriptions, and never any watermarks." }
                    }
                }
            }
        }
    }
}

async fn home_page() -> Html<String> {
    let seo = SeoMetadata {
        title: "Compress PDF Online Free — Fast & High Quality | FitKB",
        description: "Compress PDF files online for free. Reduce file size while keeping crisp quality. Safe, instant in-memory processing with no email or registration required.",
        canonical_url: "https://fitkb.org/",
        h1: "Compress PDF Online",
        preselected_target_kb: None,
    };

    let content = html! {
        // Welcoming Hero Section (Clean & Focused)
        section style="padding:3.25rem 1.25rem 1.75rem 1.25rem; text-align:center; max-width:820px; margin:0 auto;" {
            div style="display:inline-flex; align-items:center; gap:0.4rem; background:var(--primary-light); color:var(--primary); font-size:0.85rem; font-weight:700; padding:0.4rem 1rem; border-radius:var(--radius-full); margin-bottom:1.25rem;" {
                "✨ Private, Fast & Unlimited PDF Compression"
            }
            h1 style="font-size:clamp(2.2rem, 5vw, 3.5rem); font-weight:800; color:var(--slate-900); margin-bottom:0.85rem; letter-spacing:-0.03em;" {
                "Compress PDF Online " br;
                span style="color:var(--primary);" { "Smaller Size. Crisp Quality." }
            }
            p style="font-size:1.1rem; color:var(--slate-600); max-width:640px; margin:0 auto 2.25rem auto; line-height:1.6;" {
                "Quickly shrink heavy PDF documents for email attachments, job applications, or web publishing. Completely private, works on any device, and requires zero sign-up."
            }
        }

        // The Compression Tool (Centerpiece)
        (render_ilovepdf_box(None))

        // Key Features (Why FitKB)
        section id="features" style="max-width:1100px; margin:4.5rem auto 0 auto; padding:0 1.25rem;" {
            div style="text-align:center; margin-bottom:2.5rem;" {
                h2 style="font-size:1.85rem; font-weight:800; color:var(--slate-900); margin-bottom:0.5rem;" {
                    "Engineered for Speed, Clarity, and Privacy"
                }
                p style="font-size:0.95rem; color:var(--slate-500);" {
                    "Here's why millions choose FitKB for their daily document compression."
                }
            }
            div style="display:grid; grid-template-columns:repeat(auto-fit, minmax(260px, 1fr)); gap:1.5rem;" {
                div style="background:#ffffff; border:1px solid var(--slate-200); border-radius:var(--radius-lg); padding:1.75rem; box-shadow:var(--shadow-sm);" {
                    div style="width:44px; height:44px; background:#EFF6FF; border-radius:var(--radius-md); display:flex; align-items:center; justify-content:center; color:var(--primary); font-size:1.3rem; margin-bottom:1rem;" {
                        "🔒"
                    }
                    h3 style="font-size:1.15rem; font-weight:700; color:var(--slate-900); margin-bottom:0.4rem;" { "100% In-Memory Privacy" }
                    p style="font-size:0.88rem; color:var(--slate-500); line-height:1.5;" {
                        "Documents are kept in temporary volatile RAM during processing and deleted permanently right after download. No file retention or cloud backups."
                    }
                }
                div style="background:#ffffff; border:1px solid var(--slate-200); border-radius:var(--radius-lg); padding:1.75rem; box-shadow:var(--shadow-sm);" {
                    div style="width:44px; height:44px; background:#ECFDF5; border-radius:var(--radius-md); display:flex; align-items:center; justify-content:center; color:var(--success); font-size:1.3rem; margin-bottom:1rem;" {
                        "⚡"
                    }
                    h3 style="font-size:1.15rem; font-weight:700; color:var(--slate-900); margin-bottom:0.4rem;" { "Compiled Rust Engine" }
                    p style="font-size:0.88rem; color:var(--slate-500); line-height:1.5;" {
                        "Powered by high-performance Tokio and multi-threaded Ghostscript pipelines for instant processing with live byte-level progress updates."
                    }
                }
                div style="background:#ffffff; border:1px solid var(--slate-200); border-radius:var(--radius-lg); padding:1.75rem; box-shadow:var(--shadow-sm);" {
                    div style="width:44px; height:44px; background:#FEF3C7; border-radius:var(--radius-md); display:flex; align-items:center; justify-content:center; color:#D97706; font-size:1.3rem; margin-bottom:1rem;" {
                        "🎯"
                    }
                    h3 style="font-size:1.15rem; font-weight:700; color:var(--slate-900); margin-bottom:0.4rem;" { "Smart Lossless Clarity" }
                    p style="font-size:0.88rem; color:var(--slate-500); line-height:1.5;" {
                        "Only heavy raster images are downsampled. Text vectors, mathematical symbols, and embedded tables remain 100% sharp and readable."
                    }
                }
            }
        }

        // How It Works (3 Simple Steps)
        section id="how-it-works" style="max-width:1100px; margin:4.5rem auto 0 auto; padding:0 1.25rem;" {
            div style="text-align:center; margin-bottom:2.5rem;" {
                h2 style="font-size:1.85rem; font-weight:800; color:var(--slate-900); margin-bottom:0.5rem;" {
                    "How to Compress a PDF in 3 Easy Steps"
                }
                p style="font-size:0.95rem; color:var(--slate-500);" {
                    "Fast, simple, and effortless on both mobile and desktop."
                }
            }
            div style="display:grid; grid-template-columns:repeat(auto-fit, minmax(260px, 1fr)); gap:1.5rem;" {
                div style="background:#ffffff; border:1px solid var(--slate-200); border-radius:var(--radius-lg); padding:1.75rem; box-shadow:var(--shadow-sm); text-align:center;" {
                    div style="width:44px; height:44px; background:var(--primary-light); color:var(--primary); border-radius:var(--radius-full); display:flex; align-items:center; justify-content:center; font-size:1.2rem; font-weight:800; margin:0 auto 1rem auto;" {
                        "1"
                    }
                    h3 style="font-size:1.15rem; font-weight:700; margin-bottom:0.4rem;" { "Select Your PDF" }
                    p style="font-size:0.88rem; color:var(--slate-500); line-height:1.5;" {
                        "Choose or drag & drop one or multiple PDF documents up to 50 MB each from your phone or computer."
                    }
                }
                div style="background:#ffffff; border:1px solid var(--slate-200); border-radius:var(--radius-lg); padding:1.75rem; box-shadow:var(--shadow-sm); text-align:center;" {
                    div style="width:44px; height:44px; background:var(--primary-light); color:var(--primary); border-radius:var(--radius-full); display:flex; align-items:center; justify-content:center; font-size:1.2rem; font-weight:800; margin:0 auto 1rem auto;" {
                        "2"
                    }
                    h3 style="font-size:1.15rem; font-weight:700; margin-bottom:0.4rem;" { "Choose Compression" }
                    p style="font-size:0.88rem; color:var(--slate-500); line-height:1.5;" {
                        "Pick Recommended for best balance, Extreme for maximum reduction, or specify an exact target size."
                    }
                }
                div style="background:#ffffff; border:1px solid var(--slate-200); border-radius:var(--radius-lg); padding:1.75rem; box-shadow:var(--shadow-sm); text-align:center;" {
                    div style="width:44px; height:44px; background:var(--success-light); color:var(--success); border-radius:var(--radius-full); display:flex; align-items:center; justify-content:center; font-size:1.2rem; font-weight:800; margin:0 auto 1rem auto;" {
                        "3"
                    }
                    h3 style="font-size:1.15rem; font-weight:700; margin-bottom:0.4rem;" { "Download Instantly" }
                    p style="font-size:0.88rem; color:var(--slate-500); line-height:1.5;" {
                        "Download your compressed PDF right beside the tool without scrolling. Files are wiped from memory immediately."
                    }
                }
            }
        }

        // FAQ Section
        section id="faq" style="max-width:820px; margin:4.5rem auto 0 auto; padding:0 1.25rem;" {
            div style="text-align:center; margin-bottom:2.25rem;" {
                h2 style="font-size:1.85rem; font-weight:800; color:var(--slate-900); margin-bottom:0.5rem;" {
                    "Frequently Asked Questions"
                }
                p style="font-size:0.95rem; color:var(--slate-500);" {
                    "Common questions regarding compression quality, limits, and privacy."
                }
            }
            div style="display:flex; flex-direction:column; gap:1rem;" {
                div style="background:#ffffff; border:1px solid var(--slate-200); border-radius:var(--radius-lg); padding:1.35rem;" {
                    h3 style="font-size:1.05rem; font-weight:700; color:var(--slate-900); margin-bottom:0.4rem;" {
                        "Will compressing my PDF degrade text sharpness or ruin the formatting?"
                    }
                    p style="font-size:0.92rem; color:var(--slate-600); line-height:1.6;" {
                        "No. FitKB downsamples heavy embedded raster photos and re-encodes DCT streams while leaving font definitions, vector lines, and forms untouched. Text remains crisp and selectable."
                    }
                }
                div style="background:#ffffff; border:1px solid var(--slate-200); border-radius:var(--radius-lg); padding:1.35rem;" {
                    h3 style="font-size:1.05rem; font-weight:700; color:var(--slate-900); margin-bottom:0.4rem;" {
                        "Is it safe to compress confidential legal, medical, or tax PDFs?"
                    }
                    p style="font-size:0.92rem; color:var(--slate-600); line-height:1.6;" {
                        "Yes. We operate under a strict zero-retention guarantee. Uploaded PDFs exist only in temporary in-memory RAM buffers and are unlinked and destroyed immediately once downloaded."
                    }
                }
                div style="background:#ffffff; border:1px solid var(--slate-200); border-radius:var(--radius-lg); padding:1.35rem;" {
                    h3 style="font-size:1.05rem; font-weight:700; color:var(--slate-900); margin-bottom:0.4rem;" {
                        "Does it work directly on iPhones and Android phones?"
                    }
                    p style="font-size:0.92rem; color:var(--slate-600); line-height:1.6;" {
                        "Yes. FitKB is fully responsive and optimized for mobile touchscreens. You can pick files directly from Files on iOS or your Downloads / Google Drive on Android without downloading an app."
                    }
                }
            }
        }
    };

    Html(base_layout(&seo, content).into_string())
}

async fn compress_100kb_page() -> Html<String> {
    let seo = SeoMetadata {
        title: "Compress PDF to 100KB Online Free — FitKB",
        description: "Reduce PDF file size to under 100 KB online without software installation. Perfect for exam applications, portals, and KYC forms.",
        canonical_url: "https://fitkb.org/compress-pdf-to-100kb",
        h1: "Compress PDF to 100 KB Online",
        preselected_target_kb: Some(100),
    };

    let content = html! {
        section style="padding:3rem 1.5rem 1.5rem 1.5rem; text-align:center; max-width:800px; margin:0 auto;" {
            div style="display:inline-flex; align-items:center; gap:0.4rem; background:var(--primary-light); color:var(--primary); font-size:0.85rem; font-weight:700; padding:0.4rem 1rem; border-radius:var(--radius-full); margin-bottom:1rem;" {
                "📄 Pre-configured for strict 100 KB portals"
            }
            h1 style="font-size:clamp(2.2rem, 5vw, 3.25rem); font-weight:800; color:var(--slate-900); margin-bottom:0.75rem;" {
                (seo.h1)
            }
            p style="font-size:1.1rem; color:var(--slate-600); line-height:1.5;" {
                "Automatically tunes compression passes to fit your document strictly under 100 KB for government exams, job portals, and KYC uploads."
            }
        }
        (render_ilovepdf_box(Some(100)))
    };
    Html(base_layout(&seo, content).into_string())
}

async fn compress_200kb_page() -> Html<String> {
    let seo = SeoMetadata {
        title: "Compress PDF to 200KB Online Free — UPSC & SSC Ready",
        description: "Compress PDF documents under 200 KB for UPSC, SSC, and state job application portals while keeping certificates sharp.",
        canonical_url: "https://fitkb.org/compress-pdf-to-200kb",
        h1: "Compress PDF to 200 KB Online",
        preselected_target_kb: Some(200),
    };

    let content = html! {
        section style="padding:3rem 1.5rem 1.5rem 1.5rem; text-align:center; max-width:800px; margin:0 auto;" {
            div style="display:inline-flex; align-items:center; gap:0.4rem; background:var(--primary-light); color:var(--primary); font-size:0.85rem; font-weight:700; padding:0.4rem 1rem; border-radius:var(--radius-full); margin-bottom:1rem;" {
                "🎓 Optimized for UPSC, SSC & Government Portals"
            }
            h1 style="font-size:clamp(2.2rem, 5vw, 3.25rem); font-weight:800; color:var(--slate-900); margin-bottom:0.75rem;" {
                (seo.h1)
            }
            p style="font-size:1.1rem; color:var(--slate-600); line-height:1.5;" {
                "The standard 200 KB ceiling requirement for UPSC, SSC, state civil services, and college entrance examinations."
            }
        }
        (render_ilovepdf_box(Some(200)))
    };
    Html(base_layout(&seo, content).into_string())
}

async fn compress_1mb_page() -> Html<String> {
    let seo = SeoMetadata {
        title: "Compress PDF to 1MB Online Free — High Quality",
        description: "Compress large multi-page PDF documents to under 1 MB online. Retain crisp illustrations and readable fonts.",
        canonical_url: "https://fitkb.org/compress-pdf-to-1mb",
        h1: "Compress PDF to 1 MB Online",
        preselected_target_kb: Some(1024),
    };

    let content = html! {
        section style="padding:3rem 1.5rem 1.5rem 1.5rem; text-align:center; max-width:800px; margin:0 auto;" {
            div style="display:inline-flex; align-items:center; gap:0.4rem; background:var(--primary-light); color:var(--primary); font-size:0.85rem; font-weight:700; padding:0.4rem 1rem; border-radius:var(--radius-full); margin-bottom:1rem;" {
                "📊 Ideal for Reports, Portfolios & Theses"
            }
            h1 style="font-size:clamp(2.2rem, 5vw, 3.25rem); font-weight:800; color:var(--slate-900); margin-bottom:0.75rem;" {
                (seo.h1)
            }
            p style="font-size:1.1rem; color:var(--slate-600); line-height:1.5;" {
                "Shrink heavy multi-page documents under 1 MB while retaining high typographic sharpness and image definition."
            }
        }
        (render_ilovepdf_box(Some(1024)))
    };
    Html(base_layout(&seo, content).into_string())
}

async fn compress_email_page() -> Html<String> {
    let seo = SeoMetadata {
        title: "Compress PDF for Email Online — Pass Attachment Limits",
        description: "Compress PDF attachments to easily fit within Gmail, Outlook, and corporate email size limits without bounce-backs.",
        canonical_url: "https://fitkb.org/compress-pdf-for-email",
        h1: "Compress PDF for Email Attachments",
        preselected_target_kb: Some(2000),
    };

    let content = html! {
        section style="padding:3rem 1.5rem 1.5rem 1.5rem; text-align:center; max-width:800px; margin:0 auto;" {
            div style="display:inline-flex; align-items:center; gap:0.4rem; background:var(--primary-light); color:var(--primary); font-size:0.85rem; font-weight:700; padding:0.4rem 1rem; border-radius:var(--radius-full); margin-bottom:1rem;" {
                "✉️ Pass Corporate Email Gateways & Limits"
            }
            h1 style="font-size:clamp(2.2rem, 5vw, 3.25rem); font-weight:800; color:var(--slate-900); margin-bottom:0.75rem;" {
                (seo.h1)
            }
            p style="font-size:1.1rem; color:var(--slate-600); line-height:1.5;" {
                "Ensure your invoices, contracts, and proposals safely pass Gmail and Outlook 25 MB email limits without bouncing."
            }
        }
        (render_ilovepdf_box(Some(2000)))
    };
    Html(base_layout(&seo, content).into_string())
}

async fn sitemap_xml() -> impl IntoResponse {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
    <url><loc>https://fitkb.org/</loc><priority>1.0</priority></url>
    <url><loc>https://fitkb.org/compress-pdf-to-100kb</loc><priority>0.9</priority></url>
    <url><loc>https://fitkb.org/compress-pdf-to-200kb</loc><priority>0.9</priority></url>
    <url><loc>https://fitkb.org/compress-pdf-to-1mb</loc><priority>0.8</priority></url>
    <url><loc>https://fitkb.org/compress-pdf-for-email</loc><priority>0.8</priority></url>
</urlset>"#;
    ([(header::CONTENT_TYPE, "application/xml; charset=utf-8")], xml)
}

async fn robots_txt() -> impl IntoResponse {
    let txt = "User-agent: *\nAllow: /\n\nSitemap: https://fitkb.org/sitemap.xml\n";
    ([(header::CONTENT_TYPE, "text/plain; charset=utf-8")], txt)
}

// -------------------------------------------------------------------------------------------------
// API V2 Handlers (Multi-file + ZIP support + Streaming disk upload)
// -------------------------------------------------------------------------------------------------

#[derive(Serialize)]
struct FileResultItem {
    id: String,
    original_name: String,
    original_size: usize,
    compressed_size: usize,
    percent_saved: f64,
    download_url: String,
    message: Option<String>,
}

#[derive(Serialize)]
struct MultiCompressResponse {
    files: Vec<FileResultItem>,
    total_original_bytes: usize,
    total_compressed_bytes: usize,
    total_saved_percent: f64,
    zip_download_url: String,
}

async fn api_compress_v2(mut multipart: Multipart) -> Result<Response, (StatusCode, String)> {
    let temp_dir = get_temp_dir();
    let mut uploaded_files: Vec<(PathBuf, String)> = Vec::new();
    let mut mode_str = "recommended".to_string();
    let mut target_kb: Option<usize> = None;

    while let Ok(Some(mut field)) = multipart.next_field().await {
        let name = field.name().unwrap_or("").to_string();
        if name == "pdf" {
            let file_name = field.file_name().unwrap_or("document.pdf").to_string();
            let upload_id = Uuid::new_v4().to_string();
            let temp_input_path = temp_dir.join(format!("upload_{}.pdf", upload_id));

            // Stream chunks directly to disk/RAM disk to prevent high memory spikes
            let mut file = File::create(&temp_input_path)
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed creating temp file: {}", e)))?;

            while let Ok(Some(chunk)) = field.chunk().await {
                file.write_all(&chunk)
                    .await
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed writing chunk: {}", e)))?;
            }
            uploaded_files.push((temp_input_path, file_name));
        } else if name == "mode" {
            mode_str = field.text().await.unwrap_or_default();
        } else if name == "target_kb" {
            if let Ok(val) = field.text().await {
                target_kb = val.parse::<usize>().ok();
            }
        }
    }

    if uploaded_files.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "No PDF files provided.".to_string()));
    }

    let mode = CompressionMode::parse(&mode_str, target_kb);

    // ── Parallel multi-file compression ──────────────────────────────────────
    // Validate all files first (fast), then compress concurrently.
    // Concurrency is bounded by the GS_SEMAPHORE in engine.rs (2 x CPU cores).
    for (temp_path, _) in &uploaded_files {
        if let Err(e) = validate_pdf_file(temp_path).await {
            let _ = tokio::fs::remove_file(temp_path).await;
            return Err((StatusCode::BAD_REQUEST, e.to_string()));
        }
    }

    let mut join_set: tokio::task::JoinSet<Result<ProcessedPdf, String>> = tokio::task::JoinSet::new();
    for (temp_path, orig_name) in uploaded_files {
        join_set.spawn(async move {
            let res = compress_pdf_file(&temp_path, orig_name, mode).await;
            let _ = tokio::fs::remove_file(&temp_path).await; // delete upload immediately
            res.map_err(|e| format!("Compression error: {}", e))
        });
    }

    let mut processed_results = Vec::new();
    while let Some(task_result) = join_set.join_next().await {
        match task_result {
            Ok(Ok(p)) => processed_results.push(p),
            Ok(Err(msg)) => return Err((StatusCode::INTERNAL_SERVER_ERROR, msg)),
            Err(join_err) => return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Task panicked: {}", join_err))),
        }
    }

    let total_original: usize = processed_results.iter().map(|p| p.original_size).sum();
    let total_compressed: usize = processed_results.iter().map(|p| p.compressed_size).sum();
    let total_saved_pct = if total_original > 0 {
        ((total_original - total_compressed) as f64 / total_original as f64) * 100.0
    } else {
        0.0
    };

    // If multiple files, create a ZIP file bundle
    let zip_id = Uuid::new_v4().to_string();
    let zip_path = temp_dir.join(format!("{}.zip", zip_id));
    if processed_results.len() > 1 {
        let zip_file = std::fs::File::create(&zip_path)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed creating zip: {}", e)))?;
        let mut zip = zip::ZipWriter::new(zip_file);
        let options = zip::write::FileOptions::<()>::default()
            .compression_method(zip::CompressionMethod::Deflated);

        for item in &processed_results {
            let _ = zip.start_file(&item.original_name, options);
            if let Ok(data) = std::fs::read(&item.file_path) {
                use std::io::Write;
                let _ = zip.write_all(&data);
            }
        }
        let _ = zip.finish();
    }

    let file_items: Vec<FileResultItem> = processed_results
        .iter()
        .map(|p| FileResultItem {
            id: p.id.clone(),
            original_name: p.original_name.clone(),
            original_size: p.original_size,
            compressed_size: p.compressed_size,
            percent_saved: (p.percent_saved * 10.0).round() / 10.0,
            download_url: format!("/api/download/{}", p.id),
            message: p.message.clone(),
        })
        .collect();

    let json_resp = MultiCompressResponse {
        files: file_items,
        total_original_bytes: total_original,
        total_compressed_bytes: total_compressed,
        total_saved_percent: (total_saved_pct * 10.0).round() / 10.0,
        zip_download_url: format!("/api/download-zip/{}", zip_id),
    };

    Ok((
        [(header::CONTENT_TYPE, "application/json")],
        serde_json::to_string(&json_resp).unwrap(),
    )
        .into_response())
}

async fn api_download_file(AxumPath(id): AxumPath<String>) -> Result<Response, StatusCode> {
    let temp_dir = get_temp_dir();
    let file_path = temp_dir.join(format!("{}.pdf", id));

    if !file_path.exists() {
        return Err(StatusCode::NOT_FOUND);
    }

    let file_data = tokio::fs::read(&file_path)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Immediate cleanup: Delete file once read for zero-storage guarantee
    let _ = tokio::fs::remove_file(&file_path).await;

    let mut headers = axum::http::HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "application/pdf".parse().unwrap());
    headers.insert(
        header::CONTENT_DISPOSITION,
        "attachment; filename=\"compressed.pdf\"".parse().unwrap(),
    );

    Ok((headers, file_data).into_response())
}

async fn api_download_zip(AxumPath(id): AxumPath<String>) -> Result<Response, StatusCode> {
    let temp_dir = get_temp_dir();
    let zip_path = temp_dir.join(format!("{}.zip", id));

    if !zip_path.exists() {
        return Err(StatusCode::NOT_FOUND);
    }

    let data = tokio::fs::read(&zip_path)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let _ = tokio::fs::remove_file(&zip_path).await;

    let mut headers = axum::http::HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "application/zip".parse().unwrap());
    headers.insert(
        header::CONTENT_DISPOSITION,
        "attachment; filename=\"fitkb_compressed_files.zip\"".parse().unwrap(),
    );

    Ok((headers, data).into_response())
}

/// Periodic background cleaner: sweeps temp dir every 60 seconds to purge any file older than 10 minutes
fn spawn_cleanup_task() {
    tokio::spawn(async {
        let temp_dir = get_temp_dir();
        let max_age = Duration::from_secs(600); // 10 minutes

        loop {
            tokio::time::sleep(Duration::from_secs(60)).await;
            if let Ok(mut entries) = tokio::fs::read_dir(&temp_dir).await {
                while let Ok(Some(entry)) = entries.next_entry().await {
                    if let Ok(metadata) = entry.metadata().await {
                        if let Ok(modified) = metadata.modified() {
                            if let Ok(elapsed) = modified.elapsed() {
                                if elapsed > max_age {
                                    let _ = tokio::fs::remove_file(entry.path()).await;
                                }
                            }
                        }
                    }
                }
            }
        }
    });
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // Clean up temp dir on startup
    let temp_dir = get_temp_dir();
    if let Ok(mut entries) = std::fs::read_dir(&temp_dir) {
        for entry in entries.flatten() {
            let _ = std::fs::remove_file(entry.path());
        }
    }

    // Spawn 10-minute auto-deletion task
    spawn_cleanup_task();

    let app = Router::new()
        // SEO Landing pages
        .route("/", get(home_page))
        .route("/compress-pdf", get(home_page))
        .route("/compress-pdf-to-100kb", get(compress_100kb_page))
        .route("/compress-pdf-to-200kb", get(compress_200kb_page))
        .route("/compress-pdf-to-1mb", get(compress_1mb_page))
        .route("/compress-pdf-for-email", get(compress_email_page))
        // SEO robots & sitemap
        .route("/sitemap.xml", get(sitemap_xml))
        .route("/robots.txt", get(robots_txt))
        // Modern iLovePDF API endpoints
        .route("/api/compress-v2", post(api_compress_v2))
        .route("/api/download/:id", get(api_download_file))
        .route("/api/download-zip/:id", get(api_download_zip))
        .layer(DefaultBodyLimit::max(MAX_UPLOAD_LIMIT))
        .layer(TimeoutLayer::new(Duration::from_secs(150)))
        .layer(CompressionLayer::new())
        // Security headers
        .layer(SetResponseHeaderLayer::overriding(
            header::X_CONTENT_TYPE_OPTIONS,
            axum::http::HeaderValue::from_static("nosniff"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            header::X_FRAME_OPTIONS,
            axum::http::HeaderValue::from_static("DENY"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            header::REFERRER_POLICY,
            axum::http::HeaderValue::from_static("strict-origin-when-cross-origin"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            header::CONTENT_SECURITY_POLICY,
            axum::http::HeaderValue::from_static("default-src 'self' https: data: blob: 'unsafe-inline' 'unsafe-eval';"),
        ))
        .layer(TraceLayer::new_for_http());

    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr: SocketAddr = format!("0.0.0.0:{}", port).parse().unwrap();

    println!("==================================================");
    println!("FitKB iLovePDF-Grade Rust Server on http://{}", addr);
    println!("==================================================");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
