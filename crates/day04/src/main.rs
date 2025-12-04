use anyhow::Error;
use aoc_common::{load_grid, print_grid};

const IS_PART_TWO: bool = true;

fn get_checked(grid: &ndarray::Array2<char>, row: i64, col: i64) -> Option<char> {
    let Ok(row) = usize::try_from(row) else {
        return None;
    };

    let Ok(col) = usize::try_from(col) else {
        return None;
    };

    grid.get((row, col)).copied()
}

fn count_adjacent(grid: &ndarray::Array2<char>, row: usize, col: usize) -> usize {
    let row = i64::try_from(row).unwrap();
    let col = i64::try_from(col).unwrap();
    let mut num_adjacent = 0;

    let offsets = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for (row_off, col_off) in &offsets {
        let Some(val) = get_checked(grid, row + row_off, col + col_off) else {
            continue;
        };

        if val == '@' {
            num_adjacent += 1;
        }
    }

    num_adjacent
}

fn main() -> Result<(), Error> {
    let (mut grid, _extra) = load_grid(std::io::stdin().lock())?;

    print_grid(&grid);

    if IS_PART_TWO {
        let mut num_removed = 0;

        loop {
            let mut to_remove = Vec::new();
            for ((row, col), val) in grid.indexed_iter() {
                if *val == '@' && count_adjacent(&grid, row, col) < 4 {
                    to_remove.push((row, col));
                }
            }

            if to_remove.is_empty() {
                break;
            }

            for (row, col) in &to_remove {
                grid[(*row, *col)] = '.';
            }

            num_removed += to_remove.len();
        }

        println!("num_removed = {num_removed}");
    } else {
        let mut num_accessible = 0;
        for ((row, col), val) in grid.indexed_iter() {
            if *val == '@' && count_adjacent(&grid, row, col) < 4 {
                num_accessible += 1;
            }
        }

        println!("num_accessible = {num_accessible}");
    }

    Ok(())
}
