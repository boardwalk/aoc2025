use anyhow::Error;

fn main() -> Result<(), Error> {
    let (grid, _extra) = aoc_common::load_grid(std::io::stdin().lock())?;

    aoc_common::print_grid(&grid);
    Ok(())
}
