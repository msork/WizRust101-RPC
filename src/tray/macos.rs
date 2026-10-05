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
    platform::macos::{ActivationPolicy, EventLoopBuilderExtMacOS},
    window::WindowId,
};

use crate::{app::watch_until_stopped, crossover_bottles};

const STATUS_ID: &str = "watch-status";
const ADD_BOTTLE_ID: &str = "add-bottle";
const QUIT_ID: &str = "quit";

enum UserEvent {
    Menu(MenuEvent),
}

struct MacTrayApp {
    tray: Option<TrayIcon>,
    status_item: MenuItem,
    status_rx: mpsc::Receiver<String>,
    event_proxy: EventLoopProxy<UserEvent>,
    stop: Arc<AtomicBool>,
    watcher: Option<thread::JoinHandle<()>>,
}

impl MacTrayApp {
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
        let add_bottle = MenuItem::with_id(ADD_BOTTLE_ID, "Add CrossOver bottle…", true, None);
        let quit = MenuItem::with_id(QUIT_ID, "Quit WizRust101-RPC", true, None);
        menu.append(&self.status_item)?;
        menu.append(&add_bottle)?;
        menu.append(&quit)?;

        self.tray = Some(
            TrayIconBuilder::new()
                .with_id("wizrust101-rpc")
                .with_tooltip("WizRust101-RPC")
                .with_icon(app_icon()?)
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
        self.status_item.set_text(format!("Status: {status}"));
    }

    fn choose_bottle(&mut self) {
        self.update_status("Select CrossOver bottle folder".into());
        let result = rfd::FileDialog::new()
            .set_title("Select a CrossOver bottle containing Steam Wizard101")
            .pick_folder()
            .ok_or_else(|| "No folder selected".to_owned())
            .and_then(|root| persist_bottle(root).map(|root| root.display().to_string()));
        match result {
            Ok(_) => self.update_status("CrossOver bottle added".into()),
            Err(error) => {
                eprintln!("[warn] CrossOver bottle not added: {error}");
                self.update_status("CrossOver bottle not added".into());
            }
        }
    }

    fn stop_watcher(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(watcher) = self.watcher.take() {
            let _ = watcher.join();
        }
        MenuEvent::set_event_handler::<fn(MenuEvent)>(None);
    }
}

impl ApplicationHandler<UserEvent> for MacTrayApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.tray.is_none()
            && let Err(error) = self.create_tray()
        {
            self.update_status(format!("Menu bar error: {error}"));
            event_loop.exit();
        }
    }

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: UserEvent) {
        match event {
            UserEvent::Menu(event) if event.id == ADD_BOTTLE_ID => self.choose_bottle(),
            UserEvent::Menu(event) if event.id == QUIT_ID => event_loop.exit(),
            UserEvent::Menu(_) => {}
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
    let mut builder = EventLoop::<UserEvent>::with_user_event();
    builder.with_activation_policy(ActivationPolicy::Accessory);
    let event_loop = builder.build()?;
    let mut app = MacTrayApp::start(event_loop.create_proxy());
    event_loop.run_app(&mut app)?;
    Ok(())
}

fn persist_bottle(root: PathBuf) -> Result<PathBuf, String> {
    let registry = crossover_bottles::default_registry_path()
        .ok_or_else(|| "the application data directory is unavailable".to_owned())?;
    crossover_bottles::add_root(&registry, &root).map_err(|error| error.to_string())?;
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
    fn embedded_menu_bar_icon_is_valid_rgba() {
        app_icon().expect("decode embedded menu bar icon");
    }
}
