use std::{
    fs::File,
    io::{BufReader, Lines},
};

pub trait ExerciseEngine {
    fn solve(&self, lines: Lines<BufReader<File>>) -> i128;
}
