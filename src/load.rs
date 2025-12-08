use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
};

pub fn load_file(path: &str) -> Lines<BufReader<File>> {
    let file = File::open(path);

    match file {
        Err(err) => panic!("{err}"),
        Ok(file) => {
            let lines = BufReader::new(file).lines();
            lines
        }
    }
}
