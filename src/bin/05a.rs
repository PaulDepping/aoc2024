use std::{
    collections::{HashMap, HashSet},
    error,
};

type BasicResult<T> = Result<T, Box<dyn error::Error>>;

const INPUT: &str = include_str!("../input/05.txt");

fn main() -> BasicResult<()> {
    let mut itr = INPUT.lines();
    let mut rules: HashMap<u64, HashSet<u64>> = HashMap::new();
    while let Some(line) = itr.next() {
        if line.is_empty() {
            break;
        }
        let mut before = 0;
        let mut after = 0;
        for (i, el) in line.split('|').enumerate() {
            match i {
                0 => before = el.parse()?,
                1 => after = el.parse()?,
                _ => unreachable!(),
            }
        }
        assert!(before != 0 && after != 0);
        let _res = rules.entry(after).or_default().insert(before);
        assert!(_res == true);
    }

    let mut sum = 0;
    'line_loop: while let Some(line) = itr.next() {
        let mut done_elements = HashSet::new();
        let full_list: Vec<u64> = line.split(',').map(|s| s.parse().unwrap()).collect();
        let full_set: HashSet<u64> = HashSet::from_iter(full_list.iter().map(|r| *r));
        for el in &full_list {
            if let Some(rule) = rules.get(el) {
                for rule_el in rule {
                    if full_set.contains(rule_el) && !done_elements.contains(rule_el) {
                        continue 'line_loop;
                    }
                }
            }
            let _res = done_elements.insert(*el);
            assert!(_res == true);
        }
        sum += full_list[full_list.len() / 2];
    }

    println!("Sum: {}", sum);
    Ok(())
}
