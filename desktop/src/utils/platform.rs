/// Platform-appropriate label for the "Reveal in Finder" / "Show in Explorer" action.
pub fn reveal_label() -> &'static str {
    #[cfg(target_os = "macos")]
    {
        "Reveal in Finder"
    }
    #[cfg(target_os = "windows")]
    {
        "Show in Explorer"
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        "Show in File Manager"
    }
}
