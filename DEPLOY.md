# FitKB Rust Production Deployment Guide

## 1. Deploy on Fly.io
```bash
# 1. Install Fly CLI
curl -L https://fly.io/install.sh | sh

# 2. Launch from fitkb-rust directory
cd /path/to/fitkb-rust
fly launch --name fitkb-rust --region sin

# 3. Deploy
fly deploy
```

`fly.toml` configuration:
```toml
app = "fitkb-rust"
primary_region = "sin"

[http_service]
  internal_port = 8080
  force_https = true
  auto_stop_machines = true
  auto_start_machines = true
  min_machines_running = 1

[[vm]]
  memory = "512mb"
  cpu_kind = "shared"
  cpus = 1
```

---

## 2. Deploy on Render
1. Create a new **Web Service** on [render.com](https://render.com).
2. Connect your Git repository.
3. Select **Docker** as the Environment.
4. Set Port to `8080`.
5. Deploy.
