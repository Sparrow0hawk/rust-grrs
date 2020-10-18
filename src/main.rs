use structopt::StructOpt;
use std::io::BufRead;

// search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    // the pattern to look for
    pattern: String,
    // the path to the files to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

// main function
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();
    let f = std::fs::File::open(&args.path)?;
    let reader = std::io::BufReader::new(f);
    
    for line in reader.lines() {
        println!("{}", line?);
    }
    Ok(())
}
