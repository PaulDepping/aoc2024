use std::error;

type BoxedResult<T> = Result<T, Box<dyn error::Error>>;

fn solution(input: &str) -> BoxedResult<u64> {
    let regex = regex::Regex::new(
        "(?m)^Button A: X\\+(\\d+), Y\\+(\\d+)\nButton B: X\\+(\\d+), Y\\+(\\d+)\nPrize: X=(\\d+), Y=(\\d+)$"
    )?;

    let res = regex
        .captures_iter(input)
        .filter_map(|cap| -> Option<u64> {
            let (_, [a_x_str, a_y_str, b_x_str, b_y_str, prize_x_str, prize_y_str]) = cap.extract();

            let a_x: u64 = a_x_str.parse().unwrap();
            let a_y: u64 = a_y_str.parse().unwrap();
            let b_x: u64 = b_x_str.parse().unwrap();
            let b_y: u64 = b_y_str.parse().unwrap();
            let prize_x = prize_x_str.parse::<u64>().unwrap();
            let prize_y = prize_y_str.parse::<u64>().unwrap();

            for total_cost in 0u64.. {
                let a_min = (total_cost.saturating_sub(97u64)) / 3;
                let a_max = (total_cost / 3).min(100);
                let a_range = a_min..=a_max;
                if a_range.is_empty() {
                    break;
                }
                for a_count in a_range {
                    let b_count = total_cost - (a_count * 3);
                    assert!(
                        b_count <= 100,
                        "Too much b! total_cost={}, a_count={}, b_count={}",
                        total_cost,
                        a_count,
                        b_count
                    );

                    let spot_x = a_count * a_x + b_count * b_x;
                    let spot_y = a_count * a_y + b_count * b_y;
                    if spot_x == prize_x && spot_y == prize_y {
                        return Some(total_cost);
                    }
                }
            }

            None
        })
        .fold(0, |l, r| l + r);

    Ok(res)
}

const INPUT: &str = include_str!("../input/13.txt");

fn main() {
    let res = solution(INPUT).expect("failed to compute solution");
    println!("Total cost: {}", res);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";
    #[test]
    fn check_test() {
        assert_eq!(solution(INPUT).unwrap(), 480);
    }
}
