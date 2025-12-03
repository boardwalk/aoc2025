use anyhow::{Error, anyhow};
use std::collections::BTreeSet;

const IS_PART_TWO: bool = true;

fn is_invalid_id(id: i64) -> bool {
    let id_str = id.to_string();

    if IS_PART_TWO {
        let mut sequence_len = 1;
        while sequence_len <= id_str.len() {
            if id_str.len() % sequence_len != 0 {
                // id string doesn't chunk evenly at this sequence_len
                sequence_len += 1;
                continue;
            }

            let mut begin: usize = 0;
            let mut all_match = true;
            let mut num_matches: usize = 0;

            while begin + sequence_len <= id_str.len() {
                let left = &id_str[0..sequence_len];
                let right = &id_str[begin..begin + sequence_len];

                if left != right {
                    all_match = false;
                    break;
                }
                num_matches += 1;

                begin += sequence_len;
            }

            if all_match && num_matches >= 2 {
                return true;
            }

            sequence_len += 1;
        }

        false
    } else {
        if id_str.len() % 2 != 0 {
            // odd number of digits
            return false;
        }

        let (left, right) = id_str.split_at(id_str.len() / 2);

        left == right
    }
}

fn main() -> Result<(), Error> {
    let mut invalid_ids = BTreeSet::new();

    for line in std::io::stdin().lines() {
        let line = line?;

        for range in line.split(',') {
            let sep = range
                .find('-')
                .ok_or_else(|| anyhow!("no separator in range"))?;

            let first = i64::from_str_radix(&range[0..sep], 10)?;
            let last = i64::from_str_radix(&range[sep + 1..], 10)?;

            for id in first..=last {
                if is_invalid_id(id) {
                    invalid_ids.insert(id);
                }
            }
        }
    }

    println!("{invalid_ids:?}");

    let tot_invalid: i64 = invalid_ids.iter().sum();

    println!("tot_invalid = {tot_invalid}");

    Ok(())
}
