use std::{
    error::Error,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
        mpsc::{self, Sender, TryRecvError},
    },
    thread,
    time::Duration,
};

use ashpd::desktop::file_chooser::SelectedFiles;
use ksni::{Tray, blocking::TrayMethods, menu::StandardItem};

use crate::{app::watch_until_stopped, steam_libraries};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TrayAction {
    AddSteamLibrary,
    Quit,
}

struct LinuxTray {
    status: String,
    actions: Sender<TrayAction>,
}

impl Tray for LinuxTray {
    fn id(&self) -> String {
        "wizrust101-rpc".into()
    }

    fn title(&self) -> String {
        "WizRust101-RPC".into()
    }

    fn icon_name(&self) -> String {
        "applications-games".into()
    }

    fn icon_pixmap(&self) -> Vec<ksni::Icon> {
        vec![app_icon()]
    }

    fn menu(&self) -> Vec<ksni::MenuItem<Self>> {
        let status = StandardItem {
            label: format!("Status: {}", self.status),
            enabled: false,
            ..Default::default()
        };
        let add_sender = self.actions.clone();
        let add_library = StandardItem {
            label: "Add Steam library…".into(),
            icon_name: "folder-open".into(),
            activate: Box::new(move |_| {
                let _ = add_sender.send(TrayAction::AddSteamLibrary);
            }),
            ..Default::default()
        };
        let quit_sender = self.actions.clone();
        let quit = StandardItem {
            label: "Quit WizRust101-RPC".into(),
            icon_name: "application-exit".into(),
            activate: Box::new(move |_| {
                let _ = quit_sender.send(TrayAction::Quit);
            }),
            ..Default::default()
        };
        vec![status.into(), add_library.into(), quit.into()]
    }
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let (action_tx, action_rx) = mpsc::channel();
    let (status_tx, status_rx) = mpsc::channel();
    let tray = LinuxTray {
        status: "Starting".into(),
        actions: action_tx,
    };
    let tray = tray
        .disable_dbus_name(std::env::var_os("FLATPAK_ID").is_some())
        .assume_sni_available(true)
        .spawn()?;

    let stop = Arc::new(AtomicBool::new(false));
    let watcher_stop = Arc::clone(&stop);
    let watcher_status = status_tx.clone();
    let watcher = thread::spawn(move || watch_until_stopped(watcher_stop, watcher_status));
    let mut status = "Starting".to_owned();
    let mut quitting = false;

    while !quitting {
        if let Ok(next_status) = status_rx.recv_timeout(Duration::from_millis(250))
            && status != next_status
        {
            status.clone_from(&next_status);
            let status_for_tray = status.clone();
            tray.update(|ui| ui.status = status_for_tray);
        }
        loop {
            match action_rx.try_recv() {
                Ok(TrayAction::Quit) => quitting = true,
                Ok(TrayAction::AddSteamLibrary) => {
                    if status != "Choose Steam library folder" {
                        status = "Choose Steam library folder".into();
                        let status_for_tray = status.clone();
                        tray.update(|ui| ui.status = status_for_tray);
                    }
                    let status_tx = status_tx.clone();
                    thread::spawn(move || {
                        let result = choose_steam_library();
                        let message = match result {
                            Ok(()) => "Steam library added; discovery will retry".to_owned(),
                            Err(error) => format!("Steam library not added: {error}"),
                        };
                        let _ = status_tx.send(message);
                    });
                }
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => {
                    quitting = true;
                    break;
                }
            }
        }
    }

    stop.store(true, Ordering::Relaxed);
    let _ = watcher.join();
    tray.shutdown().wait();
    Ok(())
}

fn choose_steam_library() -> Result<(), Box<dyn Error + Send + Sync>> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;
    let selected = runtime.block_on(async {
        SelectedFiles::open_file()
            .title("Select a Steam library folder containing Wizard101")
            .accept_label("Select library")
            .directory(true)
            .send()
            .await?
            .response()
    })?;
    let uri = selected
        .uris()
        .first()
        .ok_or_else(|| std::io::Error::other("no Steam library folder was selected"))?;
    let folder = uri
        .to_file_path()
        .map_err(|_| std::io::Error::other("portal returned a non-file URI"))?;
    let registry = steam_libraries::default_registry_path()
        .ok_or_else(|| std::io::Error::other("application data directory is unavailable"))?;
    steam_libraries::add_root(&registry, &folder)?;
    Ok(())
}

fn app_icon() -> ksni::Icon {
    const SIZE: usize = 32;
    let mut data = vec![0; SIZE * SIZE * 4];
    for y in 0..SIZE {
        for x in 0..SIZE {
            let dx = x as i32 - 16;
            let dy = y as i32 - 16;
            let inside = dx * dx + dy * dy <= 15 * 15;
            let (a, r, g, b) = if inside {
                if (y > 8 && y < 23 && (x as i32 - 16).abs() <= (22 - y) as i32 / 2)
                    || ((20..=24).contains(&y) && (12..=20).contains(&x))
                {
                    (255, 72, 203, 255)
                } else {
                    (255, 44, 30, 98)
                }
            } else {
                (0, 0, 0, 0)
            };
            let index = (y * SIZE + x) * 4;
            data[index..index + 4].copy_from_slice(&[a, r, g, b]);
        }
    }
    // A small golden star gives the icon contrast at tray scale.
    for (x, y) in [(23, 7), (23, 8), (22, 9), (23, 9), (24, 9), (23, 10)] {
        let index = (y * SIZE + x) * 4;
        data[index..index + 4].copy_from_slice(&[255, 255, 220, 92]);
    }
    ksni::Icon {
        width: SIZE as i32,
        height: SIZE as i32,
        data,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tray_icon_has_valid_argb_dimensions_and_visible_pixels() {
        let icon = app_icon();
        assert_eq!((icon.width, icon.height), (32, 32));
        assert_eq!(icon.data.len(), 32 * 32 * 4);
        assert!(
            icon.data
                .as_chunks::<4>()
                .0
                .iter()
                .any(|pixel| pixel[0] > 0)
        );
    }

    #[test]
    fn menu_status_and_actions_are_stable() {
        let (sender, receiver) = mpsc::channel();
        let mut tray = LinuxTray {
            status: "Watching Wizard101 (Steam)".into(),
            actions: sender,
        };
        let menu = tray.menu();
        assert_eq!(menu.len(), 3);
        tray.status = "Discord reconnecting".into();
        let updated = tray.menu();
        assert_eq!(updated.len(), 3);
        drop(receiver);
    }
}
