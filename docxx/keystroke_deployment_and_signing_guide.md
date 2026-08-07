# KeyStroke — Windows Deployment & Code Signing Guide

## 1. Overview

KeyStroke requires low-level keyboard access (`WH_KEYBOARD_LL`) and system-wide text injection (`SendInput`). On Windows, running alongside elevated applications (e.g. Administrator Command Prompts, Task Manager, IDEs running as Administrator) requires bypassing Windows User Interface Privilege Isolation (UIPI).

To achieve UIPI bypass, KeyStroke is configured with:
1. **Application Manifest**: `keystroke.manifest` containing `uiAccess="true"` and `requestedExecutionLevel level="asInvoker"`.
2. **Installation Location**: `tauri.conf.json` configured for `perMachine` installation targeting `C:\Program Files\KeyStroke\`.
3. **Digital Code-Signing**: Binaries must be digitally signed with a trusted code-signing certificate (OV or EV).

---

## 2. Free Open-Source Code-Signing via SignPath Foundation

KeyStroke qualifies for free open-source code-signing provided by the **SignPath Foundation** using an OV certificate.

### Step 1: Apply for Free Open-Source Signing
1. Go to [https://signpath.org/apply.html](https://signpath.org/apply.html).
2. Submit your KeyStroke repository URL: `https://github.com/PriyanshuKr-2027/KeyStroke`.
3. Select open-source project type and wait for approval (typically 1–2 business days).

### Step 2: Configure GitHub Repository Secrets
Once approved in SignPath dashboard, add the following secrets in GitHub Repository $\rightarrow$ Settings $\rightarrow$ Secrets and variables $\rightarrow$ Actions:

| Secret Name | Source in SignPath Dashboard | Purpose |
| :--- | :--- | :--- |
| `SIGNPATH_API_TOKEN` | SignPath Organization $\rightarrow$ API Tokens | Authenticates GitHub Actions runner to SignPath |
| `SIGNPATH_ORGANIZATION_ID` | SignPath Organization Settings | Identifies your open-source organization |
| `SIGNPATH_ENABLED` (Variable) | Set to `true` in Repository Variables | Toggles automated signing step in CI workflow |

---

## 3. GitHub Actions CI/CD Workflow (`.github/workflows/release.yml`)

The repository includes an automated release workflow that triggers whenever a new tag is pushed (e.g., `git tag v1.0.0 && git push origin v1.0.0`):

```yaml
name: Release KeyStroke Desktop App

on:
  push:
    tags:
      - 'v*'
  workflow_dispatch:

jobs:
  release-windows:
    permissions:
      contents: write
      id-token: write  # Required for SignPath OIDC authentication
    runs-on: windows-latest
    steps:
      - name: Checkout repository
        uses: actions/checkout@v4

      - name: Node.js & Rust setup
        uses: actions/setup-node@v4
        with:
          node-version: 20
      - uses: dtolnay/rust-toolchain@stable

      - name: Install frontend dependencies
        run: |
          cd keymind-control-center
          npm install

      - name: Build Windows Application (Tauri)
        uses: tauri-apps/tauri-action@v0
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
        with:
          projectPath: ./keymind-control-center
          tagName: v__VERSION__
          releaseName: 'KeyStroke v__VERSION__'

      - name: Submit to SignPath for code signing
        if: vars.SIGNPATH_ENABLED == 'true' && secrets.SIGNPATH_API_TOKEN != ''
        uses: signpath/github-action-submit-signing-request@v1
        with:
          api-token: '${{ secrets.SIGNPATH_API_TOKEN }}'
          organization-id: '${{ secrets.SIGNPATH_ORGANIZATION_ID }}'
          project-slug: 'keystroke'
          signing-policy-slug: 'release-signing'
          github-artifact-id: 'keystroke-windows'
          wait-for-completion: true
          output-artifact-directory: 'signed-output'
```

---

## 4. User Installation & SmartScreen Guidance

When users download newly released executables, Windows SmartScreen may show a temporary warning: **"Windows protected your PC"**.

### User Installation Notes (`distribution/windows/README-windows.md`):
* **SmartScreen Prompt**: Click **"More info"** $\rightarrow$ **"Run anyway"**. As install volume grows, SmartScreen reputation builds automatically.
* **Default Directory**: Always install to `C:\Program Files\KeyStroke\` (default selected by installer) to ensure full UIPI bypass in elevated windows.
* **Verifying Signature**: Right-click `.exe` $\rightarrow$ Properties $\rightarrow$ Digital Signatures to verify SignPath Foundation certificate.
