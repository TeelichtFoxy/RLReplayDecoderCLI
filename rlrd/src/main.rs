use anyhow::{Context, Result};
use clap::Parser;
use std::fs;
use std::path::PathBuf;
use std::time::SystemTime;
use std::ffi::OsStr;
use anyhow::{anyhow, bail};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// The .replay file path to decode
    #[clap(short, long, value_parser, conflicts_with = "get_latest", required_unless_present = "get_latest")]
    replay_file: Option<PathBuf>,

    /// The output path for the decoded .json
    #[clap(short, long, value_parser, requires = "replay_file")]
    output: Option<PathBuf>,

    /// Gives you the path for the latest .replay file
    #[clap(short, long, action = clap::ArgAction::SetTrue)]
    get_latest: bool,
}

fn get_latest_replay() -> Result<PathBuf> {
    let documents_dir = dirs::document_dir()
        .ok_or_else(|| anyhow!("Could not find users documents directory"))?;

    let replay_dir = documents_dir
        .join("My Games")
        .join("Rocket League")
        .join("TAGame")
        .join("Demos");

    if !replay_dir.exists() {
        bail!("Rocket League replay directory does not found at: {:?}", replay_dir);
    }

    let mut latest_file: Option<PathBuf> = None;
    let mut latest_mod_time: Option<SystemTime> = None;

    for entry_result in fs::read_dir(&replay_dir).with_context(|| { format!("Failed to read replay directory: {:?}", replay_dir) })? {
        let entry = entry_result.with_context(|| { format!("Error reading entry in directory: {:?}", replay_dir) })?;

        let path = entry.path();

        if path.is_file() && path.extension() == Some(OsStr::new("replay")) {
            let metadata = fs::metadata(&path).with_context(|| format!("Failed reading metadata for file: {:?}", path))?;

            let modified_time = metadata.modified().with_context(|| format!("Failed getting modification time for: {:?}", path))?;

            if latest_mod_time.is_none() || modified_time > latest_mod_time.unwrap() {
                latest_mod_time = Some(modified_time);
                latest_file = Some(path);
            }
        }
    }

    latest_file.ok_or_else(|| {
        anyhow!("No .replay file found in directory: {:?}", replay_dir)
    })
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.get_latest {
        let latest_replay = get_latest_replay()?;
        println!("Latest .replay file: {:?}", latest_replay.display());
        return Ok(());
    }

    println!("Reading .replay file: {:?}", cli.replay_file);

    let replay_file_clone = cli.replay_file.clone().unwrap();

    if !replay_file_clone.exists() {
        anyhow::bail!(".replay file not foun: {:?}", cli.replay_file);
    }

    let replay_bytes = fs::read(&replay_file_clone)
        .with_context(|| format!("Error reading .replay file: {:?}", replay_file_clone))?;

    let replay_data = boxcars::ParserBuilder::new(&replay_bytes)
        .must_parse_network_data()
        .parse()
        .with_context(|| format!("Error parsing .replay file: {:?}", replay_file_clone))?;

    let full_replay_json = serde_json::to_string_pretty(&replay_data)
        .context("Error serializing full .replay file to json")?;
        
    if let Some(output_path) = cli.output {
        println!("--- Full Replay Data ---");
        println!("{}", &full_replay_json);
        fs::write(&output_path, full_replay_json)
            .with_context(|| format!("Error writing replay to: {:?}", output_path));
        println!("Full .replay file data written to: {:?}", output_path);
    }

    Ok(())
}