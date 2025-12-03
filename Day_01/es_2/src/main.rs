use std::{
    collections::VecDeque,
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, PartialEq)]
enum Direction {
    Left,
    Right,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Left => write!(f, "Left"),
            Direction::Right => write!(f, "Right"),
        }
    }
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

    fn calc_pos(self, prev_pos: &mut i32) -> i32 {
        let mut new_pos = match self.sign {
            Direction::Left => *prev_pos - self.amount,
            Direction::Right => *prev_pos + self.amount,
        };

        let factor = if new_pos < 0 { 100 } else { -100 };
        let mut cnt = 0;

        if *prev_pos == 0 {
            cnt = self.amount / 100;
            new_pos %= 100;
            new_pos += if new_pos < 0 { 100 } else { 0 }
        } else {
            while new_pos < 0 || new_pos > 100 {
                new_pos += factor;
                cnt += 1;
            }

            new_pos %= 100;

            cnt += if new_pos % 100 == 0 { 1 } else { 0 };
        }

        println!(
            "Move {} {} from {} to {} => {}",
            self.sign, self.amount, *prev_pos, new_pos, cnt
        );

        *prev_pos = new_pos;

        cnt
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
        let new_cnt = instruction.calc_pos(&mut pos);
        cnt += new_cnt;
    }

    return cnt;
}

fn main() {
    let file = load();
    let solution = calc(file);
    println!("{}", solution);
}
