use std::{
    collections::VecDeque,
    fs::File,
    io::{BufReader, Read},
};

#[derive(Debug)]
struct NumRange {
    start: i64,
    end: i64,
}

impl NumRange {
    fn count_matching(self) -> i64 {
        let mut sum = 0;

        for num in self.start..self.end + 1 {
            if number_matches(num) {
                sum += num;
            }
        }

        sum
    }
}

fn number_matches(num: i64) -> bool {
    let num_digits = num.checked_ilog10().unwrap_or(0) + 1;

    if num_digits % 2 != 0 {
        return false;
    }

    let divider = 10_i64.checked_pow(num_digits / 2).unwrap();

    (num % divider) == (num / divider)
}

fn load_file(path: &str) -> VecDeque<NumRange> {
    let mut res: VecDeque<NumRange> = VecDeque::new();

    let file = File::open(path);
    match file {
        Err(err) => panic!("{err}"),
        Ok(file) => {
            let mut content: String = String::default();
            BufReader::new(file).read_to_string(&mut content).unwrap();

            let ranges = content.trim().split(",");
            for range in ranges {
                let limits = range.split("-").collect::<Vec<&str>>();

                assert!(limits.len() == 2);

                let min = limits[0].parse::<i64>().unwrap();
                let max = limits[1].parse::<i64>().unwrap();

                res.push_back(NumRange {
                    start: min,
                    end: max,
                });
            }
        }
    }

    res
}

fn main() {
    let data = load_file("../input.txt");
    // let data = load_file("../input.example.txt");
    // println!("{:?}", data);

    let res = data
        .into_iter()
        .map(|range| range.count_matching())
        .reduce(|prev, new| prev + new)
        .unwrap();

    println!("{}", res);
}
