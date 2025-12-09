use anyhow::Error;
use aoc_common::{load_grid, print_grid};

fn rotate_left(row: usize, col: usize) -> (usize, usize) {
    (row, col)
}

fn main() -> Result<(), Error> {
    let (mut grid, _extra) = load_grid(std::io::stdin().lock())?;

    let (num_rows, num_cols) = grid.dim();

    for row in 0..num_rows {
        for col in 0..num_cols {
            let (new_row, new_col) = rotate_left(row, col);

            grid.swap((row, col), (new_row, new_col));
        }
    }

    print_grid(&grid);
    Ok(())
}
