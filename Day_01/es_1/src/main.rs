use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Movement {
    sign: Direction,
    amount: i32,
}

impl Movement {
    fn new(line: &str) -> Self {
        let sign = match line.chars().nth(0) {
            Some('L') => Direction::Left,
            Some('R') => Direction::Right,
            _ => panic!("Wrongly formatted"),
        };

        let amount = (&line[1..]).parse::<i32>().unwrap();

        return Movement { sign, amount };
    }

    fn calc_pos(self, prev_pos: i32) -> i32 {
        match self.sign {
            Direction::Left => prev_pos - self.amount,
            Direction::Right => prev_pos + self.amount,
        }
    }
}

fn load() -> VecDeque<Movement> {
    let mut res: VecDeque<Movement> = VecDeque::new();

    let file = File::open("../input.txt");
    match file {
        Err(err) => panic!("{err}"),
        Ok(file) => {
            let lines = BufReader::new(file).lines();

            for line in lines.map_while(Result::ok) {
                res.push_back(Movement::new(&line));
            }
        }
    }

    res
}

fn calc(instructions: VecDeque<Movement>) -> i32 {
    let mut pos = 50;
    let mut cnt = 0;

    for instruction in instructions {
        pos = instruction.calc_pos(pos);
        if pos % 100 == 0 {
            cnt += 1;
        }
    }

    return cnt;
}

fn main() {
    let file = load();
    let solution = calc(file);
    println!("{}", solution);
}
