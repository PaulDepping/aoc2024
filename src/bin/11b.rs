use std::{collections::HashMap, error, fmt::Write, mem, num::ParseIntError};

const INPUT: &str = include_str!("../input/11.txt");

fn get_input() -> Result<Vec<u64>, ParseIntError> {
    INPUT.split_whitespace().map(|s| s.parse()).collect()
}

type BoxedResult<T> = Result<T, Box<dyn error::Error>>;

fn main() -> BoxedResult<()> {
    let raw_input = get_input()?;

    let mut input = HashMap::new();
    for el in raw_input {
        *input.entry(el).or_insert(0usize) += 1;
    }

    let mut output = HashMap::new();
    let mut s = String::new();

    for _ in 0..75 {
        output.clear();
        for (&num, &count) in &input {
            if num == 0 {
                *output.entry(1).or_insert(0) += count;
                continue;
            }

            s.clear();
            write!(s, "{}", num)?;
            if s.len() % 2 == 0 {
                let (l_str, r_str) = s.split_at(s.len() / 2);
                let l_num = l_str.parse()?;
                let r_num = r_str.parse()?;
                *output.entry(l_num).or_insert(0) += count;
                *output.entry(r_num).or_insert(0) += count;
                continue;
            }

            *output.entry(num * 2024).or_insert(0) += count;
        }
        // output is new input, input is new output
        mem::swap(&mut input, &mut output);
    }

    let res = input
        .into_iter()
        .map(|(_num, count)| count)
        .fold(0, |l, r| l + r);
    println!("Total len: {}", res);
    Ok(())
}
