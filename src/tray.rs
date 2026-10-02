#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "linux")]
pub use linux::{TrayAction, run};

#[cfg(not(target_os = "linux"))]
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    Err("the system tray frontend is implemented for Linux in M6".into())
}
