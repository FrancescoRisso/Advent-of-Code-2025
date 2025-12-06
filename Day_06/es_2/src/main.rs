use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    ops::{Add, Mul},
};

#[derive(Debug)]
enum Operator {
    Add,
    Mul,
}

impl Operator {
    fn from_string(s: char) -> Self {
        match s {
            '+' => Operator::Add,
            '*' => Operator::Mul,
            _ => panic!("Unknown operation"),
        }
    }

    fn apply<T: Add<Output = T> + Mul<Output = T>>(&self, lhs: T, rhs: T) -> T {
        match self {
            Operator::Add => lhs + rhs,
            Operator::Mul => lhs * rhs,
        }
    }
}

#[derive(Debug)]
struct Problem {
    operands: Vec<i16>,
    operator: Operator,
    num_operands: usize,
}

type ResType = i64;

impl Problem {
    fn new(operator: char, num_operands: usize) -> Self {
        Self {
            operands: vec![0; num_operands],
            operator: Operator::from_string(operator),
            num_operands: num_operands,
        }
    }

    fn add_operand(&mut self, operand: &str) {
        // println!("Add operand {} to {:?}", operand, self);

        for (prev, addition) in self.operands.iter_mut().zip(operand.chars()) {
            // print!("\tChar \"{}\" for {}", addition, prev);

            *prev = match addition {
                ' ' => *prev,
                '0'..='9' => *prev * 10 + (addition.to_digit(10).unwrap() as i16),
                other => panic!("Unrecognised digit \"{}\"", other),
            };

            // println!(" ==> {}", prev);
        }
    }

    fn solve(self) -> ResType {
        assert!(self.operands.len() > 1);

        let mut res = self.operands[0] as ResType;
        for operand in self.operands.iter().skip(1) {
            res = self.operator.apply(res, *operand as ResType);
        }

        res
    }
}

fn load_file_and_compute(path: &str) -> ResType {
    let file = File::open(path);
    let mut problems: Vec<Problem> = Vec::default();
    let mut prev_lines: Vec<String> = Vec::default();

    match file {
        Err(err) => panic!("{err}"),
        Ok(file) => {
            let lines = BufReader::new(file.try_clone().unwrap()).lines();

            for line in lines.map_while(Result::ok) {
                if !line.starts_with(&['+', '*']) {
                    prev_lines.push(line);
                    continue;
                }

                let mut operator = line.chars().nth(0).unwrap();
                let mut space_cnt = 0;

                for char in line.chars().skip(1) {
                    if char == ' ' {
                        space_cnt += 1;
                    } else {
                        problems.push(Problem::new(operator, space_cnt));
                        space_cnt = 0;
                        operator = char;
                    }
                }

                problems.push(Problem::new(operator, space_cnt + 1));
            }

            for line in prev_lines {
                let mut chars = line.chars();

                for problem in problems.iter_mut() {
                    let len = problem.num_operands;
                    let new_operand = chars.by_ref().take(len).collect::<String>();
                    problem.add_operand(&new_operand);

                    chars.next();
                }
            }

            // println!("{:?}", problems);

            problems
                .into_iter()
                .map(|problem| problem.solve())
                .reduce(|prev, new| prev + new)
                .unwrap()
        }
    }
}

fn main() {
    let fname = env::args().nth(1).unwrap_or("../input.txt".into());

    let res = load_file_and_compute(&fname);

    println!("Output of file \"{}\": {:?}", fname, res);
}
