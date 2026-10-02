use std::{
    error::Error,
    fs::File,
    io::BufReader,
    path::Path,
    thread,
    time::{Duration, Instant, SystemTime},
};

use wizrust101_rpc::{
    config::{
        AppConfig, EnvironmentOverrides, LogLevel, default_config_path, resolve_log_candidates,
    },
    discord::{IpcTransport, PresencePublisher},
    discovery::{LogCandidate, discover_system},
    log_tailer::{LogTailer, StartPosition},
    mapping::ZoneCatalog,
    parser::LogParser,
    presence::{Presence, PresenceConfig, WorldAssetCatalog},
    state::GameState,
};

fn main() -> Result<(), Box<dyn Error>> {
    let catalog_path = Path::new("data/zones.json");
    let catalog = load_catalog(catalog_path)?;
    let mut state = GameState::default();
    let config = match default_config_path() {
        Some(path) => {
            let loaded = AppConfig::load(&path);
            for warning in loaded.warnings {
                eprintln!("[warn] {warning}");
            }
            loaded.config.resolve(&EnvironmentOverrides::from_process())
        }
        None => {
            eprintln!("[warn] per-user config directory unavailable; using built-in defaults");
            AppConfig::default().resolve(&EnvironmentOverrides::from_process())
        }
    };
    for warning in &config.warnings {
        eprintln!("[warn] {warning}");
    }
    let world_assets =
        load_world_assets(Path::new("data/world-assets.json")).unwrap_or_else(|error| {
            report(
                LogLevel::Warn,
                config.log_level,
                &format!("world asset catalog unavailable; omitting world images: {error}"),
            );
            WorldAssetCatalog::default()
        });
    let presence_config = PresenceConfig {
        display_stat: config.display_stat,
        world_asset_keys: world_assets.worlds,
    };
    let mut publisher = match std::env::var("WIZRUST101_DISCORD_APP_ID") {
        Ok(id) if !id.is_empty() && id.bytes().all(|byte| byte.is_ascii_digit()) => {
            Some(PresencePublisher::new(IpcTransport::new(id)))
        }
        Ok(_) => {
            report(
                LogLevel::Warn,
                config.log_level,
                "invalid WIZRUST101_DISCORD_APP_ID; Discord IPC disabled",
            );
            None
        }
        Err(_) => {
            report(
                LogLevel::Info,
                config.log_level,
                "WIZRUST101_DISCORD_APP_ID is unset; Discord IPC disabled",
            );
            None
        }
    };

    let mut override_warning_reported = false;
    loop {
        let (candidates, override_warning) =
            resolve_log_candidates(config.game_log_path.as_deref(), discover_system);
        if let Some(warning) = override_warning {
            if !override_warning_reported {
                report(LogLevel::Warn, config.log_level, &warning);
                override_warning_reported = true;
            }
        } else {
            override_warning_reported = false;
        }

        match candidates {
            Ok(candidates) => {
                if let Some(candidate) = candidates.first() {
                    if let Err(error) = monitor_candidate(
                        candidate,
                        &catalog,
                        &mut state,
                        &presence_config,
                        &mut publisher,
                        config.log_level,
                    ) {
                        report(
                            LogLevel::Error,
                            config.log_level,
                            &format!("log monitoring stopped; retrying discovery: {error}"),
                        );
                    }
                }
            }
            Err(error) => report(
                LogLevel::Error,
                config.log_level,
                &format!("log discovery failed; retrying: {error}"),
            ),
        }
        state = GameState::default();
        update_presence(&state, &presence_config, &mut publisher, config.log_level);
        thread::sleep(Duration::from_secs(3));
    }
}

fn load_catalog(path: &Path) -> Result<ZoneCatalog, Box<dyn Error>> {
    match File::open(path) {
        Ok(file) => Ok(ZoneCatalog::from_reader(BufReader::new(file))?),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(ZoneCatalog::default()),
        Err(error) => Err(error.into()),
    }
}

fn load_world_assets(path: &Path) -> Result<WorldAssetCatalog, Box<dyn Error>> {
    match File::open(path) {
        Ok(file) => Ok(WorldAssetCatalog::from_reader(BufReader::new(file))?),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            Ok(WorldAssetCatalog::default())
        }
        Err(error) => Err(error.into()),
    }
}

fn monitor_candidate(
    candidate: &LogCandidate,
    catalog: &ZoneCatalog,
    state: &mut GameState,
    presence_config: &PresenceConfig,
    publisher: &mut Option<PresencePublisher<IpcTransport>>,
    log_level: LogLevel,
) -> Result<(), Box<dyn Error>> {
    report(
        LogLevel::Info,
        log_level,
        &format!("watching Wizard101 log at {}", candidate.path.display()),
    );
    let mut tailer = LogTailer::open(&candidate.path, StartPosition::End)?;
    let mut parser = LogParser::default();
    let mut generation = tailer.generation();
    loop {
        let lines = match tailer.poll() {
            Ok(lines) => lines,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(()),
            Err(error) => return Err(error.into()),
        };
        if tailer.generation() != generation {
            parser.reset();
            *state = GameState::default();
            generation = tailer.generation();
        }
        for line in lines {
            let observed_at = std::time::Instant::now();
            for event in parser.parse_line(&line) {
                state.apply(event, catalog, observed_at);
            }
        }
        update_presence(state, presence_config, publisher, log_level);
        thread::sleep(Duration::from_millis(250));
    }
}

fn update_presence(
    state: &GameState,
    config: &PresenceConfig,
    publisher: &mut Option<PresencePublisher<IpcTransport>>,
    log_level: LogLevel,
) {
    let now = Instant::now();
    let desired = Presence::from_game_state(state, config, now, SystemTime::now());
    if let Some(publisher) = publisher {
        if let Some(error) = publisher.tick(desired.as_ref(), now) {
            report(
                LogLevel::Error,
                log_level,
                &format!("Discord IPC unavailable; retrying: {error}"),
            );
        }
    }
}

fn report(message_level: LogLevel, configured_level: LogLevel, message: &str) {
    if configured_level.permits(message_level) {
        eprintln!("[{}] {message}", message_level.as_str());
    }
}
