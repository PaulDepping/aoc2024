use core::fmt;
use std::fmt::Write;

use array2d::Array2D;

#[derive(Clone, Copy, Debug)]
enum MapState {
    NotVisited,
    Visited(Direction),
    Wall,
}

impl MapState {
    fn to_char(self) -> char {
        match self {
            MapState::NotVisited => '.',
            MapState::Wall => '#',
            MapState::Visited(direction) => match direction {
                Direction::Up => '^',
                Direction::Right => '>',
                Direction::Down => 'v',
                Direction::Left => '<',
            },
        }
    }
}

#[derive(Debug, Clone)]
struct Map(Array2D<MapState>);

impl fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.0.rows_iter() {
            for el in row {
                f.write_char(el.to_char())?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Ord, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Copy)]
struct Position {
    col: usize,
    row: usize,
}

impl Direction {
    fn next_direction(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

#[derive(Clone, Copy)]
struct Guard {
    pos: Position,
    dir: Direction,
}

#[derive(Clone)]
struct GuardRoute {
    map: Map,
    guard_state: Guard,
}

#[derive(Clone)]
enum WalkResult {
    Walking,
    Looped,
    Escaped,
}

impl GuardRoute {
    fn walk_one_square(&mut self) -> WalkResult {
        let guard = &self.guard_state;
        let dir = &guard.dir;
        let Some(pos) = (match dir {
            Direction::Up => {
                let col = guard.pos.col;
                match guard.pos.row.checked_sub(1) {
                    Some(row) => Some(Position { col, row }),
                    None => None,
                }
            }
            Direction::Right => {
                let row = guard.pos.row;
                let col = guard.pos.col + 1;
                Some(Position { col, row })
            }
            Direction::Down => {
                let row = guard.pos.row + 1;
                let col = guard.pos.col;
                Some(Position { col, row })
            }
            Direction::Left => {
                let row = guard.pos.row;
                match guard.pos.col.checked_sub(1) {
                    Some(col) => Some(Position { col, row }),
                    None => None,
                }
            }
        }) else {
            return WalkResult::Escaped;
        };

        let Some(map_detail) = self.map.0.get_mut(pos.row, pos.col) else {
            return WalkResult::Escaped;
        };

        match map_detail {
            MapState::NotVisited => *map_detail = MapState::Visited(self.guard_state.dir),
            MapState::Visited(direction) => {
                if *direction == self.guard_state.dir {
                    return WalkResult::Looped;
                }
            }
            MapState::Wall => {
                self.guard_state.dir = self.guard_state.dir.next_direction();
                return self.walk_one_square();
            }
        }
        self.guard_state.pos = pos;
        WalkResult::Walking
    }
}

const INPUT: &str = include_str!("../input/06.txt");

fn main() {
    let original_inputs: Vec<Vec<MapState>> = INPUT
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => MapState::NotVisited,
                    '#' => MapState::Wall,
                    '^' => MapState::Visited(Direction::Up),
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    let map = Array2D::from_rows(&original_inputs).unwrap();

    let guard_state = Guard {
        pos: find_initial_position(&map),
        dir: Direction::Up,
    };

    let route = GuardRoute {
        map: Map(map),
        guard_state,
    };

    let row_len = route.map.0.column_len();
    let col_len = route.map.0.row_len();
    let mut sum_loops = 0;
    for row in 0..row_len {
        for col in 0..col_len {
            println!("checking {} {}", row, col);
            let mut m = route.clone();
            {
                let el_ref = m.map.0.get_mut(row, col).unwrap();
                match el_ref {
                    MapState::NotVisited => *el_ref = MapState::Wall,
                    _ => continue,
                };
            }
            loop {
                let s = m.walk_one_square();
                match s {
                    WalkResult::Walking => (),
                    WalkResult::Looped => {
                        sum_loops += 1;
                        break;
                    }
                    WalkResult::Escaped => break,
                }
            }
        }
    }

    println!("total blockers possible: {}", sum_loops);
}

fn find_initial_position(map: &Array2D<MapState>) -> Position {
    for row in 0..map.column_len() {
        for col in 0..map.row_len() {
            if matches!(map.get(row, col).unwrap(), MapState::Visited(_)) {
                return Position { row, col };
            }
        }
    }
    unreachable!()
}
