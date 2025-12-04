use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const DELTAS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn load_file(path: &str) -> Vec<Vec<bool>> {
    let mut res: Vec<Vec<bool>> = Vec::default();
    let file = File::open(path);

    match file {
        Err(err) => panic!("{err}"),
        Ok(file) => {
            let lines = BufReader::new(file).lines();

            for line in lines.map_while(Result::ok) {
                res.push(line.chars().map(|cell| cell == '@').collect());
            }
        }
    }

    res
}

fn check_cell(
    grid: &Vec<Vec<bool>>,
    row: isize,
    col: isize,
    emptied_grid: &mut Vec<Vec<bool>>,
) -> bool {
    let mut cnt = 0;

    if !grid[row as usize][col as usize] {
        return false;
    }

    let num_rows = TryInto::<isize>::try_into(grid.len()).unwrap();
    let num_cols = TryInto::<isize>::try_into(grid[0].len()).unwrap();

    for (delta_row, delta_col) in DELTAS {
        let new_row = row + delta_row;
        let new_col = col + delta_col;

        if new_row < 0 || new_col < 0 || new_row >= num_rows || new_col >= num_cols {
            continue;
        }

        let cell = grid[new_row as usize][new_col as usize];
        if cell {
            cnt += 1;

            if cnt > 3 {
                return false;
            }
        }
    }

    emptied_grid[row as usize][col as usize] = false;
    true
}

fn cnt_cells(paper_grid: &mut Vec<Vec<bool>>) -> i32 {
    let mut cnt = 0;

    let mut emptied_grid = (*paper_grid).clone();

    let num_rows = TryInto::<isize>::try_into(paper_grid.len()).unwrap();
    let num_cols = TryInto::<isize>::try_into(paper_grid[0].len()).unwrap();

    for row in 0..num_rows {
        for col in 0..num_cols {
            if check_cell(&paper_grid, row, col, &mut emptied_grid) {
                cnt += 1;
            }
        }
    }

    *paper_grid = emptied_grid.clone();
    cnt
}

fn main() {
    let mut paper_grid = load_file("../input.txt");
    // let mut paper_grid = load_file("../input.example.txt");

    // println!("{:?}", paper_grid);

    let mut tot = 0;
	let mut iter_cnt = 1;
    while iter_cnt != 0 {
		iter_cnt = cnt_cells(&mut paper_grid);
        tot += iter_cnt;
    }
	
    println!("{:?}", tot);
}
