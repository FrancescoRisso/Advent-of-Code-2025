use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn load_file(path: &str) -> Vec<String> {
    let file = File::open(path);

    match file {
        Err(err) => panic!("{err}"),
        Ok(file) => {
            let lines = BufReader::new(file).lines();
            lines.map_while(Result::ok).collect()
        }
    }
}
