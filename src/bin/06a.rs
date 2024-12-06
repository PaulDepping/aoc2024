use array2d::Array2D;

#[derive(Clone, Copy, PartialEq, Debug)]
enum MapState {
    NotVisited,
    Visited,
    Wall,
}

#[derive(Debug)]
struct Map(Array2D<MapState>);

#[derive(Clone, Copy)]
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
struct GuardRoute {
    map: Map,
    guard_state: Guard,
}

impl GuardRoute {
    fn walk_one_square(&mut self) -> Option<()> {
        let guard = &self.guard_state;
        let dir = &guard.dir;
        let pos = match dir {
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
        }?;
        let map_detail = self.map.0.get_mut(pos.row, pos.col)?;
        match map_detail {
            MapState::NotVisited => *map_detail = MapState::Visited,
            MapState::Visited => (),
            MapState::Wall => {
                self.guard_state.dir = self.guard_state.dir.next_direction();
                return self.walk_one_square();
            }
        }
        self.guard_state.pos = pos;
        Some(())
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
                    '^' => MapState::Visited,
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

    let mut route = GuardRoute {
        map: Map(map),
        guard_state,
    };

    while let Some(_) = route.walk_one_square() {}

    let total_visited = route
        .map
        .0
        .elements_row_major_iter()
        .filter(|el| matches!(el, MapState::Visited))
        .count();

    println!("total element counted: {}", total_visited);
}

fn find_initial_position(map: &Array2D<MapState>) -> Position {
    for row in 0..map.column_len() {
        for col in 0..map.row_len() {
            if matches!(map.get(row, col).unwrap(), MapState::Visited) {
                return Position { row, col };
            }
        }
    }
    unreachable!()
}
