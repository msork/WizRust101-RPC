#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

#[cfg(target_os = "macos")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::args_os().any(|argument| argument == "--ci-load-check") {
        let expected = std::env::var("WIZRUST101_CI_EXPECTED_APP_ID")?;
        if !wizrust101_rpc::app::embedded_application_id_matches(&expected) {
            return Err("embedded Discord Application ID does not match CI secret".into());
        }
        return Ok(());
    }
    wizrust101_rpc::tray::run()
}

#[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
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
