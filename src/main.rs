use structopt::StructOpt;

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
fn main() {
    let args = Cli::from_args();
    let f = std::fs::File::open(&args.path)
        .expect("Unable to open.");
    let reader = std::io::BufReader::new(f);
    
    for line in reader.lines() {
        println!("{}", line);
    }
}
