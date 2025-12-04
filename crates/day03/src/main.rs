use anyhow::{Error, bail};
use std::io::BufRead;
const IS_PART_TWO: bool = true;

fn main() -> Result<(), Error> {
    let m = if IS_PART_TWO { 12 } else { 2 };

    let mut total = 0;

    for line in std::io::stdin().lock().lines() {
        let line = line?;
        let mut digits = Vec::new();

        for ch in line.chars() {
            let digit = match ch {
                '1' => 1,
                '2' => 2,
                '3' => 3,
                '4' => 4,
                '5' => 5,
                '6' => 6,
                '7' => 7,
                '8' => 8,
                '9' => 9,
                _ => bail!("invalid digit"),
            };

            digits.push(digit);
        }

        // println!("digits = {digits:?}");

        let mut k = digits.len() - m;
        let mut stack = Vec::new();
        for digit in digits.iter().copied() {
            while k > 0 && !stack.is_empty() && stack.last().copied().unwrap() < digit {
                stack.pop();
                k -= 1;
            }
            stack.push(digit);
        }

        while stack.len() > m {
            stack.pop();
        }

        // println!("stack = {stack:?}");

        let mut subtotal = 0;

        for digit in stack.iter().copied() {
            let digit = usize::try_from(digit)?;
            subtotal = subtotal * 10 + digit;
        }

        // println!("subtotal = {subtotal}");

        total += subtotal;
    }

    println!("total = {total}");

    Ok(())
}
