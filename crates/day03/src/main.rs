use anyhow::Error;

fn main() -> Result<(), Error> {
    let (grid, _extra) = aoc_common::load_grid(std::io::stdin().lock())?;

    aoc_common::print_grid(&grid);
    let (num_rows, num_cols) = grid.dim();

    let mut total_val: usize = 0;

    for row in 0..num_rows {
        // try every pair of batteries in this row
        let mut best_val: Option<usize> = None;
        for col1 in 0..num_cols {
            let val1 = grid[(row, col1)];
            for col2 in col1 + 1..num_cols {
                let val2 = grid[(row, col2)];

                let val1 = val1.to_digit(10).unwrap() as usize;

                let val2 = val2.to_digit(10).unwrap() as usize;
                let combined_val = val1 * 10 + val2;

                if let Some(bv) = best_val {
                    if combined_val > bv {
                        best_val = Some(combined_val);
                    }
                } else {
                    best_val = Some(combined_val);
                }
            }
        }

        total_val += best_val.unwrap();
    }

    println!("total_val = {total_val}");

    Ok(())
}
