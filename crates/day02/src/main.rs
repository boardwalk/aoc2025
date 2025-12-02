use anyhow::{Error, anyhow};
use std::ops::RangeInclusive;

// search in an range of ids for invalid ids, appending them onto `out`
fn find_invalid_ids(ids: RangeInclusive<i64>, out: &mut Vec<i64>) {
    for id in ids {
        let id_str = id.to_string();
        if id_str.len() % 2 != 0 {
            // odd number of digits
            continue;
        }

        let (left, right) = id_str.split_at(id_str.len() / 2);

        if left == right {
            out.push(id);
        }

        // println!("{id}");
    }
}

fn main() -> Result<(), Error> {
    let mut invalid_ids = Vec::new();

    for line in std::io::stdin().lines() {
        let line = line?;

        for range in line.split(',') {
            let sep = range
                .find('-')
                .ok_or_else(|| anyhow!("no separator in range"))?;

            let first = i64::from_str_radix(&range[0..sep], 10)?;
            let last = i64::from_str_radix(&range[sep + 1..], 10)?;

            find_invalid_ids(first..=last, &mut invalid_ids);
        }

        println!("{line}");
    }

    // println!("{invalid_ids:?}");

    let tot: i64 = invalid_ids.iter().sum();
    println!("tot = {tot}");

    Ok(())
}
