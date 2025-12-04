use anyhow::Error;
use aoc_common::{load_grid, print_grid};

fn main() -> Result<(), Error> {
    let (grid, _extra) = load_grid(std::io::stdin().lock())?;

    print_grid(&grid);
    Ok(())
}
