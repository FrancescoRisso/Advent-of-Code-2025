use std::{
    collections::HashSet,
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn load_file_and_compute(path: &str) -> i32 {
    let mut beams: HashSet<usize> = HashSet::new();
    let mut splits = 0;

    let file = File::open(path);

    match file {
        Err(err) => panic!("{err}"),
        Ok(file) => {
            let mut lines = BufReader::new(file).lines();

            let line = lines.next().unwrap().unwrap();
            let start_beam = line.chars().position(|ch| ch == 'S').unwrap();
            beams.insert(start_beam);

            for line in lines.map_while(Result::ok) {
                for beam in beams.clone() {
                    if line.chars().nth(beam).unwrap() == '^' {
                        splits += 1;
                        beams.remove(&beam);
                        beams.insert(beam - 1);
                        beams.insert(beam + 1);
                    }
                }
            }
        }
    }

    splits
}

fn main() {
    let fname = env::args().nth(1).unwrap_or("../input.txt".into());

    let res = load_file_and_compute(&fname);

    println!("Output of file \"{}\": {:?}", fname, res);
}
