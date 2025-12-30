use anyhow::Error;
use regex::Regex;
use std::collections::VecDeque;

#[derive(Debug)]
struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<usize>,
}

fn parse_machines(input: impl std::io::BufRead) -> Result<Vec<Machine>, Error> {
    let mut machines = Vec::new();

    let re = Regex::new(r"^\[(.*?)\](.*)\{([^}]+)\}$").unwrap();

    for line in input.lines() {
        let line = line?;
        if line.is_empty() {
            continue;
        }

        let caps = re
            .captures(&line)
            .ok_or_else(|| Error::msg("line does not match expected pattern"))?;

        let lights_section = caps.get(1).unwrap().as_str();
        let buttons_section = caps.get(2).unwrap().as_str();
        let jolt_section = caps.get(3).unwrap().as_str();
        let lights: Vec<bool> = lights_section.chars().map(|c| c == '#').collect();

        let wiring_re = Regex::new(r"\(([^)]+)\)").unwrap();
        let mut buttons = Vec::new();

        for m in wiring_re.find_iter(buttons_section) {
            let inner = m.as_str();
            let light_indices: Vec<usize> = inner[1..inner.len() - 1]
                .split(',')
                .map(|s| {
                    s.trim()
                        .parse()
                        .map_err(|_| Error::msg("failed to parse light index"))
                })
                .collect::<Result<_, _>>()?;

            // Ensure all indices are within the range of the lights vector
            for idx in &light_indices {
                if *idx >= lights.len() {
                    return Err(Error::msg("light index out of range"));
                }
            }

            buttons.push(light_indices);
        }

        let joltages: Vec<usize> = jolt_section
            .split(',')
            .map(|s| {
                s.trim()
                    .parse()
                    .map_err(|_| Error::msg("failed to parse joltage"))
            })
            .collect::<Result<_, _>>()?;

        machines.push(Machine {
            lights,
            buttons,
            joltages,
        });
    }

    Ok(machines)
}

fn min_button_pushes(machine: &Machine) -> Result<usize, Error> {
    // Assume all lights start turned off and "machine.lights" is the desired final state of lights
    // Figure out the minimum number of buttons to push to reach the final state of lights.
    // Each button push toggles the lights in machine.buttons.
    let n = machine.lights.len();
    // Compute target bitmask
    let target = machine
        .lights
        .iter()
        .enumerate()
        .map(|(i, &on)| if on { 1 << i } else { 0 })
        .sum::<usize>();

    // Precompute toggle masks for each button
    let masks: Vec<usize> = machine
        .buttons
        .iter()
        .map(|button| {
            let mut mask = 0usize;
            for &idx in button {
                mask |= 1 << idx;
            }
            mask
        })
        .collect();

    // BFS over states
    let mut visited = vec![false; 1 << n];
    let mut queue = VecDeque::new();
    queue.push_back((0usize, 0usize)); // (state, steps)
    visited[0] = true;

    while let Some((state, steps)) = queue.pop_front() {
        if state == target {
            return Ok(steps);
        }
        for &mask in &masks {
            let new_state = state ^ mask;
            if !visited[new_state] {
                visited[new_state] = true;
                queue.push_back((new_state, steps + 1));
            }
        }
    }

    // If unreachable (should not happen per problem constraints)
    Err(Error::msg("No solution found"))
}

fn main() -> Result<(), Error> {
    let machines = parse_machines(std::io::stdin().lock())?;
    println!("{machines:?}");
    let mut result = 0;

    for machine in &machines {
        result += min_button_pushes(machine)?;
    }

    println!("result = {result}");
    Ok(())
}
