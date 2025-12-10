mod args;
mod day_types;
mod load;
mod solvers;

use crate::load::load_file;
use crate::solvers::day_08_es_01::Solver08_1;
use crate::solvers::day_08_es_02::Solver08_2;
use crate::solvers::day_09_es_01::Solver09_1;
use args::Args;
use clap::Parser;

use crate::day_types::ExerciseEngine;
use args::day::Day;
use args::exercise::Exercise;

fn main() {
    let args = Args::parse();
    println!("{}", args);

    let exercise: &dyn ExerciseEngine = match (&args.day, &args.exercise) {
        (Day::Eight, Exercise::One) => &Solver08_1 {},
        (Day::Eight, Exercise::Two) => &Solver08_2 {},
        (Day::Nine, Exercise::One) => &Solver09_1 {},
        (Day::Nine, Exercise::Two) => unimplemented!(),
        _ => todo!(),
    };

    let fpath = &args.input_file.path(&args.day);

    let file_content = load_file(&fpath);
    println!("{}", exercise.solve(args, file_content));
}
