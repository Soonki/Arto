use std::path::Path;

#[cfg(any(target_os = "macos", target_os = "windows"))]
use std::process::Command;

/// Reveal a file in Finder (macOS), Explorer (Windows), or file manager
pub fn reveal_in_finder(path: impl AsRef<Path>) {
    let path = path.as_ref();

    #[cfg(target_os = "macos")]
    {
        if let Err(e) = Command::new("open").arg("-R").arg(path).spawn() {
            tracing::error!(%e, ?path, "Failed to reveal in Finder");
        }
    }

    #[cfg(target_os = "windows")]
    {
        // Use explorer.exe /select, to highlight the file in Explorer
        if let Err(e) = Command::new("explorer.exe")
            .arg(format!("/select,{}", path.display()))
            .spawn()
        {
            tracing::error!(%e, ?path, "Failed to reveal in Explorer");
        }
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        if let Some(parent) = path.parent() {
            if let Err(e) = open::that(parent) {
                tracing::error!(%e, ?parent, "Failed to open parent directory");
            }
        }
    }
}

/// Open a directory in Finder (macOS) or file explorer.
pub fn open_directory_in_finder(path: impl AsRef<Path>) {
    let path = path.as_ref();

    #[cfg(target_os = "macos")]
    {
        if let Err(e) = Command::new("open").arg(path).spawn() {
            tracing::error!(%e, ?path, "Failed to open directory in Finder");
        }
    }

    #[cfg(not(target_os = "macos"))]
    {
        if let Err(e) = open::that(path) {
            tracing::error!(%e, ?path, "Failed to open directory");
        }
    }
}
