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
}
