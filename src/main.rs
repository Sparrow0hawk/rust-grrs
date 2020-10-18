use structopt::StructOpt;
use anyhow::{Context, Result};
use log::{info};
use clap_verbosity_flag;
use grrs;

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
    grrs::find_matches(reader, &args.pattern, &mut std::io::stdout());
    info!("Finishing up");
    Ok(())
}

// testing module
#[cfg(test)]
mod tests {

    #[test]
    fn find_a_match() {
        let mut result = Vec::new();
        let f = std::fs::File::open("test_data/test.txt")
            .expect("The test file is not found.");
        let reader = std::io::BufReader::new(f);
        grrs::find_matches(reader, "foo", &mut result);
        assert_eq!(&result, b"foo: 100\n");
    }
}