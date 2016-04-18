extern crate walkdir;
use walkdir::WalkDir;

use std::env;

use std::io::prelude::*;
use std::fs::File;


type Utf8Lines = Vec<(usize, String)>;


fn get_utf8_lines(content: &str) -> Utf8Lines {
    let mut lines = Utf8Lines::new();
    for (idx, line) in content.lines().enumerate() {
        let has_utf8 = line.chars().fold(false, |acc, c| acc || c > 128 as char );
        if has_utf8 {
            lines.push((idx, line.to_string()));
        }
    }
    lines
}

fn process_file(path: &str) -> Result<Utf8Lines, String> {
    let mut f = try!(File::open(path).map_err(|e| e.to_string()));

    let mut content = "".to_string();
    try!(f.read_to_string(&mut content).map_err(|e| e.to_string()));

    Ok(get_utf8_lines(&content))
}


fn main() {
    if env::args().len() < 2 {
        println!("Not enough parameters.\n\
                  Specify exactly one or more directories to visit.");
        return;
    }

    let args_iter = env::args().enumerate().filter(|&(idx, _)| idx != 0);
    for arg in args_iter {
        let (_, arg_str) = arg;
        for dir_file in WalkDir::new(&arg_str).into_iter().filter_map(|e| e.ok()) {
            let path_str = dir_file.path().to_str().unwrap();
            println!("\nFile: \"{}\"", path_str);

            let utf8_lines = process_file(path_str);
            if utf8_lines.is_err() {
                println!("Error occurred: {}", utf8_lines.unwrap_err());
                continue;
            }

            let utf8_lines = utf8_lines.unwrap();

            if utf8_lines.is_empty() {
                println!("(everything is ascii)");
                continue;
            }

            for line in utf8_lines {
                let (idx, data) = line;
                println!("[{}]\t{:?}", idx, data);
            }
        }
    }
}
