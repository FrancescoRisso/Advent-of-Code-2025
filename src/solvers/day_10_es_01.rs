use std::{collections::HashSet, str::Chars};

use crate::day_types::ExerciseEngine;

pub struct Solver10_1 {}

#[derive(Debug)]
struct Button {
    targets: Vec<usize>,
}

impl Button {
    fn from_string(line: &mut Chars<'_>) -> Self {
        let mut targets: Vec<usize> = Vec::new();
        let mut num = 0_usize;

        loop {
            match line.next().unwrap() {
                '0' => num = num * 10,
                '1' => num = num * 10 + 1,
                '2' => num = num * 10 + 2,
                '3' => num = num * 10 + 3,
                '4' => num = num * 10 + 4,
                '5' => num = num * 10 + 5,
                '6' => num = num * 10 + 6,
                '7' => num = num * 10 + 7,
                '8' => num = num * 10 + 8,
                '9' => num = num * 10 + 9,
                ',' => {
                    targets.push(num);
                    num = 0;
                }
                ')' => {
                    targets.push(num);
                    break;
                }
                _ => panic!("Unrecognized button character"),
            }
        }

        Button { targets }
    }

    fn push(&self, cur_status: &Vec<bool>) -> Vec<bool> {
        let mut new_status = cur_status.clone();

        for target in self.targets.iter() {
            new_status[*target] = !new_status[*target];
        }

        new_status
    }
}

#[derive(Debug)]
struct Machine {
    lights_target: Vec<bool>,
    buttons: Vec<Button>,
}

impl Machine {
    fn from_string(line: String) -> Self {
        let mut lights: Vec<bool> = Vec::new();
        let mut buttons: Vec<Button> = Vec::new();

        let mut chars = line.chars();

        assert!(chars.next().unwrap() == '[');

        loop {
            match chars.next().unwrap() {
                '.' => lights.push(false),
                '#' => lights.push(true),
                ']' => break,
                _ => panic!("Unrecognized light character"),
            };
        }

        loop {
            match chars.next().unwrap() {
                ' ' => continue,
                '(' => buttons.push(Button::from_string(&mut chars)),
                '{' => break,
                _ => panic!("Unrecognized description character"),
            };
        }

        Machine {
            lights_target: lights,
            buttons,
        }
    }

    fn pushes_required(&self) -> i128 {
        let mut iters = 0_i128;
        let mut statuses: HashSet<Vec<bool>> = HashSet::new();
        let mut next_step_statuses: HashSet<Vec<bool>> = HashSet::new();

        statuses.insert(vec![false; self.lights_target.len()]);

        loop {
            iters += 1;

            for status in statuses.iter() {
                for button in self.buttons.iter() {
                    next_step_statuses.insert(button.push(status));
                }
            }

            if next_step_statuses.contains(&self.lights_target) {
                break;
            }

            statuses = next_step_statuses;
            next_step_statuses = HashSet::new();
        }

        return iters;
    }
}

impl ExerciseEngine for Solver10_1 {
    fn solve(&self, _: crate::args::Args, lines: Vec<String>) -> i128 {
        lines
            .into_iter()
            .map(|line| Machine::from_string(line).pushes_required())
            .reduce(|prev, next| prev + next)
            .unwrap()
    }
}
