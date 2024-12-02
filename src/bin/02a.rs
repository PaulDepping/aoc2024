use std::{error::Error, i64};

const INPUT: &'static str = include_str!("../input/02.txt");

enum ChangeState {
    NotSet,
    Start(i64),
    Decreasing(i64),
    Increasing(i64),
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut safe_count = 0;
    for l in INPUT.lines() {
        let mut state = ChangeState::NotSet;
        let mut is_safe = true;
        for (i, s) in l.split_whitespace().enumerate() {
            let val: i64 = s.parse()?;
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
                            is_safe = false;
                            break;
                        }
                    }
                }
                ChangeState::Decreasing(last_val) => {
                    let dif = last_val - val;
                    if !(-3..=-1).contains(&dif) {
                        is_safe = false;
                        break;
                    }
                    state = ChangeState::Decreasing(val);
                }
                ChangeState::Increasing(last_val) => {
                    let dif = last_val - val;
                    if !(1..=3).contains(&dif) {
                        is_safe = false;
                        break;
                    }
                    state = ChangeState::Increasing(val);
                }
            }
        }
        if is_safe {
            safe_count += 1;
        }
    }
    println!("total safe count: {}", safe_count);
    Ok(())
}
