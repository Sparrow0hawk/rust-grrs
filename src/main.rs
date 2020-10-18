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

#[derive(Debug)]
struct CustomError(String);

// main function
fn main() -> Result<(), CustomError> {
    let args = Cli::from_args();
    let f = std::fs::File::open(&args.path)
        .map_err(|err| CustomError(format!("Error reading `{}`: {}", &args.path.to_str().unwrap(), err)))?;
    let reader = std::io::BufReader::new(f);
    
    for line in reader.lines() {
        println!("{}", line.expect("An error has occured."));
    }
    Ok(())
}
