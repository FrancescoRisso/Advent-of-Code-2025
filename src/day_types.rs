use crate::args::Args;

pub trait ExerciseEngine {
    fn solve(&self, args: Args, lines: Vec<String>) -> i128;
}
