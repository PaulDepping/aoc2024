use std::error;

const INPUT: &str = include_str!("../input/07.txt");

fn main() -> Result<(), Box<dyn error::Error>> {
    let line_regex = regex::Regex::new(r#"(?m)^(\d+): (.+)$"#)?;
    let res = line_regex
        .captures_iter(INPUT)
        .filter_map(|el| {
            let (_, [sum_str, el_str]) = el.extract();
            let sum: u64 = sum_str.parse().unwrap();
            let elements_sum = el_str.split(' ').map(|s| s.parse::<u64>().unwrap());
            check_sum(sum, 0, elements_sum).then_some(sum)
        })
        .fold(0, |l, r| l + r);

    println!("total valid sequences: {}", res);
    Ok(())
}

fn check_sum(
    total_sum: u64,
    current_sum: u64,
    mut elements_iter: impl Iterator<Item = u64> + Clone,
) -> bool {
    if current_sum > total_sum {
        return false;
    }
    if let Some(next_val) = elements_iter.next() {
        let sum_add = current_sum + next_val;
        let sum_mul = current_sum * next_val;

        let current_state = elements_iter.clone();
        check_sum(total_sum, sum_add, elements_iter) || check_sum(total_sum, sum_mul, current_state)
    } else {
        total_sum == current_sum
    }
}
