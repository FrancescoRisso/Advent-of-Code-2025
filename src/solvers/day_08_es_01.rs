use std::collections::HashMap;

use crate::{args::Args, day_types::ExerciseEngine};

pub struct Solver08_1 {}

type JuncCoord = i128;

#[derive(Debug, PartialEq)]
struct JunctionBox {
    x: JuncCoord,
    y: JuncCoord,
    z: JuncCoord,
}

impl JunctionBox {
    fn new(line: String) -> Self {
        let coords: Vec<&str> = line.split(",").collect();
        assert!(coords.len() == 3);

        let x = coords[0].parse::<JuncCoord>().unwrap();
        let y = coords[1].parse::<JuncCoord>().unwrap();
        let z = coords[2].parse::<JuncCoord>().unwrap();

        Self { x, y, z }
    }

    fn dst_sqr(&self, other: &JunctionBox) -> JuncCoord {
        (self.x - other.x) * (self.x - other.x)
            + (self.y - other.y) * (self.y - other.y)
            + (self.z - other.z) * (self.z - other.z)
    }
}

impl ExerciseEngine for Solver08_1 {
    fn solve(&self, args: Args, lines: Vec<String>) -> i128 {
        let mut boxes: Vec<JunctionBox> = Vec::new();
        let mut circuit_ids: Vec<usize> = Vec::new();

        for (id, line) in lines.into_iter().enumerate() {
            boxes.push(JunctionBox::new(line));
            circuit_ids.push(id);
        }

        let mut distances: HashMap<(usize, usize), JuncCoord> = HashMap::new();

        for first_circuit in 0..boxes.len() {
            for other_circuit in first_circuit..boxes.len() {
                if first_circuit != other_circuit {
                    distances.insert(
                        (first_circuit, other_circuit),
                        boxes[first_circuit].dst_sqr(&boxes[other_circuit]),
                    );
                }
            }
        }

        let iterations = match args.input_file {
            crate::args::input_files::InputFile::Example => 10,
            crate::args::input_files::InputFile::Real => 1000,
            crate::args::input_files::InputFile::Test => todo!(),
        };

        for _ in 0..iterations {
            let closest_pair = distances
                .iter()
                .min_by(|(_k1, v1), (_k2, v2)| v1.cmp(v2))
                .unwrap()
                .0;

            let (link_sm, link_gt) = *closest_pair;

            let circuit_to_join = circuit_ids[link_gt];
            let circuit_to_be_joined = circuit_ids[link_sm];

            if circuit_to_be_joined != circuit_to_join {
                for boxid in 0..circuit_ids.len() {
                    if circuit_ids[boxid] == circuit_to_be_joined {
                        circuit_ids[boxid] = circuit_to_join
                    };
                }
            }
            distances.remove(&(link_sm, link_gt));
        }

        let mut circuit_sizes: Vec<i32> = vec![0; circuit_ids.len()];
        for id in circuit_ids {
            circuit_sizes[id] += 1;
        }

        circuit_sizes.sort_by(|a, b| b.cmp(a));

        (circuit_sizes[0] as i128) * (circuit_sizes[1] as i128) * (circuit_sizes[2] as i128)
    }
}
