use structopt::StructOpt;
use std::io::BufRead;
use anyhow::{Context, Result};

// search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    // the pattern to look for
    pattern: String,
    // the path to the files to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

#[derive(Debug)]
struct CustomError(String);

// main function
fn main() -> Result<()> {
    let args = Cli::from_args();
    let f = std::fs::File::open(&args.path)
        .with_context(|| format!("Could not read file `{}`", &args.path.to_str().unwrap()))?;
    let reader = std::io::BufReader::new(f);
    
    for line in reader.lines() {
        println!("{}", line.expect("An error has occured."));
    }
    Ok(())
}
