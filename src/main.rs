use structopt::StructOpt;
use std::io::BufRead;
use anyhow::{Context, Result};
use log::{info};
use clap_verbosity_flag;

// search for a pattern in a file and display the lines that contain it.
#[derive(Debug, StructOpt)]
struct Cli {
    // the pattern to look for
    pattern: String,
    // the path to the files to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,

    // add verbose flag
    #[structopt(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

#[derive(Debug)]
struct CustomError(String);

// main function
fn main() -> Result<()> {
    env_logger::init();
    info!("Starting up");
    let args = Cli::from_args();
    info!("Received arguments: `{}` {}", &args.pattern, &args.path.to_str().unwrap());
    let f = std::fs::File::open(&args.path)
        .with_context(|| format!("Could not read file `{}`", &args.path.to_str().unwrap()))?;
    let reader = std::io::BufReader::new(f);
    
    info!("Printing lines in file");
    for line in reader.lines() {
        println!("{}", line.expect("An error has occured."));
    }
    info!("Finishing up");
    Ok(())
}
