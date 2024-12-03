use std::{collections::HashMap, error::Error};

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

    // a.sort();
    // b.sort();

    let mut similarity_list = HashMap::<u64, u64>::new();
    for el in b {
        let entry = similarity_list.entry(el).or_insert(0);
        *entry += 1;
    }

    let mut score = 0;
    for el in a {
        match similarity_list.get(&el) {
            Some(val) => score += el * val,
            None => (),
        };
    }

    println!("The final score is {}.", score);

    Ok(())
}
