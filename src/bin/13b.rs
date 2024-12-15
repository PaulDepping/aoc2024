use std::error;

type BoxedResult<T> = Result<T, Box<dyn error::Error>>;

#[derive(Clone, Copy)]
struct EuclideanSolution {
    a: i64,
    b: i64,
    gcd: i64,
    a_coefficient: i64,
    b_coefficient: i64,
    a_offset: i64,
    b_offset: i64,
}

impl EuclideanSolution {
    fn new(a: i64, b: i64) -> Self {
        let mut a_mut = a;
        let mut b_mut = b;
        let mut x0 = 1;
        let mut x1 = 0;
        let mut y0 = 0;
        let mut y1 = 1;
        while b_mut != 0 {
            let q = a_mut.div_euclid(b_mut);
            (a_mut, b_mut) = (b_mut, a_mut.rem_euclid(b_mut));
            (x0, x1) = (x1, x0 - q * x1);
            (y0, y1) = (y1, y0 - q * y1);
        }
        let a_coefficient = x0;
        let b_coefficient = y0;
        let gcd = a_mut;

        assert_eq!(b % gcd, 0);
        assert_eq!(a % gcd, 0);

        let a_offset = b / gcd;
        let b_offset = a / gcd;

        EuclideanSolution {
            a,
            b,
            gcd,
            a_coefficient,
            b_coefficient,
            a_offset,
            b_offset,
        }
    }
}

fn find_solution(
    prize_x: i64,
    prize_y: i64,
    a_x: i64,
    a_y: i64,
    b_x: i64,
    b_y: i64,
) -> Option<i64> {
    if a_x * b_y == a_y * b_x {
        todo!("check most efficient solution on line.");
        /*         if a_x * prize_y != prize_x * a_y {
                   // not on path! prize unreachable.
                   return None;
               }
               let solution = EuclideanSolution::new(a_x, b_x);
               if prize_x % solution.gcd != 0 {
                   // no solutions!
                   return None;
               }
               let prize_mul = prize_x / solution.gcd;
               let mul_by = if a_x < b_x * 3 {
                   // a is more efficient
                   -solution.a_coefficient.div_euclid(solution.a_offset)
               } else {
                   solution.b_coefficient.div_euclid(solution.b_offset)
               };
               let a_count = solution.a_coefficient + mul_by * solution.a_offset;
               let b_count = solution.b_coefficient - mul_by * solution.b_offset;
               if a_count >= 0 && b_count >= 0 {
                   assert_eq!(
                       prize_x,
                       a_count * a_x * prize_mul + b_count * b_x * prize_mul
                   );
                   assert_eq!(
                       prize_y,
                       a_count * a_y * prize_mul + b_count * b_y * prize_mul
                   );
                   return Some(a_count * prize_mul * 3 + b_count * prize_mul);
               }
               return None;
        */
    }

    let a_count_top = prize_x * b_y - prize_y * b_x;
    let a_count_bottom = a_x * b_y - b_x * a_y;

    let a_rem = a_count_top % a_count_bottom;
    if a_rem != 0 {
        // no solution!
        return None;
    }
    let a_count = a_count_top / a_count_bottom;

    let _dbg_a_x = a_count * a_x;
    let _dbg_a_y = a_count * a_y;

    let b_count_top = prize_y * a_x - prize_x * a_y;
    let b_count_bottom = a_x * b_y - b_x * a_y;

    let b_rem = b_count_top % b_count_bottom;
    if b_rem != 0 {
        // no solution!
        return None;
    }
    let b_count = b_count_top / b_count_bottom;

    let _dbg_b_x = b_count * b_x;
    let _dbg_b_y = b_count * b_y;

    let _dbg_x = _dbg_a_x + _dbg_b_x;
    let _dbg_y = _dbg_a_y + _dbg_b_y;

    assert_eq!(prize_x, a_count * a_x + b_count * b_x);
    assert_eq!(prize_y, a_count * a_y + b_count * b_y);

    Some(a_count * 3 + b_count)
}

fn solution(input: &str, prize_offset: i64) -> BoxedResult<i64> {
    let regex = regex::Regex::new(
        "(?m)^Button A: X\\+(\\d+), Y\\+(\\d+)\nButton B: X\\+(\\d+), Y\\+(\\d+)\nPrize: X=(\\d+), Y=(\\d+)$"
    )?;

    let res = regex
        .captures_iter(input)
        .filter_map(|cap| -> Option<i64> {
            let (_, [a_x_str, a_y_str, b_x_str, b_y_str, prize_x_str, prize_y_str]) = cap.extract();

            // let a_cost: i64 = 3;
            let a_x: i64 = a_x_str.parse().unwrap();
            let a_y: i64 = a_y_str.parse().unwrap();

            // let b_cost: i64 = 1;
            let b_x: i64 = b_x_str.parse().unwrap();
            let b_y: i64 = b_y_str.parse().unwrap();
            let prize_x = prize_x_str.parse::<i64>().unwrap() + prize_offset;
            let prize_y = prize_y_str.parse::<i64>().unwrap() + prize_offset;

            find_solution(prize_x, prize_y, a_x, a_y, b_x, b_y)

            // not yet guaranteed but likely? i think?
            // can probably be done!
        })
        .fold(0, |l, r| l + r);

    Ok(res)
}

const INPUT: &str = include_str!("../input/13.txt");

fn main() {
    let res = solution(INPUT, 10000000000000).expect("failed to compute solution");
    println!("Total cost: {}", res);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Button A: X+94, Y+34
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
Prize: X=18641, Y=10279";

    #[test]
    fn example_input() {
        assert_eq!(solution(TEST_INPUT, 0).unwrap(), 480)
    }

    fn amount_of_steps(initial: i64, step: i64) -> i64 {
        -initial.div_euclid(step)
    }

    #[test]
    fn step_calculation() {
        assert_eq!(amount_of_steps(-8, 15), 1);
        assert_eq!(amount_of_steps(-8, 8), 1);
        assert_eq!(amount_of_steps(-8, 7), 2);
        assert_eq!(amount_of_steps(8, 15), 0);
        assert_eq!(amount_of_steps(15, 15), -1);
        assert_eq!(amount_of_steps(0, 15), 0);
        assert_eq!(amount_of_steps(151, 15), -10);
    }
}
