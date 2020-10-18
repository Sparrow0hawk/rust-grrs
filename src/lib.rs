use std::io::{BufRead, BufReader};
use std::fs::{File};

pub fn find_matches(reader : BufReader<File>, pattern : &str, mut writer: impl std::io::Write) {
    for line in reader.lines() {
        let line = line.expect("An error has occured.");
        if line.contains(pattern) {
            writeln!(writer, "{}", line)
                .expect("Some error has occured.");
        }
    }
}