use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn load_file(path: &str, alt_path: Option<&str>) -> (Vec<String>, String) {
    let file = File::open(path);

    match file {
        Err(err) => match alt_path {
            Some(other) => load_file(other, None),
            None => panic!("{err}"),
        },
        Ok(file) => {
            let lines = BufReader::new(file).lines();
            (lines.map_while(Result::ok).collect(), path.to_string())
        }
    }
}
