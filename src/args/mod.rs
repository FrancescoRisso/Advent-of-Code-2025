use clap::Parser;

pub mod day;
use day::Day;

pub mod exercise;
use exercise::Exercise;

pub mod input_files;
use input_files::InputFile;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(required = true)]
    pub day: Day,

    #[arg(required = true)]
    pub exercise: Exercise,

    #[arg(required = true)]
    pub input_file: InputFile,
}

impl Args {
    pub fn print(&self, used_path: String) {
        println!(
            "Executing exercise {} from day {} on file \"{}\"",
            self.exercise, self.day, used_path
        )
    }
}
