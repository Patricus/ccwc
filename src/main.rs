use clap::Parser;
use std::{
    ffi::OsStr,
    fs::{self},
    io::{BufRead, BufReader},
    os::unix::fs::MetadataExt,
};

#[derive(Parser)]
struct Cli {
    flags: String,
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();

    println!("Flags: {:?}, Path: {:?}", args.flags, args.path);

    let Cli { flags, path } = args;

    if std::path::Path::new(&path).exists() == false {
        println!("File not found!");
        std::process::exit(1);
    }

    let file = match fs::File::open(&path) {
        Ok(file) => file,
        Err(_err) => {
            println!("Filed to open file!");
            std::process::exit(1);
        }
    };

    let file_name = &path
        .file_name()
        .take()
        .unwrap_or_else(|| &OsStr::new("Failed to read file name!"));

    match flags.as_str() {
        "-c" => {
            let metadata = match fs::metadata(&path) {
                Ok(mdata) => mdata,
                Err(err) => {
                    println!("{}", err);
                    std::process::exit(1);
                }
            };

            println!("{:?} {:?}", metadata.size(), file_name)
        }
        "-l" => {
            let mut lines: usize = 0;

            for _line in BufReader::new(file).lines() {
                lines += 1;
            }
            println!("{:?} {:?}", lines, file_name)
        }
        "-w" => {
            let mut word_count: usize = 0;

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

            println!("{:?} {:?}", word_count, file_name)
        }
        _ => println!("default"),
    }
}
