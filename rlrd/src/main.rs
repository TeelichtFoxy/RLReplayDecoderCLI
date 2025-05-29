use anyhow::{Context, Result};
use clap::Parser;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(short, long, value_parser)]
    replay_file: PathBuf,

    #[clap(short, long, value_parser)]
    output: Option<PathBuf>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    println!("Reading .replay file: {:?}", cli.replay_file);

    if !cli.replay_file.exists() {
        anyhow::bail!(".replay file not foun: {:?}", cli.replay_file);
    }

    let replay_bytes = fs::read(&cli.replay_file)
        .with_context(|| format!("Error reading .replay file: {:?}", cli.replay_file))?;

    let replay_data = boxcars::ParserBuilder::new(&replay_bytes)
        .must_parse_network_data()
        .parse()
        .with_context(|| format!("Error parsing .replay file: {:?}", cli.replay_file))?;

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