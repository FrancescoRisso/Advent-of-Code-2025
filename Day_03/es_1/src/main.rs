use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
    ops::Add,
};

const MIN_DIGIT_CHAR: char = char::from_u32(0).unwrap();

#[derive(Debug)]
struct Battery {
    value: u8,
    rest_of_pack: Option<String>,
}

impl Add for Battery {
    type Output = u32;

    fn add(self, rhs: Self) -> Self::Output {
        10 * Into::<u32>::into(self.value) + Into::<u32>::into(rhs.value)
    }
}

impl Battery {
    fn new(val: char, rest_of_pack: Option<&str>) -> Self {
        Self {
            value: val.to_digit(10).unwrap().try_into().unwrap(),
            rest_of_pack: rest_of_pack.map(|rest_str| String::from(rest_str)),
        }
    }
}

fn load_file(path: &str) -> VecDeque<String> {
    let mut res: VecDeque<String> = VecDeque::new();

    let file = File::open(path);
    match file {
        Err(err) => panic!("{err}"),
        Ok(file) => {
            let lines = BufReader::new(file).lines();

            for line in lines.map_while(Result::ok) {
                res.push_back(line);
            }
        }
    }

    res
}

fn find_max_single(pack: &str, remaining_to_choose: usize) -> Battery {
    let mut max = MIN_DIGIT_CHAR;
    let mut max_index = 0;

    let mut chars = pack.chars().collect::<Vec<char>>();

    chars = chars
        .into_iter()
        .rev()
        .skip(remaining_to_choose)
        .rev()
        .collect::<Vec<char>>();

    for (index, val) in chars.iter().enumerate() {
        if *val == '9' {
            let rest = if remaining_to_choose == 0 {
                None
            } else {
                Some(&pack[index + 1..])
            };
            return Battery::new(*val, rest);
        };

        if *val > max {
            max = *val;
            max_index = index;
        }
    }

    Battery::new(max, Some(&pack[max_index + 1..]))
}

fn find_max_double(battery_bank: String) -> u32 {
    let first = find_max_single(&battery_bank, 1);
    let second = find_max_single(&first.rest_of_pack.as_ref().unwrap(), 0);
    first + second
}

fn main() {
    let battery_banks = load_file("../input.txt");
    // let battery_banks = load_file("../input.example.txt");

    // println!("{:?}", battery_banks);

    let res = battery_banks
        .into_iter()
        .map(|bank| find_max_double(bank))
        .reduce(|prev, new| prev + new)
        .unwrap();

    println!("{:?}", res);
}
