use anyhow::{Error, bail};
use std::collections::HashSet;

fn is_fresh(ingredient: u64, fresh: &Vec<std::ops::RangeInclusive<u64>>) -> bool {
    for fresh_range in fresh {
        if fresh_range.contains(&ingredient) {
            return true;
        }
    }

    false
}
fn main() -> Result<(), Error> {
    let mut fresh: Vec<std::ops::RangeInclusive<u64>> = Vec::new();
    let mut all: HashSet<u64> = HashSet::new();
    let mut saw_blank = false;

    for line in std::io::stdin().lines() {
        let line = line?;

        if line.is_empty() {
            saw_blank = true;
            continue;
        }

        if saw_blank {
            let id = u64::from_str_radix(&line, 10)?;

            all.insert(id);
        } else {
            let Some(sep) = line.find('-') else {
                bail!("expected range separator")
            };

            let lo = u64::from_str_radix(&line[0..sep], 10)?;
            let hi = u64::from_str_radix(&line[sep + 1..], 10)?;
            fresh.push(lo..=hi);
        }
    }
    let mut num_fresh = 0;

    for all in &all {
        if is_fresh(*all, &fresh) {
            num_fresh += 1;
        }
    }

    println!("fresh = {fresh:?}");
    println!("all = {all:?}");
    println!("num_fresh = {num_fresh}");

    Ok(())
}
