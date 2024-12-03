use std::{error::Error, iter::zip};

const INPUT: &'static str = include_str!("../input/01.txt");

fn main() -> Result<(), Box<dyn Error>> {
    // let input = read_to_string("src/input/01.txt")?; // alternate way of solving
    let mut a = Vec::new();
    let mut b = Vec::new();
    for l in INPUT.lines() {
        for (i, s) in l.split_whitespace().enumerate() {
            let el: u64 = s.parse()?;
            match i {
                0 => a.push(el),
                1 => b.push(el),
                _ => unreachable!(),
            }
        }
    }

    assert!(a.len() == b.len());

    a.sort();
    b.sort();

    let mut sum = 0;
    for (a_el, b_el) in zip(a, b) {
        let dif = a_el.abs_diff(b_el);
        sum += dif;
    }

    println!("The total difference is {}.", sum);

    Ok(())
}
