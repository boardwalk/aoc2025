use anyhow::{Error, bail};
use std::collections::HashSet;

const IS_PART_TWO: bool = true;

type IdRange = std::ops::RangeInclusive<u64>;

fn is_fresh(ingredient: u64, fresh: &Vec<IdRange>) -> bool {
    for fresh_range in fresh {
        if fresh_range.contains(&ingredient) {
            return true;
        }
    }

    false
}

fn merge_all_ranges(mut ranges: Vec<IdRange>) -> Vec<IdRange> {
    if ranges.is_empty() {
        return Vec::new();
    }

    // Sort by starting value
    ranges.sort_by_key(|r| *r.start());

    let mut out: Vec<IdRange> = Vec::new();

    // Start with the first range
    let mut current = ranges[0].clone();

    for r in ranges.into_iter().skip(1) {
        let (c_start, c_end) = (*current.start(), *current.end());
        let (r_start, r_end) = (*r.start(), *r.end());

        // Check touching-or-overlapping, inclusive ranges:
        // max(start) <= min(end) + 1
        let min_end = c_end.min(r_end);

        let touching_or_overlapping = match min_end.checked_add(1) {
            Some(min_end_plus1) => c_start.max(r_start) <= min_end_plus1,
            None => {
                // min_end == u64::MAX — everything touches it
                true
            }
        };

        if touching_or_overlapping {
            // Merge
            let new_start = c_start.min(r_start);
            let new_end = c_end.max(r_end);
            current = new_start..=new_end;
        } else {
            // Flush previous and start new
            out.push(current);
            current = r;
        }
    }

    // Push the final accumulated range
    out.push(current);

    out
}

fn count_ids(ranges: &Vec<IdRange>) -> u64 {
    let mut num_ids = 0;

    for r in ranges {
        num_ids += r.end() - r.start() + 1;
    }

    num_ids
}

fn main() -> Result<(), Error> {
    let mut fresh: Vec<IdRange> = Vec::new();
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

    if IS_PART_TWO {
        println!("fresh (before) = {fresh:?}");

        fresh = merge_all_ranges(fresh.clone());

        let num_ids = count_ids(&fresh);

        println!("num_ids = {num_ids}");

        println!("fresh (after) = {fresh:?}");
    } else {
        let mut num_fresh = 0;

        for all in &all {
            if is_fresh(*all, &fresh) {
                num_fresh += 1;
            }
        }

        println!("fresh = {fresh:?}");
        println!("all = {all:?}");
        println!("num_fresh = {num_fresh}");
    }

    Ok(())
}
