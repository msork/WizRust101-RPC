use std::{
    env,
    fs::File,
    io::{BufReader, BufWriter},
    time::{SystemTime, UNIX_EPOCH},
};

use wizrust101_rpc::mapping::ZoneCatalog;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arguments = env::args_os().skip(1).collect::<Vec<_>>();
    if arguments.len() != 2 {
        return Err("usage: import-bacon-zones <legacy-zones.json> <output-zones.json>".into());
    }

    let input_path = arguments[0].to_string_lossy().into_owned();
    let input = BufReader::new(File::open(&arguments[0])?);
    let imported_at = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    let catalog = ZoneCatalog::import_bacon_json(input, input_path, imported_at)?;
    let output = BufWriter::new(File::create(&arguments[1])?);
    serde_json::to_writer_pretty(output, &catalog)?;
    Ok(())
}
