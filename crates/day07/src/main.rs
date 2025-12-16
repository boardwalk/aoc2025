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

fn get_grid(grid: &ndarray::Array2<char>, (row, col): (i64, i64)) -> Option<char> {
    let Ok(row) = usize::try_from(row) else {
        return None;
    };

    let Ok(col) = usize::try_from(col) else {
        return None;
    };

    grid.get((row, col)).copied()
}

fn is_empty(grid: &ndarray::Array2<char>, (row, col): (i64, i64)) -> bool {
    let Some(value) = get_grid(grid, (row, col)) else {
        return true;
    };

    value == '.'
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

    while let Some((row, col)) = beams.pop_front() {
        annot_grid[(row as usize, col as usize)] = '|';

        let Some(value) = get_grid(&grid, (row, col)) else {
            continue;
        };

        if value == '^' {
            println!("hit splitter");
            let left = (row, col - 1);
            let right = (row, col + 1);

            if is_empty(&grid, left) && is_empty(&grid, right) {
                beams.push_back(left);
                beams.push_back(right);
            }
        } else if value == 'S' || value == '.' {
            let down = (row + 1, col);
            println!("normal advance");
            if get_grid(&grid, down).is_some() {
                beams.push_back(down);
            }
        } else {
            bail!("unknown grid char");
        }

        // print_grid(&annot_grid);

        // std::thread::sleep(std::time::Duration::from_millis(50));
    }

    print_grid(&annot_grid);

    Ok(())
}
