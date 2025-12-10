use std::{collections::HashSet, str::Chars};

use crate::day_types::ExerciseEngine;

pub struct Solver10_2 {}

#[derive(Debug)]
struct Button {
    targets: Vec<usize>,
}

fn parse_num_array(line: &mut Chars<'_>) -> Vec<usize> {
    let mut nums: Vec<usize> = Vec::new();
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
                nums.push(num);
                num = 0;
            }
            ')' | '}' => {
                nums.push(num);
                break;
            }
            _ => panic!("Unrecognized character"),
        }
    }

    nums
}

impl Button {
    fn from_string(line: &mut Chars<'_>) -> Self {
        Button {
            targets: parse_num_array(line),
        }
    }

    fn push(&self, cur_status: &Vec<usize>, targets: &Vec<usize>) -> Option<Vec<usize>> {
        let mut new_status = cur_status.clone();

        for target in self.targets.iter() {
            if new_status[*target] == targets[*target] {
                return None;
            }

            new_status[*target] += 1;
        }

        Some(new_status)
    }
}

#[derive(Debug)]
struct Machine {
    joltage_target: Vec<usize>,
    buttons: Vec<Button>,
}

impl Machine {
    fn from_string(line: String) -> Self {
        let mut buttons: Vec<Button> = Vec::new();

        let mut chars = line.chars();

        assert!(chars.next().unwrap() == '[');

        loop {
            match chars.next().unwrap() {
                ']' => break,
                _ => {}
            }
        }

        loop {
            match chars.next().unwrap() {
                ' ' => continue,
                '(' => buttons.push(Button::from_string(&mut chars)),
                '{' => break,
                _ => panic!("Unrecognized description character"),
            };
        }

        let joltages: Vec<usize> = parse_num_array(&mut chars);

        Machine {
            joltage_target: joltages,
            buttons,
        }
    }

    fn pushes_required(&self) -> i128 {
        let mut iters = 0_i128;
        let mut statuses: HashSet<Vec<usize>> = HashSet::new();
        let mut next_step_statuses: HashSet<Vec<usize>> = HashSet::new();

        statuses.insert(vec![0; self.joltage_target.len()]);

        loop {
            iters += 1;

            for status in statuses.iter() {
                for button in self.buttons.iter() {
                    if let Some(new_status) = button.push(status, &self.joltage_target) {
                        next_step_statuses.insert(new_status);
                    }
                }
            }

            if next_step_statuses.contains(&self.joltage_target) {
                break;
            }

            let max_tot_joltage = next_step_statuses
                .clone()
                .into_iter()
                .map(|joltages| {
                    joltages
                        .into_iter()
                        .reduce(|prev, next| prev + next)
                        .unwrap()
                })
                .max()
                .unwrap();

            let limit = max_tot_joltage as isize - 2 * self.joltage_target.len() as isize;

            statuses = next_step_statuses
                .into_iter()
                .filter(|joltage| {
                    joltage
                        .clone()
                        .into_iter()
                        .reduce(|prev, next| prev + next)
                        .unwrap() as isize
                        > limit
                })
                .collect();
            next_step_statuses = HashSet::new();
            println!("Iter {} has {} possible states", iters, statuses.len());
        }

        println!("Machine completed");
        return iters;
    }
}

impl ExerciseEngine for Solver10_2 {
    fn solve(&self, _: crate::args::Args, lines: Vec<String>) -> i128 {
        lines
            .into_iter()
            .map(|line| Machine::from_string(line).pushes_required())
            .reduce(|prev, next| prev + next)
            .unwrap()
    }
}
