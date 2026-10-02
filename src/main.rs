#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

#[cfg(not(target_os = "windows"))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    wizrust101_rpc::tray::run()
}

#[cfg(target_os = "windows")]
fn main() {
    if std::env::args_os().any(|argument| argument == "--ci-load-check") {
        return;
    }

    if let Err(error) = wizrust101_rpc::tray::run() {
        rfd::MessageDialog::new()
            .set_title("WizRust101-RPC")
            .set_description(format!("The tray application could not start: {error}"))
            .show();
    }
}
