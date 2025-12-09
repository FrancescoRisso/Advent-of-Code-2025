use std::collections::HashMap;

use crate::{args::Args, day_types::ExerciseEngine};

pub struct Solver08_2 {}

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

impl ExerciseEngine for Solver08_2 {
    fn solve(&self, _: Args, lines: Vec<String>) -> i128 {
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

        loop {
            let closest_pair = distances
                .iter()
                .min_by(|(_k1, v1), (_k2, v2)| v1.cmp(v2))
                .unwrap()
                .0;

            let (link_sm, link_gt) = *closest_pair;

            let circuit_to_join = circuit_ids[link_gt];
            let circuit_to_be_joined = circuit_ids[link_sm];

            for box1 in 0..circuit_ids.len() {
                if circuit_ids[box1] != circuit_to_join {
                    continue;
                }

                for box2 in 0..circuit_ids.len() {
                    if circuit_ids[box2] != circuit_to_be_joined {
                        continue;
                    }

                    distances.remove(&(box1, box2));
                    distances.remove(&(box2, box1));
                }
            }

            for boxid in 0..circuit_ids.len() {
                if circuit_ids[boxid] == circuit_to_be_joined {
                    circuit_ids[boxid] = circuit_to_join;
                };
            }

            // distances.remove(&(link_sm, link_gt));

            let first_circuit_size = circuit_ids
                .iter()
                .filter(|id| **id == circuit_ids[0])
                .count();
            print!("\r{}/{}", first_circuit_size, circuit_ids.len());

            if first_circuit_size == circuit_ids.len() {
                let first_box = &boxes[link_sm];
                let other_box = &boxes[link_gt];

                println!("");
                return (first_box.x as i128) * (other_box.x as i128);
            }
        }
    }
}
