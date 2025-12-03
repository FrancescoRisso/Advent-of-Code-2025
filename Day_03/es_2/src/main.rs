use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
    ops::Add,
};

const MIN_DIGIT_CHAR: char = char::from_u32(0).unwrap();
const NUM_BATTERIES: usize = 12;

#[derive(Debug)]
struct Battery {
    value: u8,
    rest_of_pack: Option<String>,
    value_cumul: u64,
}

impl Add for Battery {
    type Output = Battery;

    fn add(self, rhs: Self) -> Self::Output {
        // println!("{} + {}", self.value_cumul, rhs.value_cumul);

        Battery {
            value: rhs.value,
            rest_of_pack: rhs.rest_of_pack,
            value_cumul: 10 * self.value_cumul + rhs.value_cumul,
        }
    }
}

impl Battery {
    fn new(val: char, rest_of_pack: Option<&str>) -> Self {
        let val_number: u64 = val.to_digit(10).unwrap().into();

        Self {
            value: val_number.try_into().unwrap(),
            rest_of_pack: rest_of_pack.map(|rest_str| String::from(rest_str)),
            value_cumul: val_number,
        }
    }

    fn new_pack(rest_of_pack: String) -> Self {
        Self {
            value: 0,
            rest_of_pack: Some(rest_of_pack),
            value_cumul: 0,
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

fn find_max_n(battery_bank: String) -> u64 {
    let mut chosen = Battery::new_pack(battery_bank);

    for choice_no in 0..NUM_BATTERIES {
        let next = find_max_single(
            &chosen.rest_of_pack.as_ref().unwrap(),
            NUM_BATTERIES - choice_no - 1,
        );
        chosen = chosen + next;
    }

    chosen.value_cumul
}

fn main() {
    let battery_banks = load_file("../input.txt");
    // let battery_banks = load_file("../input.example.txt");

    // println!("{:?}", battery_banks);

    let res = battery_banks
        .into_iter()
        .map(|bank| find_max_n(bank))
        .reduce(|prev, new| prev + new)
        .unwrap();

    println!("{:?}", res);
}
