use std::{
    error::Error,
    fs::File,
    io::BufReader,
    path::Path,
    thread,
    time::{Duration, Instant, SystemTime},
};

use wizrust101_rpc::{
    discord::{IpcTransport, PresencePublisher},
    discovery::{LogCandidate, discover_system},
    log_tailer::{LogTailer, StartPosition},
    mapping::ZoneCatalog,
    parser::LogParser,
    presence::{DisplayStat, Presence, PresenceConfig, WorldAssetCatalog},
    state::GameState,
};

fn main() -> Result<(), Box<dyn Error>> {
    let catalog_path = Path::new("data/zones.json");
    let catalog = load_catalog(catalog_path)?;
    let mut state = GameState::default();
    let display_stat = match std::env::var("WIZRUST101_DISPLAY_STAT") {
        Ok(value) => value.parse::<DisplayStat>().unwrap_or_else(|error| {
            eprintln!("invalid WIZRUST101_DISPLAY_STAT ({error}); using health");
            DisplayStat::Health
        }),
        Err(_) => DisplayStat::Health,
    };
    let world_assets =
        load_world_assets(Path::new("data/world-assets.json")).unwrap_or_else(|error| {
            eprintln!("world asset catalog unavailable; omitting world images: {error}");
            WorldAssetCatalog::default()
        });
    let presence_config = PresenceConfig {
        display_stat,
        world_asset_keys: world_assets.worlds,
    };
    let mut publisher = match std::env::var("WIZRUST101_DISCORD_APP_ID") {
        Ok(id) if !id.is_empty() && id.bytes().all(|byte| byte.is_ascii_digit()) => {
            Some(PresencePublisher::new(IpcTransport::new(id)))
        }
        Ok(_) => {
            eprintln!("invalid WIZRUST101_DISCORD_APP_ID; Discord IPC disabled");
            None
        }
        Err(_) => {
            eprintln!("WIZRUST101_DISCORD_APP_ID is unset; Discord IPC disabled");
            None
        }
    };

    loop {
        match discover_system() {
            Ok(candidates) => {
                if let Some(candidate) = candidates.first() {
                    if let Err(error) = monitor_candidate(
                        candidate,
                        &catalog,
                        &mut state,
                        &presence_config,
                        &mut publisher,
                    ) {
                        eprintln!("log monitoring stopped; retrying discovery: {error}");
                    }
                }
            }
            Err(error) => eprintln!("log discovery failed; retrying: {error}"),
        }
        state = GameState::default();
        update_presence(&state, &presence_config, &mut publisher);
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
) -> Result<(), Box<dyn Error>> {
    eprintln!("Watching Wizard101 log at {}", candidate.path.display());
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
        update_presence(state, presence_config, publisher);
        thread::sleep(Duration::from_millis(250));
    }
}

fn update_presence(
    state: &GameState,
    config: &PresenceConfig,
    publisher: &mut Option<PresencePublisher<IpcTransport>>,
) {
    let now = Instant::now();
    let desired = Presence::from_game_state(state, config, now, SystemTime::now());
    if let Some(publisher) = publisher {
        if let Some(error) = publisher.tick(desired.as_ref(), now) {
            eprintln!("Discord IPC unavailable; retrying: {error}");
        }
    }
}
