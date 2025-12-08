mod args;
mod day_types;
mod load;
mod solvers;

use crate::load::load_file;
use crate::solvers::day_08::Solver08_1;
use args::Args;
use clap::Parser;

use crate::day_types::ExerciseEngine;
use args::day::Day;
use args::exercise::Exercise;

fn main() {
    let args = Args::parse();
    println!("{}", args);

    let exercise: &dyn ExerciseEngine = match (args.day, args.exercise) {
        (Day::Eight, Exercise::One) => &Solver08_1 {},
        _ => todo!(),
    };

    let file_content = load_file(&args.input_file);
    println!("{}", exercise.solve(file_content));
}
