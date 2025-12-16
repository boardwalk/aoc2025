use anyhow::{Error, bail};

fn main() -> Result<(), Error> {
    let mut tiles = Vec::new();

    for line in std::io::stdin().lines() {
        let line = line?;
        let Some(sep) = line.find(',') else {
            bail!("missing separateor on line");
        };

        let x = &line[0..sep];
        let y = &line[sep + 1..];
        let x = i64::from_str_radix(x, 10)?;
        let y = i64::from_str_radix(y, 10)?;

        tiles.push((x, y));
    }

    let mut max_area = 0;

    for i in 0..tiles.len() {
        for j in i + 1..tiles.len() {
            let tile_1 = tiles[i];
            let tile_2 = tiles[j];

            let width = tile_1.0.abs_diff(tile_2.0) + 1;
            let height = tile_1.1.abs_diff(tile_2.1) + 1;
            let area = width * height;

            max_area = std::cmp::max(max_area, area);
        }
    }

    println!("max_area = {max_area}");

    Ok(())
}
