use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

// struct Splitter {
//     position: usize,
//     lchild: Option<Box<Splitter>>,
//     rchild: Option<Box<Splitter>>,
// }

// impl Splitter {
//     fn new(pos: usize) -> Self {
//         Self {
//             position: pos,
//             lchild: None,
//             rchild: None,
//         }
//     }

//     fn count_paths(&self) -> i32 {
//         let left_paths = match &self.lchild {
//             None => 1,
//             Some(splitter) => splitter.count_paths(),
//         };

//         let right_paths = match &self.lchild {
//             None => 1,
//             Some(splitter) => splitter.count_paths(),
//         };

//         left_paths + right_paths
//     }

// 	fn set_child_left(&mut self, child: Splitter) {
// 		let child = Box::new(x)
// 	}
// }

fn load_file_and_compute(path: &str) -> i64 {
    let file = File::open(path);

    match file {
        Err(err) => panic!("{err}"),
        Ok(file) => {
            let mut lines = BufReader::new(file).lines();

            let line = lines.next().unwrap().unwrap();
            let num_cols = line.chars().clone().count();

            let mut beams = vec![0_i64; num_cols];

            let start_beam = line.chars().position(|ch| ch == 'S').unwrap();
            beams[start_beam] = 1;

            for line in lines.map_while(Result::ok) {
                for (beam, paths_to_it) in beams
                    .clone()
                    .iter()
                    .enumerate()
                    .filter(|(_, cnt)| **cnt != 0)
                {
                    if line.chars().nth(beam).unwrap() == '^' {
                        beams[beam] -= paths_to_it;
                        beams[beam + 1] += paths_to_it;
                        beams[beam - 1] += paths_to_it;
                    }
                }
            }

            beams.into_iter().reduce(|prev, new| prev + new).unwrap()
        }
    }
}

fn main() {
    let fname = env::args().nth(1).unwrap_or("../input.txt".into());

    let res = load_file_and_compute(&fname);

    println!("Output of file \"{}\": {:?}", fname, res);
}
