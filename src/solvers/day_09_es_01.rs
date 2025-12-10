use std::cmp::max;

use crate::day_types::ExerciseEngine;

pub struct Solver09_1 {}

type RectType = i64;

struct Rect {
    corner1: (RectType, RectType),
    corner2: (RectType, RectType),
}

fn abs(val: RectType) -> RectType {
    if val > 0 { val } else { -val }
}

impl Rect {
    fn from_tiles(corner1: &Tile, corner2: &Tile) -> Self {
        Self {
            corner1: (corner1.x, corner1.y),
            corner2: (corner2.x, corner2.y),
        }
    }

    fn area_inside(&self) -> RectType {
        let width = abs(self.corner1.0 - self.corner2.0) + 1;
        let height = abs(self.corner1.1 - self.corner2.1) + 1;

        width * height
    }
}

#[derive(Debug, Clone)]
struct Tile {
    x: RectType,
    y: RectType,
}

impl Tile {
    fn from_string(line: String) -> Self {
        let coords = line.split(",").collect::<Vec<&str>>();
        assert!(coords.len() == 2);

        let x: RectType = coords[0].parse::<RectType>().unwrap();
        let y: RectType = coords[1].parse::<RectType>().unwrap();

        Self { x, y }
    }
}

impl ExerciseEngine for Solver09_1 {
    fn solve(&self, _: crate::args::Args, lines: Vec<String>) -> i128 {
        let tiles = lines
            .into_iter()
            .map(|line| Tile::from_string(line))
            .collect::<Vec<Tile>>();

        let mut max_area: RectType = 0;
        for tile1 in 0..tiles.len() {
            for tile2 in tile1 + 1..tiles.len() {
                max_area = max(
                    max_area,
                    Rect::from_tiles(&tiles[tile1], &tiles[tile2]).area_inside(),
                );
            }
        }

        max_area.into()
    }
}
