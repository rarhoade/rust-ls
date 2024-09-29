use std::fs;

fn main() {
    let paths = fs::read_dir("./").unwrap();
    for path in paths {
        let mut path_string = path.unwrap().path().display().to_string();
        let len = path_string.len();
        path_string = path_string[2..len].parse().unwrap();
        if !path_string.starts_with(".") {
            println!("Name: {}", path_string)
        }
    }
}
