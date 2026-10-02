use std::{
    error::Error,
    path::PathBuf,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
        mpsc,
    },
    thread,
    time::Duration,
};

use tray_icon::{
    Icon, TrayIcon, TrayIconBuilder,
    menu::{Menu, MenuEvent, MenuItem},
};
use winit::{
    application::ApplicationHandler,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop, EventLoopProxy},
    window::WindowId,
};

use crate::{app::watch_until_stopped, steam_libraries};

const STATUS_ID: &str = "watch-status";
const ADD_LIBRARY_ID: &str = "add-library";
const QUIT_ID: &str = "quit";

enum UserEvent {
    Menu(MenuEvent),
    LibraryChosen(Result<String, String>),
}

struct WindowsTrayApp {
    tray: Option<TrayIcon>,
    status_item: MenuItem,
    status_rx: mpsc::Receiver<String>,
    event_proxy: EventLoopProxy<UserEvent>,
    stop: Arc<AtomicBool>,
    watcher: Option<thread::JoinHandle<()>>,
}

impl WindowsTrayApp {
    fn start(event_proxy: EventLoopProxy<UserEvent>) -> Self {
        let (status_tx, status_rx) = mpsc::channel();
        let stop = Arc::new(AtomicBool::new(false));
        let watcher_stop = Arc::clone(&stop);
        let watcher = thread::spawn(move || watch_until_stopped(watcher_stop, status_tx));
        Self {
            tray: None,
            status_item: MenuItem::with_id(STATUS_ID, "Status: Starting", false, None),
            status_rx,
            event_proxy,
            stop,
            watcher: Some(watcher),
        }
    }

    fn create_tray(&mut self) -> Result<(), Box<dyn Error>> {
        let menu = Menu::new();
        self.status_item = MenuItem::with_id(STATUS_ID, "Status: Starting", false, None);
        let add_library = MenuItem::with_id(ADD_LIBRARY_ID, "Add Steam library…", true, None);
        let quit = MenuItem::with_id(QUIT_ID, "Quit WizRust101-RPC", true, None);
        menu.append(&self.status_item)?;
        menu.append(&add_library)?;
        menu.append(&quit)?;

        let icon = app_icon()?;
        self.tray = Some(
            TrayIconBuilder::new()
                .with_guid(0x7f1c_9df0_3ca2_4c45_98b1_77d0_a33e_1010)
                .with_tooltip("WizRust101-RPC")
                .with_icon(icon)
                .with_menu(Box::new(menu))
                .build()?,
        );

        let proxy = self.event_proxy.clone();
        MenuEvent::set_event_handler(Some(move |event| {
            let _ = proxy.send_event(UserEvent::Menu(event));
        }));
        Ok(())
    }

    fn update_status(&mut self, status: String) {
        let label = format!("Status: {status}");
        self.status_item.set_text(label);
    }

    fn choose_library(&mut self) {
        self.update_status("Choose Steam library folder".into());
        let proxy = self.event_proxy.clone();
        thread::spawn(move || {
            let selected = rfd::FileDialog::new()
                .set_title("Select a Steam library containing Wizard101")
                .pick_folder()
                .ok_or_else(|| "No folder selected".to_owned())
                .and_then(|root| persist_library(root).map(|root| root.display().to_string()));
            let _ = proxy.send_event(UserEvent::LibraryChosen(selected));
        });
    }

    fn stop_watcher(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(watcher) = self.watcher.take() {
            let _ = watcher.join();
        }
        MenuEvent::set_event_handler::<fn(MenuEvent)>(None);
    }
}

impl ApplicationHandler<UserEvent> for WindowsTrayApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.tray.is_none()
            && let Err(error) = self.create_tray()
        {
            self.update_status(format!("Tray error: {error}"));
            event_loop.exit();
        }
    }

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: UserEvent) {
        match event {
            UserEvent::Menu(event) if event.id == ADD_LIBRARY_ID => self.choose_library(),
            UserEvent::Menu(event) if event.id == QUIT_ID => event_loop.exit(),
            UserEvent::Menu(_) => {}
            UserEvent::LibraryChosen(Ok(root)) => {
                self.update_status(format!("Added library {}; discovery will retry", root));
            }
            UserEvent::LibraryChosen(Err(error)) => {
                self.update_status(format!("Steam library not added: {error}"));
            }
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        while let Ok(status) = self.status_rx.try_recv() {
            self.update_status(status);
        }
        event_loop.set_control_flow(ControlFlow::WaitUntil(
            std::time::Instant::now() + Duration::from_millis(300),
        ));
    }

    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        self.stop_watcher();
        self.tray.take();
    }

    fn window_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        _event: winit::event::WindowEvent,
    ) {
    }
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let event_loop = EventLoop::<UserEvent>::with_user_event().build()?;
    let mut app = WindowsTrayApp::start(event_loop.create_proxy());
    event_loop.run_app(&mut app)?;
    Ok(())
}

fn persist_library(root: PathBuf) -> Result<PathBuf, String> {
    let registry = steam_libraries::default_registry_path()
        .ok_or_else(|| "the application data directory is unavailable".to_owned())?;
    steam_libraries::add_root(&registry, &root).map_err(|error| error.to_string())?;
    Ok(root)
}

fn app_icon() -> Result<Icon, Box<dyn Error>> {
    let decoder = png::Decoder::new(std::io::Cursor::new(include_bytes!(
        "../../assets/icons/sizes/32.png"
    )));
    let mut reader = decoder.read_info()?;
    let size = reader
        .output_buffer_size()
        .ok_or("embedded tray icon has an invalid decoded size")?;
    let mut buffer = vec![0; size];
    let info = reader.next_frame(&mut buffer)?;
    if info.color_type != png::ColorType::Rgba || info.bit_depth != png::BitDepth::Eight {
        return Err("embedded tray PNG must use 8-bit RGBA pixels".into());
    }
    Ok(Icon::from_rgba(
        buffer[..info.buffer_size()].to_vec(),
        info.width,
        info.height,
    )?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedded_tray_icon_decodes_to_square_rgba_image() {
        app_icon().expect("decode embedded tray icon");
    }
}
