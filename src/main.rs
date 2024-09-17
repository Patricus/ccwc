// A clone of the wc CLI tool

use clap::{Arg, ArgAction, Command};
use std::fs;
use std::io::{BufRead, BufReader};
use std::os::unix::fs::MetadataExt;
use std::path::Path;

fn main() {
    let matches = Command::new("ccwc")
        .version("0.1")
        .about("ccwc: wc clone")
        .arg(
            Arg::new("bytes")
                .short('c')
                .long("bytes")
                .help("Input size in bytes")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("lines")
                .short('l')
                .long("lines")
                .help("Input number of lines")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("words")
                .short('w')
                .long("words")
                .help("Input number of words")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("chars")
                .short('m')
                .long("chars")
                .help("Input number of characters")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("input")
                .help("Text file path or string input")
                .action(ArgAction::Set)
                .required(true),
        )
        .get_matches();

    // get flags
    let bytes = match matches.get_one::<bool>("bytes") {
        Some(bytes) => bytes,
        _ => &false,
    };

    let lines = match matches.get_one::<bool>("lines") {
        Some(lines) => lines,
        _ => &false,
    };

    let words = match matches.get_one::<bool>("words") {
        Some(words) => words,
        _ => &false,
    };

    let chars = match matches.get_one::<bool>("chars") {
        Some(chars) => chars,
        _ => &false,
    };

    let input = match matches.get_one::<String>("input") {
        Some(input) => input.to_owned(),
        // if no input, return
        _ => {
            println!("No input!");
            std::process::exit(1);
        }
    };

    let file = fs::File::open(&input);

    let metadata = match file {
        Ok(_) => fs::metadata(&input),
        Err(ref err) => Err(std::io::Error::new(err.kind(), err.to_string())),
    };

    let file_name = Path::new(&input).file_name();

    let mut values = Vec::new();

    // If bytes flag or no flags
    if *bytes || !*bytes && !*lines && !*words && !*chars {
        let size = match metadata {
            // if we have file metadata use the file size
            Ok(mdata) => mdata.size(),
            // if not return the size of the input string
            _ => input.as_bytes().len() as u64,
        };
        values.push(size.to_string());
    }

    // If lines flag or no flags
    if *lines || !*bytes && !*lines && !*words && !*chars {
        let mut lines: usize = 0;

        match &file {
            Ok(file) => {
                for _line in BufReader::new(file).lines() {
                    lines += 1;
                }
            }
            Err(_) => {
                for _line in input.lines() {
                    lines += 1;
                }
            }
        }

        values.push(lines.to_string());
    }

    // If words flag or no flags
    if *words || !*bytes && !*lines && !*words && !*chars {
        let mut word_count: usize = 0;

        match &file {
            Ok(file) => {
                for line in BufReader::new(file).lines() {
                    match line {
                        Ok(words) => {
                            if words.len() == 0 {
                                continue;
                            }
                            let words: Vec<&str> = words.trim().split(" ").collect();
                            word_count += words.len();
                        }
                        _ => continue,
                    }
                }
            }
            Err(_) => {
                for line in input.lines() {
                    if line.len() > 0 {
                        let words: Vec<&str> = line.trim().split(" ").collect();
                        word_count += words.len();
                    }
                }
            }
        }

        values.push(word_count.to_string());
    }

    // If chars flag
    if *chars {
        let mut char_count: usize = 0;

        match file {
            Ok(file) => {
                for line in BufReader::new(file).lines() {
                    match line {
                        Ok(words) => {
                            char_count += words.len();
                        }
                        _ => continue,
                    }
                }
            }
            Err(_) => {
                for line in input.lines() {
                    char_count += line.len();
                }
            }
        }

        values.push(char_count.to_string());
    }

    let mut return_str = values.join(" ").to_owned();

    // TODO: fix logic to determine weather the input is a file

    // If there is a file append the return_str
    if let Some(file_name) = file_name {
        if let Some(file_str) = file_name.to_str() {
            return_str.push_str(" ");
            return_str.push_str(file_str);
        }
    }

    println!("{}", return_str);
}
