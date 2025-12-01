use anyhow::{Error, bail};
use std::io::{BufRead, BufReader};

const IS_PART_TWO: bool = true;

fn normalize(mut cur_pos: i64, mut val: i64) -> (i64, usize) {
    let mut num_zero = 0;

    while val < 0 {
        if cur_pos == 0 {
            cur_pos = 99;
            val += 1;
        } else {
            cur_pos -= 1;
            val += 1;
            if cur_pos == 0 {
                num_zero += 1;
            }
        }
    }

    while val > 0 {
        if cur_pos == 99 {
            cur_pos = 0;
            val -= 1;
            num_zero += 1;
        } else {
            cur_pos += 1;
            val -= 1;
        }
    }

    (cur_pos, num_zero)
}

fn main() -> Result<(), Error> {
    let rdr = BufReader::new(std::io::stdin());

    let mut cur_pos = 50;
    let mut tot_num_zero = 0;

    for line in rdr.lines() {
        let line = line?;

        let val = if let Some(rest) = line.as_str().strip_prefix("L") {
            let num = i64::from_str_radix(rest, 10)?;
            -num
        } else if let Some(rest) = line.as_str().strip_prefix("R") {
            let num = i64::from_str_radix(rest, 10)?;
            num
        } else {
            bail!("bad line: {line}")
        };

        // println!("{val}");

        let num_zero;

        (cur_pos, num_zero) = normalize(cur_pos, val);

        if IS_PART_TWO {
            tot_num_zero += num_zero;
        } else {
            if cur_pos == 0 {
                tot_num_zero += 1;
            }
        }

        // println!("cur_pos = {cur_pos}");
    }

    println!("tot_num_zero = {tot_num_zero}");

    Ok(())
}
