use std::{error, fmt::Write, mem, num::ParseIntError};

const INPUT: &str = include_str!("../input/11.txt");

fn get_input() -> Result<Vec<u64>, ParseIntError> {
    INPUT
        .split_whitespace()
        .map(|s| -> Result<u64, _> { s.parse() })
        .collect()
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut input = get_input()?;
    let mut output = Vec::new();
    let mut map_string = String::new();
    for _ in 0..25 {
        for &el in &input {
            if el == 0 {
                output.push(1);
                continue;
            }

            map_string.clear();
            write!(map_string, "{}", el)?;
            if map_string.len() % 2 == 0 {
                let (l, r) = map_string.split_at(map_string.len() / 2);
                output.push(l.parse()?);
                output.push(r.parse()?);
                continue;
            }

            output.push(el * 2024);
        }
        input.clear();
        mem::swap(&mut input, &mut output);
    }
    println!("Total len: {}", input.len());
    Ok(())
}
