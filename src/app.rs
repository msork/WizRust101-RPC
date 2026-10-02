use std::{
    error::Error,
    io::{BufReader, Cursor},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
        mpsc::Sender,
    },
    thread,
    time::{Duration, Instant, SystemTime},
};

use crate::{
    config::{
        AppConfig, EnvironmentOverrides, LogLevel, default_config_path, resolve_log_candidates,
    },
    discord::{IpcTransport, PresencePublisher},
    discovery::{LogCandidate, discover_system_with_roots},
    log_tailer::{LogTailer, StartPosition},
    mapping::ZoneCatalog,
    parser::LogParser,
    presence::{Presence, PresenceConfig, WorldAssetCatalog},
    replay::replay_existing_log,
    state::GameState,
    steam_libraries::{default_registry_path, load_roots},
};

const EMBEDDED_APPLICATION_ID: Option<&str> = option_env!("WIZRUST101_RELEASE_DISCORD_APP_ID");
const ZONE_CATALOG_JSON: &[u8] = include_bytes!("../data/zones.json");
const WORLD_ASSETS_JSON: &[u8] = include_bytes!("../data/world-assets.json");

pub fn watch_until_stopped(stop: Arc<AtomicBool>, status: Sender<String>) {
    let config = match default_config_path() {
        Some(path) => {
            let loaded = AppConfig::load(&path);
            for warning in loaded.warnings {
                report(LogLevel::Warn, LogLevel::Warn, &warning);
            }
            loaded.config.resolve(&EnvironmentOverrides::from_process())
        }
        None => {
            report(
                LogLevel::Warn,
                LogLevel::Warn,
                "per-user config directory unavailable; using built-in defaults",
            );
            AppConfig::default().resolve(&EnvironmentOverrides::from_process())
        }
    };
    for warning in &config.warnings {
        report(LogLevel::Warn, config.log_level, warning);
    }

    let catalog = match ZoneCatalog::from_reader(BufReader::new(Cursor::new(ZONE_CATALOG_JSON))) {
        Ok(catalog) => catalog,
        Err(error) => {
            report(
                LogLevel::Error,
                config.log_level,
                &format!("built-in zone catalog is invalid: {error}"),
            );
            ZoneCatalog::default()
        }
    };
    let world_assets =
        match WorldAssetCatalog::from_reader(BufReader::new(Cursor::new(WORLD_ASSETS_JSON))) {
            Ok(assets) => assets,
            Err(error) => {
                report(
                    LogLevel::Warn,
                    config.log_level,
                    &format!(
                        "built-in world asset catalog is invalid; omitting world images: {error}"
                    ),
                );
                WorldAssetCatalog::default()
            }
        };
    let presence_config = PresenceConfig {
        world_asset_keys: world_assets.worlds,
    };
    let app_id = resolve_application_id(config.log_level);
    let mut publisher = app_id.map(|id| PresencePublisher::new(IpcTransport::new(id)));
    let mut current_status = String::new();
    let mut override_warning_reported = false;

    while !stop.load(Ordering::Relaxed) {
        let additional_roots = match default_registry_path() {
            Some(path) => match load_roots(&path) {
                Ok(roots) => roots,
                Err(error) => {
                    report(
                        LogLevel::Warn,
                        config.log_level,
                        &format!("additional Steam library access registry ignored: {error}"),
                    );
                    Vec::new()
                }
            },
            None => Vec::new(),
        };
        let (candidates, override_warning) =
            resolve_log_candidates(config.game_log_path.as_deref(), || {
                discover_system_with_roots(&additional_roots)
            });
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
                    send_status(
                        &status,
                        &mut current_status,
                        "Watching Wizard101 log (Steam)",
                    );
                    let mut state = GameState::default();
                    if let Err(error) = monitor_candidate(
                        candidate,
                        &mut WatchContext {
                            catalog: &catalog,
                            state: &mut state,
                            presence_config: &presence_config,
                            publisher: &mut publisher,
                            log_level: config.log_level,
                            stop: &stop,
                            status: &status,
                            current_status: &mut current_status,
                        },
                    ) {
                        report(
                            LogLevel::Error,
                            config.log_level,
                            &format!("log monitoring stopped; retrying discovery: {error}"),
                        );
                        send_status(&status, &mut current_status, "Log error; retrying");
                    }
                    clear_presence(&mut publisher);
                } else {
                    clear_presence(&mut publisher);
                    send_status(&status, &mut current_status, "Waiting for Steam Wizard101");
                }
            }
            Err(error) => {
                report(
                    LogLevel::Error,
                    config.log_level,
                    &format!("log discovery failed; retrying: {error}"),
                );
                send_status(&status, &mut current_status, "Discovery error; retrying");
            }
        }
        thread::sleep(Duration::from_secs(3));
    }

    clear_presence(&mut publisher);
    send_status(&status, &mut current_status, "Stopped");
}

fn resolve_application_id(log_level: LogLevel) -> Option<String> {
    let runtime = std::env::var("WIZRUST101_DISCORD_APP_ID").ok();
    let selected = select_application_id(runtime.as_deref(), EMBEDDED_APPLICATION_ID);
    if runtime.is_some() && selected != runtime.as_deref() {
        report(
            LogLevel::Warn,
            log_level,
            "invalid WIZRUST101_DISCORD_APP_ID; using the embedded release ID if available",
        );
    }
    match selected {
        Some(id) if valid_application_id(id) => Some(id.to_owned()),
        Some(_) => {
            report(
                LogLevel::Error,
                log_level,
                "embedded Discord Application ID is invalid; Discord IPC disabled",
            );
            None
        }
        None => {
            report(
                LogLevel::Info,
                log_level,
                "no embedded Discord Application ID; set WIZRUST101_DISCORD_APP_ID for development",
            );
            None
        }
    }
}

fn select_application_id<'a>(
    runtime: Option<&'a str>,
    embedded: Option<&'a str>,
) -> Option<&'a str> {
    runtime
        .filter(|id| valid_application_id(id))
        .or_else(|| embedded.filter(|id| valid_application_id(id)))
}

fn valid_application_id(id: &str) -> bool {
    !id.is_empty() && id.bytes().all(|byte| byte.is_ascii_digit())
}

fn clear_presence(publisher: &mut Option<PresencePublisher<IpcTransport>>) {
    if let Some(publisher) = publisher.as_mut() {
        let _ = publisher.tick(None, Instant::now());
    }
}

struct WatchContext<'a> {
    catalog: &'a ZoneCatalog,
    state: &'a mut GameState,
    presence_config: &'a PresenceConfig,
    publisher: &'a mut Option<PresencePublisher<IpcTransport>>,
    log_level: LogLevel,
    stop: &'a AtomicBool,
    status: &'a Sender<String>,
    current_status: &'a mut String,
}

fn monitor_candidate(
    candidate: &LogCandidate,
    context: &mut WatchContext<'_>,
) -> Result<(), Box<dyn Error>> {
    let WatchContext {
        catalog,
        state,
        presence_config,
        publisher,
        log_level,
        stop,
        status,
        current_status,
    } = context;
    report(
        LogLevel::Info,
        *log_level,
        &format!("watching Wizard101 log at {}", candidate.path.display()),
    );
    let mut tailer = LogTailer::open(&candidate.path, StartPosition::End)?;
    let mut parser = LogParser::default();
    let restore_time = Instant::now();
    let replayed_lines = replay_existing_log(
        &candidate.path,
        tailer.offset(),
        &mut parser,
        state,
        catalog,
        restore_time,
    )?;
    report(
        LogLevel::Debug,
        *log_level,
        &format!("restored state from {replayed_lines} existing log records"),
    );
    let mut generation = tailer.generation();
    while !stop.load(Ordering::Relaxed) {
        let lines = match tailer.poll() {
            Ok(lines) => lines,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(()),
            Err(error) => return Err(error.into()),
        };
        if tailer.generation() != generation {
            parser.reset();
            **state = GameState::default();
            generation = tailer.generation();
        }
        for line in lines {
            let observed_at = Instant::now();
            for event in parser.parse_line(&line) {
                state.apply(event, catalog, observed_at);
            }
        }
        let now = Instant::now();
        let desired = Presence::from_game_state(state, presence_config, now, SystemTime::now());
        let ipc_error = publisher
            .as_mut()
            .and_then(|publisher| publisher.tick(desired.as_ref(), now));
        if let Some(error) = ipc_error {
            report(
                LogLevel::Error,
                *log_level,
                &format!("Discord IPC unavailable; retrying: {error}"),
            );
            send_status(status, current_status, "Discord reconnecting");
        } else if desired.is_none() {
            send_status(
                status,
                current_status,
                "Watching Steam (waiting for verified game data)",
            );
        } else if publisher
            .as_ref()
            .is_some_and(PresencePublisher::is_connected)
        {
            send_status(status, current_status, "Watching Wizard101 (Steam)");
        } else if publisher.is_some() {
            send_status(status, current_status, "Discord reconnecting");
        } else {
            send_status(
                status,
                current_status,
                "Watching Steam (Discord ID unavailable)",
            );
        }
        thread::sleep(Duration::from_millis(250));
    }
    clear_presence(publisher);
    Ok(())
}

fn send_status(status: &Sender<String>, current: &mut String, value: &str) {
    if current != value {
        *current = value.to_owned();
        let _ = status.send(current.clone());
    }
}

fn report(message_level: LogLevel, configured_level: LogLevel, message: &str) {
    if configured_level.permits(message_level) {
        eprintln!("[{}] {message}", message_level.as_str());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_development_id_overrides_embedded_release_id() {
        assert_eq!(
            select_application_id(Some("123456"), Some("654321")),
            Some("123456")
        );
    }

    #[test]
    fn invalid_development_id_falls_back_to_valid_embedded_id() {
        assert_eq!(
            select_application_id(Some("not-an-id"), Some("654321")),
            Some("654321")
        );
    }

    #[test]
    fn absent_or_invalid_ids_disable_discord_ipc() {
        assert_eq!(select_application_id(None, None), None);
        assert_eq!(select_application_id(Some("bad"), Some("also-bad")), None);
    }
}
