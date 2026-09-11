---
name: Bug Report
about: Report a bug to help us improve Evoury
title: '[BUG] '
labels: bug
assignees: ''
---

**Describe the bug**
A clear and concise description of what the bug is.

**To reproduce**
Steps to reproduce the behavior:
1. Go to '...'
2. Click on '...'
3. Scroll down to '...'
4. See error

**Expected behavior**
A clear and concise description of what you expected to happen.

**Screenshots**
If applicable, add screenshots to help explain your problem.

**Environment:**
- OS: [e.g., Windows 11, macOS 14, Ubuntu 22.04]
- Rust version: [e.g., 1.75.0]
- Node version: [e.g., 18.19.0]
- Evoury version: [e.g., 0.1.0]

**Additional context**
Add any other context about the problem here.

**Logs**
If applicable, please attach logs from:
- `~/.config/evoury/logs/` (Linux/macOS)
- `%APPDATA%\Evoury\logs\` (Windows)

You can also run Evoury with debug logging:
```bash
RUST_LOG=debug pnpm tauri dev
```
