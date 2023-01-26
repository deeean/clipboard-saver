#[cfg(target_os = "macos")]
pub mod macos;

pub fn save_clipboard_tiff() -> Option<String> {
    #[cfg(target_os = "macos")]
    return macos::save_clipboard_tiff();

    #[cfg(not(target_os = "macos"))]
    panic!("Unsupported platform")
}