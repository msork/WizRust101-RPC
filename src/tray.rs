#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "linux")]
pub use linux::{TrayAction, run};

#[cfg(target_os = "windows")]
pub use windows::run;

#[cfg(not(any(target_os = "linux", target_os = "windows")))]
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    Err("the system tray frontend is not implemented for this platform".into())
}
