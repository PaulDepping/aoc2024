use std::{error::Error, i64};

const INPUT: &'static str = include_str!("../input/02.txt");

enum ChangeState {
    NotSet,
    Start(i64),
    Decreasing(i64),
    Increasing(i64),
}

fn is_safe(v: &[i64]) -> bool {
    let mut state = ChangeState::NotSet;
    for (i, val_ref) in v.iter().enumerate() {
        let val = *val_ref;
        if i == 0 {
            state = ChangeState::Start(val);
            continue;
        }
        match state {
            ChangeState::NotSet => unreachable!(),
            ChangeState::Start(last_val) => {
                let dif = last_val - val;
                match dif {
                    // decreasing
                    _x @ -3..=-1 => {
                        state = ChangeState::Decreasing(val);
                    }
                    // increasing
                    _x @ 1..=3 => {
                        state = ChangeState::Increasing(val);
                    }
                    // others
                    _ => {
                        return false;
                    }
                }
            }
            ChangeState::Decreasing(last_val) => {
                let dif = last_val - val;
                if !(-3..=-1).contains(&dif) {
                    return false;
                }
                state = ChangeState::Decreasing(val);
            }
            ChangeState::Increasing(last_val) => {
                let dif = last_val - val;
                if !(1..=3).contains(&dif) {
                    return false;
                }
                state = ChangeState::Increasing(val);
            }
        }
    }
    true
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut safe_count = 0;
    for l in INPUT.lines() {
        let values: Vec<i64> = l
            .split_whitespace()
            .map(|s| s.parse().expect("failed to parse input"))
            .collect();

        if is_safe(&values) {
            safe_count += 1;
            continue;
        }

        // try other permutations
        for i in 0..values.len() {
            let v: Vec<i64> = values
                .iter()
                .enumerate()
                .filter(|(el_num, _)| *el_num != i)
                .map(|(_, b)| *b)
                .collect();
            if is_safe(&v) {
                safe_count += 1;
                break;
            }
        }
    }
    println!("total safe count: {}", safe_count);
    Ok(())
}
