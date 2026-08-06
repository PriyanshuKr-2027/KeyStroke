fn main() {
    // Embed the Windows application manifest on Windows targets.
    // This enables uiAccess=true (UIPI bypass) when the binary is:
    //   1. Code-signed with a trusted certificate
    //   2. Installed in a trusted location (e.g. C:\Program Files\KeyStroke)
    #[cfg(target_os = "windows")]
    {
        embed_resource::compile("keystroke.rc", embed_resource::NONE);
    }

    tauri_build::build()
}
