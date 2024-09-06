use clap::{Arg, ArgAction, Command};
use fs;

//#[derive(Parser)]
//struct Cli {
//    input: Option<String>,
//}

fn main() {
    // Get input
    //let Cli { input } = Cli::parse();
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
        _ => {
            println!("No input!");
            std::process::exit(1);
        }
    }

    // TODO: open file, if error then use string input, else steam file.

    // Open file
    let file = match fs::File::open(input) {
        Ok(file) => file,
        Err(_err) => {
            println!("Failed to open file!");
            std::process::exit(1);
        }
    };

    let file_name = path
        .expect("Path should exist")
        .file_name()
        .take()
        .unwrap_or_else(|| &OsStr::new("Failed to read file name!"));

    match flags.as_str() {
        "-c" => {
            let metadata = match fs::metadata(path) {
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
        "-m" => {
            let mut char_count: usize = 0;

            for line in BufReader::new(file).lines() {
                match line {
                    Ok(words) => {
                        char_count += words.len();
                    }
                    _ => continue,
                }
            }

            println!("{:?} {:?}", char_count, file_name)
        }
        _ => {
            let metadata = match fs::metadata(&path) {
                Ok(mdata) => mdata,
                Err(err) => {
                    println!("{}", err);
                    std::process::exit(1);
                }
            };

            let mut lines: usize = 0;
            let mut word_count: usize = 0;

            for line in BufReader::new(file).lines() {
                lines += 1;
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

            println!(
                "{:?} {:?} {:?} {:?}",
                metadata.size(),
                lines,
                word_count,
                file_name
            );
        }
    }
}
