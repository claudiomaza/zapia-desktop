# COMPILATION_LOG - ZAPIA

## Fixes Applied
1. **Initialization Fix**: Replaced the basic entry point with the cm2labs Instance Manager logic to handle multiple accounts/profiles.
2. **Vault (Bóveda) Requirement**:
   - The app now creates a `storage/` directory relative to the executable.
   - Each instance/profile created in the UI gets its own isolated folder in `storage/instances/`.
   - `user_data_dir` is correctly set in the backend to ensure portability of cookies and local data.
3. **UI Update**: Overwrote `ui/index.html` with the Instance Launcher UI (v1.1.0) which correctly calls the `launch_instance` command.
4. **Tauri Config Update**:
   - Set `csp: null`.
   - Set `windows[0].url: "index.html"`.
5. **NSIS Nesting Fix**: Configured NSIS for `currentUser` install mode and portable-friendly behavior.

## Deployment Protocol v1.0 (2026-06-27)
- **Workflow Standardization**: Renamed to `cm2labs-zapia-deployment.yml`.
- **Standardized Build Logic**: Using `tauri-apps/tauri-action@v0` for stability.
- **Artifact Management**: Explicitly generating and uploading:
  1. `standalone-executable` (Portable EXE)
  2. `installer-setup` (NSIS Installer)
- **Branding**: `productName` standardized to `zapia-by-cm2labs`.

## Result
- **Initialization**: Fully operational.
- **Vault**: Data stored in `storage/`.
- **URL Verification**: Points to `https://app.zapia.com`.
