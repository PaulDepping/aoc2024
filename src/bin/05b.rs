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
                    // rule element exists and is not invluded
                    if full_set.contains(rule_el) && !done_elements.contains(rule_el) {
                        sum += reorder_list(&rules, &full_list, &full_set);
                        continue 'line_loop;
                    }
                }
            }
            let _res = done_elements.insert(*el);
            assert!(_res == true);
        }
    }

    println!("Sum: {}", sum);
    Ok(())
}

fn reorder_list(
    rules: &HashMap<u64, HashSet<u64>>,
    full_list: &[u64],
    full_set: &HashSet<u64>,
) -> u64 {
    let mut relevant_rules: HashMap<u64, HashSet<u64>> = HashMap::new();
    for list_el in full_set {
        let list_rule = relevant_rules.entry(*list_el).or_default();
        if let Some(rule) = rules.get(list_el) {
            for rule_el in rule {
                if full_set.contains(rule_el) {
                    let _res = list_rule.insert(*rule_el);
                    assert!(_res);
                }
            }
        }
    }

    let mut correct_output = Vec::with_capacity(full_list.len());
    let mut output_set = HashSet::new();
    let mut leftover_set = full_set.clone();

    loop {
        if leftover_set.len() == 0 {
            break;
        }

        let el = find_next_element(&relevant_rules, &output_set, &leftover_set);
        correct_output.push(el);
        output_set.insert(el);
        leftover_set.remove(&el);
    }

    correct_output[correct_output.len() / 2]
}

fn find_next_element(
    relevant_rules: &HashMap<u64, HashSet<u64>>,
    output_set: &HashSet<u64>,
    leftover_set: &HashSet<u64>,
) -> u64 {
    for el in leftover_set {
        let rule = relevant_rules.get(&el).unwrap();
        if rule.is_subset(output_set) {
            return *el;
        }
    }
    unreachable!()
}
