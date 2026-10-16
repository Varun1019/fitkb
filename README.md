# FitKB — Fast, Private, High-Performance PDF Compressor

A production-grade PDF compression engine and web platform written in **Rust** (Axum + Tokio + Ghostscript -dSAFER backend) with a modern, responsive, mobile-first interface inspired by iLovePDF.

Live demo: **[https://cathedral-anyway-rounds-macro.trycloudflare.com](https://cathedral-anyway-rounds-macro.trycloudflare.com)**

---

## Key Features

- ⚡ **Compiled Rust Core**: High-throughput parallel processing with multi-threaded stream pipelines.
- 🔒 **100% In-Memory Privacy**: Uploads stream to volatile memory (`/dev/shm` RAM disk) and are unlinked and permanently purged immediately upon download. No files or metadata stored.
- 🎯 **Lossless Vector Clarity**: Selectively downsamples heavy raster images and recompresses object streams while leaving typography, vector illustrations, and interactive forms crisp and sharp.
- 📱 **Mobile & Desktop First**: Fully responsive UI with drag-and-drop, clipboard paste support, and native smartphone file picker support.
- 📊 **Realistic Progress Tracking**: Tracks byte-level upload speeds and real-time engine optimization phases.
- 🛡️ **Hardened Sandbox**: Executes with `-dSAFER`, random UUID namespaces, strict semaphore concurrency bounds, and hardened HTTP security headers.

---

## Compression Modes

1. **Recommended (Default)**: Balanced size reduction with crisp 150 DPI resolution for everyday documents, resumes, and portfolios.
2. **Extreme**: Aggressive downsampling (72 DPI) for low-bandwidth scenarios or strict portal limits.
3. **Less**: Subtle optimization (300 DPI) preserving print-grade clarity.
4. **Exact Target Size**: Adaptive binary-search passes that progressively tune DPI and JPEG quality to fit strictly under 100 KB, 200 KB, 500 KB, or 1 MB limits.

---

## Local Development

### Prerequisites
- [Rust](https://www.rust-lang.org/) (1.75+)
- [Ghostscript](https://www.ghostscript.com/) (`gs`)

### Running Locally
```bash
git clone https://github.com/Varun1019/fitkb.git
cd fitkb
cargo run
```
The server will start at `http://localhost:8080`.

---

## Docker & Cloud Deployment

```bash
docker build -t fitkb .
docker run -p 8080:8080 fitkb
```
Deploy to Render, Fly.io, or Railway with 1 click using the included `Dockerfile`.

---

## License

MIT License. Open-source and free for all.
