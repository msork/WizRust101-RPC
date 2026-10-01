use std::{error::Error, fs::File, io::BufReader, path::Path, thread, time::Duration};

use wizrust101_rpc::{
    discovery::{LogCandidate, discover_system},
    log_tailer::{LogTailer, StartPosition},
    mapping::ZoneCatalog,
    parser::LogParser,
    state::GameState,
};

fn main() -> Result<(), Box<dyn Error>> {
    let catalog_path = Path::new("data/zones.json");
    let catalog = load_catalog(catalog_path)?;
    let mut state = GameState::default();

    loop {
        let candidates = discover_system()?;
        if let Some(candidate) = candidates.first() {
            if let Err(error) = monitor_candidate(candidate, &catalog, &mut state) {
                eprintln!("log monitoring stopped; retrying discovery: {error}");
            }
            state = GameState::default();
        }
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

fn monitor_candidate(
    candidate: &LogCandidate,
    catalog: &ZoneCatalog,
    state: &mut GameState,
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
            generation = tailer.generation();
        }
        for line in lines {
            let observed_at = std::time::Instant::now();
            for event in parser.parse_line(&line) {
                state.apply(event, catalog, observed_at);
                println!("{state:?}");
            }
        }
        thread::sleep(Duration::from_millis(250));
    }
}
