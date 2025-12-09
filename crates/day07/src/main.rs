use anyhow::{Error, bail};
use aoc_common::{load_grid, print_grid};

fn find_start_pos(grid: &ndarray::Array2<char>) -> Option<(i64, i64)> {
    for ((row, col), val) in grid.indexed_iter() {
        if *val == 'S' {
            let Ok(row) = i64::try_from(row) else {
                return None;
            };

            let Ok(col) = i64::try_from(col) else {
                return None;
            };

            return Some((row, col));
        }
    }

    None
}

fn get_grid(grid: &ndarray::Array2<char>, row: i64, col: i64) -> Option<char> {
    let Ok(row) = usize::try_from(row) else {
        return None;
    };

    let Ok(col) = usize::try_from(col) else {
        return None;
    };

    grid.get((row, col)).copied()
}

fn main() -> Result<(), Error> {
    let (grid, _extra) = load_grid(std::io::stdin().lock())?;

    let mut annot_grid = grid.clone();

    let Some(start_pos) = find_start_pos(&grid) else {
        bail!("could not find start pos");
    };

    print_grid(&grid);

    let mut beams = std::collections::VecDeque::new();
    beams.push_back(start_pos);

    let mut num_splits = 0;
    while let Some((row, col)) = beams.pop_front() {
        println!("beam {}, {}", row, col);

        let Some(value) = get_grid(&grid, row, col) else {
            continue;
        };

        annot_grid[(row as usize, col as usize)] = '|';

        if value == '^' {
            println!("hit splitter");
            beams.push_back((row, col - 1));
            beams.push_back((row, col + 1));
            num_splits += 1;
        } else if value == 'S' || value == '.' {
            println!("normal advance");
            beams.push_back((row + 1, col));
        } else {
            bail!("unknown grid char");
        }
    }

    println!("num_splits = {num_splits}");

    print_grid(&annot_grid);

    Ok(())
}
