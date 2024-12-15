use std::{fmt::Write, io::stdin};

use array2d::Array2D;

type BoxedResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Clone, Copy)]
struct Robot {
    pos_x: isize,
    pos_y: isize,
    velocity_x: isize,
    velocity_y: isize,
}

struct Map {
    robots: Vec<Robot>,
    x_max: isize,
    y_max: isize,
}

impl Map {
    fn new(robots: Vec<Robot>, x_max: isize, y_max: isize) -> Self {
        Self {
            robots,
            x_max,
            y_max,
        }
    }
}

#[derive(Clone, Copy)]
enum MapField {
    Empty,
    Robot,
}

impl MapField {
    fn to_char(self) -> char {
        match self {
            MapField::Empty => ' ',
            MapField::Robot => '#',
        }
    }
}

struct MapDisplay(array2d::Array2D<MapField>);

impl std::fmt::Display for MapDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut write_newline = false;
        for row in self.0.rows_iter() {
            if write_newline {
                f.write_char('\n')?
            } else {
                write_newline = true;
            };
            for el in row {
                f.write_char(el.to_char())?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl Iterator for &mut Map {
    type Item = MapDisplay;

    fn next(&mut self) -> Option<Self::Item> {
        for r in &mut self.robots {
            r.pos_x = (r.pos_x + r.velocity_x).rem_euclid(self.x_max);
            r.pos_y = (r.pos_y + r.velocity_y).rem_euclid(self.y_max);
        }

        let mut output = Array2D::filled_with(MapField::Empty, self.y_max as _, self.x_max as _);

        for &r in &self.robots {
            output[(r.pos_y as _, r.pos_x as _)] = MapField::Robot;
        }

        Some(MapDisplay(output))
    }
}

fn solve(input: &str, x_max: isize, y_max: isize) -> BoxedResult<()> {
    let r = regex::Regex::new(r#"(?s)^p=([0-9]+),([0-9]+) v=(-?[0-9]+),(-?[0-9]+)$"#)?;
    let res: BoxedResult<Vec<Robot>> = input
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
        .collect();
    let mut map = Map::new(res?, x_max, y_max);

    let mut user_input = String::new();
    let robot_len = map.robots.len();
    for (i, iteration) in (&mut map).enumerate() {
        if iteration
            .0
            .elements_row_major_iter()
            .filter(|el| matches!(el, MapField::Robot))
            .count()
            != robot_len
        {
            continue;
        }
        println!("Iteration Nr. {}", i + 1);
        println!("{}", iteration);
        stdin().read_line(&mut user_input)?;
        user_input.clear();
    }
    Ok(())
}

const INPUT: &str = include_str!("../input/14.txt");

fn main() -> BoxedResult<()> {
    solve(INPUT, 101, 103)?;
    Ok(())
}
