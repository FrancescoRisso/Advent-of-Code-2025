use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
struct Range {
    start: i64,
    end: i64,
}

impl Range {
    fn new(line: String) -> Self {
        let limits = line.split("-").collect::<Vec<&str>>();

        assert!(limits.len() == 2);

        let min = limits[0].parse::<i64>().unwrap();
        let max = limits[1].parse::<i64>().unwrap();

        Self {
            start: min,
            end: max,
        }
    }

    fn contains(&self, id: i64) -> bool {
        id >= self.start && id <= self.end
    }
}

fn load_file(path: &str) -> (Vec<Range>, Vec<i64>) {
    let mut ranges: Vec<Range> = Vec::default();
    let mut ids: Vec<i64> = Vec::default();
    let mut empty_line_found = false;

    let file = File::open(path);

    match file {
        Err(err) => panic!("{err}"),
        Ok(file) => {
            let lines = BufReader::new(file).lines();

            for line in lines.map_while(Result::ok) {
                if line == "" {
                    empty_line_found = true;
                    continue;
                }

                if empty_line_found {
                    ids.push(i64::from_str_radix(&line, 10).unwrap());
                } else {
                    ranges.push(Range::new(line));
                }
            }
        }
    }

    (ranges, ids)
}

fn id_matches(id: i64, ranges: &Vec<Range>) -> bool {
    for range in ranges {
        if range.contains(id) {
            return true;
        }
    }

    false
}

fn main() {
    let (ranges, ids) = load_file("../input.txt");
    // let (ranges, ids) = load_file("../input.example.txt");

    // println!("{:?}", ranges);
    // println!("{:?}", ids);

    let res = ids
        .into_iter()
        .map(|id| id_matches(id, &ranges))
        .filter(|fresh| *fresh)
        .count();

    println!("{:?}", res);
}
