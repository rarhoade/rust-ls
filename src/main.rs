use std::{fs, io};
use std::io::Write;
use std::path::PathBuf;
use colored::Colorize;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    //Print hidden files
    #[arg(short, long, default_value_t = false)]
    all: bool
}

fn main() {
    let paths = fs::read_dir("./").unwrap();
    let mut file_vec = Vec::new();
    let mut directory_vec = Vec::new();
    let args = Args::parse();
    println!("{}", args.all);

    for path in paths {
        let path_string = path.unwrap().path().display().to_string();
        let len = path_string.len();
        let modified_path_string: String = path_string[2..len].parse().unwrap();
        if !modified_path_string.starts_with(".") {
            let path_buf = PathBuf::from(path_string);
            match path_buf.is_file() {
                true => file_vec.push(modified_path_string),
                false => directory_vec.push(modified_path_string)
            }
        }
    }
    print_vec(file_vec, false);
    print_vec(directory_vec, true);
    print!("\n");
    io::stdout().flush().unwrap();
}

fn print_vec(contents: Vec<String>, is_dir: bool) {
    for val in contents {
        match is_dir {
            true => print!("{}  ", val.bold().blue()),
            false => print!("{}  ", val),
        }
    }
}
