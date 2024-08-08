use clap::Parser;
use std::fs;

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
        return;
    }

    // let file = fs::File::open(&path);
    let file_name = &path.file_name().take();

    match flags.as_str() {
        "-c" => println!("{:?} {:?}", fs::metadata(&path).unwrap().len(), file_name),
        "-l" => println!("{:?} {:?}", fs::metadata(&path).unwrap().len(), file_name),
        _ => println!("default"),
    }
}
