use std::{
    env,
    fmt::Debug,
    fs::File,
    io::{BufRead, BufReader},
};

type T = i64;

fn min<T: PartialOrd>(a: T, b: T) -> T {
    if a < b { a } else { b }
}

fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

#[derive(Clone)]
struct SimpleRange {
    start: T,
    end: T,
}

impl SimpleRange {
    fn from_string(line: String) -> Self {
        let limits = line.split("-").collect::<Vec<&str>>();

        assert!(limits.len() == 2);

        let min = limits[0].parse::<T>().unwrap();
        let max = limits[1].parse::<T>().unwrap();

        Self {
            start: min,
            end: max,
        }
    }

    fn size(&self) -> T {
        self.end - self.start + 1
    }

    fn fully_before_than(&self, other: &SimpleRange) -> bool {
        self.end < other.start
    }

    fn fully_after_than(&self, other: &SimpleRange) -> bool {
        self.start > other.end
    }

    fn intersects(&self, other: &SimpleRange) -> bool {
        !self.fully_after_than(other) && !self.fully_before_than(other)
    }

    fn to_string(&self) -> String {
        let fact = 1; //10_000_000_000;
        format!("{}-{}", self.start / fact, self.end / fact)
    }
}

struct ComposedRange {
    ranges: Vec<SimpleRange>,
}

impl Debug for ComposedRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let content = self
            .ranges
            .iter()
            .map(|range| range.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        write!(f, "[{}]", content)
    }
}

impl ComposedRange {
    fn new() -> Self {
        Self {
            ranges: Vec::default(),
        }
    }

    fn items(&self) -> T {
        self.ranges
            .iter()
            .map(|range| range.size())
            .reduce(|prev, new| prev + new)
            .unwrap()
    }

    fn add_range(self, new: SimpleRange) -> Self {
        let mut new_composed_range: Vec<SimpleRange> = Vec::default();
        let mut i = 0;

        // keep all ranges fully before the new one
        while i < self.ranges.len() {
            if self.ranges[i].fully_before_than(&new) {
                new_composed_range.push(self.ranges[i].clone());
                i += 1;
            } else {
                break;
            }
        }
        // println!(
        //     "Fully before: {:?}",
        //     ComposedRange {
        //         ranges: new_composed_range.clone()
        //     }
        // );

        // manage "append to tail" case
        if i == self.ranges.len() {
            new_composed_range.push(new);
            return ComposedRange {
                ranges: new_composed_range,
            };
        }

        // join potential ranges overlapping with the new one
        let mut bridge_range = SimpleRange {
            start: min(new.start, self.ranges[i].start),
            end: new.end,
        };

        while i < self.ranges.len() {
            if bridge_range.intersects(&self.ranges[i]) {
                bridge_range.end = max(bridge_range.end, self.ranges[i].end);
                i += 1;
            } else {
                break;
            }
        }

        // add the bridge range
        // println!("Bridge: {:?}", bridge_range.to_string());
        new_composed_range.push(bridge_range);

        // add all ranges fully after the bridge
        while i < self.ranges.len() {
            new_composed_range.push(self.ranges[i].clone());
            i += 1;
        }

        Self {
            ranges: new_composed_range,
        }
    }
}

fn load_file_and_compute(path: &str) -> T {
    let mut intersected_ranges = ComposedRange::new();

    let file = File::open(path);

    match file {
        Err(err) => panic!("{err}"),
        Ok(file) => {
            let lines = BufReader::new(file).lines();

            for line in lines.map_while(Result::ok) {
                if line == "" {
                    break;
                }

                let new_range = SimpleRange::from_string(line);
                // println!("New range: {}", new_range.to_string());

                intersected_ranges = intersected_ranges.add_range(new_range);
                // println!("Result: {:?}\n", intersected_ranges);
            }
        }
    }

    intersected_ranges.items()
}

fn main() {
    let fname = env::args().nth(1).unwrap_or("../input.txt".into());

    let res = load_file_and_compute(&fname);

    println!("Output of file \"{}\": {:?}", fname, res);
}
