use std::fs;
use std::path::PathBuf;

fn main() {
    let paths = fs::read_dir("./").unwrap();
    for path in paths {
        let path_string = path.unwrap().path().display().to_string();
        let len = path_string.len();
        let modified_path_string: String = path_string[2..len].parse().unwrap();
        if !modified_path_string.starts_with(".") {
            println!("Name: {}", modified_path_string);
            let path_buf = PathBuf::from(path_string);
            println!("is_dir: {}", path_buf.is_dir());
            println!("is_file: {}", path_buf.is_file())
        }
    }
}
