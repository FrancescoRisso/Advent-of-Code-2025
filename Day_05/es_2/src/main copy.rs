use std::{
    env,
    fmt::Debug,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Clone)]
struct Range {
    start: u128,
    end: u128,
}


impl Debug for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // f.debug_struct("Range").field("start", &self.start).field("end", &self.end).finish()
        let fact = 1; //10_000_000_000;
        write!(f, "{}-{}", self.start / fact, self.end / fact)
    }
}

fn min<T: PartialOrd>(a: T, b: T) -> T {
    if a < b { a } else { b }
}

impl Range {
    fn new(line: String) -> Self {
        let limits = line.split("-").collect::<Vec<&str>>();

        assert!(limits.len() == 2);

        let min = limits[0].parse::<u128>().unwrap();
        let max = limits[1].parse::<u128>().unwrap();

        Self {
            start: min,
            end: max,
        }
    }

    fn size(&self) -> u128 {
        // println!("{} - {} + 1", self.end, self.start);
        self.end - self.start + 1
    }

    fn all_before_than(&self, other: &Range) -> bool {
        self.end < other.start
    }

    fn all_after_than(&self, other: &Range) -> bool {
        self.start > other.end
    }

    fn intersect_single(&self, other: &Range) -> bool {
        !self.all_after_than(other) && !self.all_before_than(other)
    }

    fn intersect_multi(self, others: &mut Vec<Range>) {
        let mut index = 0;

        for other in others.iter() {
            if self.all_after_than(other) {
                index += 1;
            } else {
                break;
            }
        }

        // others[index] is either intersecting or fully after self, if it exists

        // if (others.len() > index) && self.intersect_single(&others[index]){
        // 	index=
        // }
        // let index = min(index - 1, 0) as usize;

        let tmp = self.clone();

        // if index == -1 {
        //     if others.len() == 0 || self.end < others[0].start {
        //         others.insert(0, self);
        //         println!("{:?} insert head", tmp);
        //         return;
        //     }

        //     index = 0;
        // }

        if index >= others.len() {
            others.insert(index, self);
            println!("{:?} insert tail", tmp);
        }

        let intersects_before = self.intersect_single(&others[index]);
		// otherwhise, index is fully after self 

		let next_status = if index_status == StatusBefore::Before {
			StatusBefore::Intersecting
		} else if index+1 >= others.len() || !

        let intersect_before = (others.len() > index) && self.start <= others[index].end;
        let intersects_after = (others.len() > index + 1) && (self.end >= others[index + 1].start);

        match (intersect_before, intersects_after) {
            (false, false) => others.insert(index, self),
            (true, false) => others[index].end = self.end,
            (false, true) => others[index + 1].start = self.start,
            (true, true) => {
                others[index].end = others[index + 1].end;
                others.remove(index + 1);
            }
        }

        println!(
            "{:?} {} {} {}",
            tmp, intersect_before, intersects_after, index
        );

        return;
    }
}

fn load_file_and_compute(path: &str) -> u128 {
    let mut intersected_ranges = Vec::<Range>::new();

    let file = File::open(path);

    match file {
        Err(err) => panic!("{err}"),
        Ok(file) => {
            let lines = BufReader::new(file).lines();

            for line in lines.map_while(Result::ok) {
                if line == "" {
                    break;
                }

                Range::new(line).intersect_multi(&mut intersected_ranges);
                println!("{:?}\n", intersected_ranges);
            }
        }
    }

    intersected_ranges
        .iter()
        .map(|range| range.size())
        .reduce(|prev, new| prev + new)
        .unwrap()
}

fn main() {
    let fname = env::args().nth(1).unwrap_or("../input.txt".into());

    let res = load_file_and_compute(&fname);

    println!("Output of file \"{}\": {:?}", fname, res);
}
