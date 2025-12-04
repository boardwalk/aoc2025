use anyhow::Error;

fn main() -> Result<(), Error> {
    let (grid, _extra) = aoc_common::load_grid(std::io::stdin().lock())?;

    aoc_common::print_grid(&grid);

    // for a cell in the grid, this is the column of the first battery to pick
    let mut _bat1_best_col: ndarray::Array2<Option<usize>> =
        ndarray::Array2::default(grid.raw_dim());

    // for a cell in the grid, this is the column of the second battery to pick
    let mut _bat2_best_col: ndarray::Array2<Option<usize>> =
        ndarray::Array2::default(grid.raw_dim());

    Ok(())
}
