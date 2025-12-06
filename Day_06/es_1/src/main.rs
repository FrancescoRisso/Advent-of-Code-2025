use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    ops::{Add, Mul},
};

enum Operator {
    Add,
    Mul,
}

impl Operator {
    fn from_string(s: &str) -> Self {
        match s {
            "+" => Operator::Add,
            "*" => Operator::Mul,
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

struct Problem {
    operands: Vec<i16>,
    operator: Option<Operator>,
}

fn parse_operand(operand: &str) -> i16 {
    operand.parse::<i16>().unwrap()
}

type ResType = i64;

impl Problem {
    fn new(first_operand: &str) -> Self {
        Self {
            operands: Vec::from([parse_operand(first_operand)]),
            operator: None,
        }
    }

    fn add_operand(&mut self, operand: &str) {
        self.operands.push(parse_operand(operand));
    }

    fn set_operator(&mut self, operator: &str) {
        self.operator = Some(Operator::from_string(operator))
    }

    fn solve(self) -> ResType {
        assert!(self.operands.len() > 1);
        assert!(self.operator.is_some());

        let operator = self.operator.unwrap();

        let mut res = self.operands[0] as ResType;
        for operand in self.operands.iter().skip(1) {
            res = operator.apply(res, *operand as ResType);
        }

        res
    }
}

fn load_file_and_compute(path: &str) -> ResType {
    let file = File::open(path);

    match file {
        Err(err) => panic!("{err}"),
        Ok(file) => {
            let mut lines = BufReader::new(file).lines();

            let line = lines.next().unwrap().unwrap();
            let mut problems: Vec<Problem> = line
                .trim()
                .split_whitespace()
                .map(|num| Problem::new(num))
                .collect();

            for mut line in lines.map_while(Result::ok) {
                line = line.trim().to_string();

                let is_operator = line.starts_with(&['+', '*']);

                for (col, problem) in line.split_whitespace().zip(problems.iter_mut()) {
                    if is_operator {
                        problem.set_operator(col);
                    } else {
                        problem.add_operand(col);
                    }
                }
            }

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
