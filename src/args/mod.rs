use std::fmt::Display;

use clap::Parser;

pub mod day;
use day::Day;

pub mod exercise;
use exercise::Exercise;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(required = true)]
    pub day: Day,

    #[arg(required = true)]
    pub exercise: Exercise,

    #[arg(required = true)]
    pub input_file: String,
}

impl Display for Args {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Executing exercise {} from day {} on file \"{}\"",
            self.exercise, self.day, self.input_file
        )
    }
}
