use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const MAX_LINES: usize = 1000; // Maximum number of lines to read

fn fetch_lines_from_file(file_path: &str) -> io::Result<[String; MAX_LINES]> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let mut lines: [String; MAX_LINES] = Default::default();
    let mut i = 0;
    for line in reader.lines() {
        if let Ok(line) = line {
            lines[i] = line;
            i += 1;
            if i >= MAX_LINES {
                break;
            }
        }
    }
    Ok(lines)
}

fn main() {
    let file_path = "path/to/your/file.txt";
    match fetch_lines_from_file(file_path) {
        Ok(lines) => {
            for line in lines.iter().filter(|line| !line.is_empty()) {
                println!("{}", line);
            }
        }
        Err(error) => {
            eprintln!("Error reading file: {}", error);
        }
    }
}
