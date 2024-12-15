type BoxedResult<T> = Result<T, Box<dyn std::error::Error>>;

struct Robot {
    pos_x: isize,
    pos_y: isize,
    velocity_x: isize,
    velocity_y: isize,
}

enum Quadrant {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

fn solve(input: &str, x_max: isize, y_max: isize) -> BoxedResult<isize> {
    let r = regex::Regex::new(r#"(?s)^p=([0-9]+),([0-9]+) v=(-?[0-9]+),(-?[0-9]+)$"#)?;
    let res = input
        .lines()
        .map(|line| -> BoxedResult<Robot> {
            let captures = r
                .captures(line)
                .ok_or_else(|| std::io::Error::other("could not find regex!"))?;
            let (_, [pos_x_str, pos_y_str, velocity_x_str, velocity_y_str]) = captures.extract();

            let pos_x = pos_x_str.parse()?;
            let pos_y = pos_y_str.parse()?;

            let velocity_x = velocity_x_str.parse()?;
            let velocity_y = velocity_y_str.parse()?;

            Ok(Robot {
                pos_x,
                pos_y,
                velocity_x,
                velocity_y,
            })
        })
        .map(|robot| -> BoxedResult<Robot> {
            let mut robot = robot?;
            robot.pos_x += robot.velocity_x * 100;
            robot.pos_y += robot.velocity_y * 100;
            Ok(robot)
        })
        .filter_map(|robot| -> Option<Quadrant> {
            let mut robot = robot.unwrap();
            robot.pos_x = robot.pos_x.rem_euclid(x_max);
            robot.pos_y = robot.pos_y.rem_euclid(y_max);

            let middle_x_min = x_max / 2;
            let middle_y_min = y_max / 2;

            let middle_x_max = (x_max + 1) / 2;
            let middle_y_max = (y_max + 1) / 2;

            if robot.pos_x < middle_x_min && robot.pos_y < middle_y_min {
                return Some(Quadrant::TopLeft);
            }
            if robot.pos_x < middle_x_min && robot.pos_y >= middle_y_max {
                return Some(Quadrant::BottomLeft);
            }
            if robot.pos_x >= middle_x_max && robot.pos_y < middle_y_min {
                return Some(Quadrant::TopRight);
            }
            if robot.pos_x >= middle_x_max && robot.pos_y >= middle_y_max {
                return Some(Quadrant::BottomRight);
            }
            None
        })
        .fold((0, 0, 0, 0), |acc, el| match el {
            Quadrant::TopLeft => (acc.0 + 1, acc.1, acc.2, acc.3),
            Quadrant::TopRight => (acc.0, acc.1 + 1, acc.2, acc.3),
            Quadrant::BottomLeft => (acc.0, acc.1, acc.2 + 1, acc.3),
            Quadrant::BottomRight => (acc.0, acc.1, acc.2, acc.3 + 1),
        });
    Ok(res.0 * res.1 * res.2 * res.3)
}

const INPUT: &str = include_str!("../input/14.txt");

fn main() -> BoxedResult<()> {
    println!("Total Count: {}", solve(INPUT, 101, 103)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn example_solution() -> BoxedResult<()> {
        assert_eq!(solve(EXAMPLE_INPUT, 11, 7)?, 12);
        Ok(())
    }
}
