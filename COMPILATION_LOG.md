# COMPILATION_LOG - ZAPIA

## Fixes Applied
1. **Initialization Fix**: Replaced the basic entry point with the cm2labs Instance Manager logic to handle multiple accounts/profiles.
2. **Vault (Bóveda) Requirement**:
   - The app now creates a `storage/` directory relative to the executable
   - Each instance/profile created in the UI gets its own isolated folder in `storage/instances/`.
   - `user_data_dir` is correctly set in the backend to ensure portability of cookies and local data.
3. **UI Update**: Overwrote `ui/index.html` with the Instance Launcher UI (v1.1.0) which correctly calls the `launch_instance` command.
4. **Tauri Config Update**:
   - Set `csp: null`.
   - Set `windows[0].url: "index.html"`.
5. **NSIS Nesting Fix**: Configured NSIS for `currentUser` install mode and portable-friendly behavior.

## Context7 Documentation Sync
- **Date**: 2026-06-28
- **Official Source**: Tauri v1.5 API Docs (tauri.app/v1) and docs.rs (tauri 1.5.0).
- **Patterns Identified**:
  - `WindowBuilder` and `WindowUrl` are the correct v1.x types (not `WebviewWindowBuilder`).
  - `.data_directory(path)` is the method for webview isolation in v1.x (not `.user_data_dir`).
  - `tauri.conf.json` uses `"tauri": { "windows": [...] }` (not `"app"`).
  - Strict CSP implemented: `default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'; img-src 'self' data: https:; connect-src 'self' https: tauri: ipc:;`
- **Actions**:
  - Refactored `main.rs` to use v1.5 stable syntax.
  - Hardened `tauri.conf.json` with explicit `allowlist` (removed `all: true`) and CSP.
  - Verified `protocol.asset` scope for maximum fidelity in local file serving.

## Result
- **Initialization**: Fixed. The app now starts as a profile manager for Zapia.
- **Vault**: Fully operational. Data is stored in `storage/`.
- **URL Verification**: Points to `https://app.zapia.com` for each instance.
