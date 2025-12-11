mod args;
mod day_types;
mod load;
mod solvers;

use crate::load::load_file;
use crate::solvers::day_08_es_01::Solver08_1;
use crate::solvers::day_08_es_02::Solver08_2;
use crate::solvers::day_09_es_01::Solver09_1;
use crate::solvers::day_10_es_01::Solver10_1;
use crate::solvers::day_10_es_02::Solver10_2;
use crate::solvers::day_11::Solver11;
use args::Args;
use clap::Parser;

use crate::day_types::ExerciseEngine;
use args::day::Day;
use args::exercise::Exercise;

fn main() {
    let args = Args::parse();

    let exercise: &dyn ExerciseEngine = match (&args.day, &args.exercise) {
        (Day::Eight, Exercise::One) => &Solver08_1 {},
        (Day::Eight, Exercise::Two) => &Solver08_2 {},
        (Day::Nine, Exercise::One) => &Solver09_1 {},
        (Day::Nine, Exercise::Two) => unimplemented!(),
        (Day::Ten, Exercise::One) => &Solver10_1 {},
        (Day::Ten, Exercise::Two) => &Solver10_2 {},
        (Day::Eleven, _) => &Solver11 {},
        _ => todo!(),
    };

    let fpath = &args.input_file.path(&args.day);
    let alt_fpath = &args.input_file.path_alt(&args.day, &args.exercise);

    let (file_content, actual_path) = load_file(&fpath, Some(&alt_fpath));
	
    args.print(actual_path);
    println!("{}", exercise.solve(args, file_content));
}
