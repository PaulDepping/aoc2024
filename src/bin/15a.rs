use std::{
    fmt,
    ops::{Add, Neg, Sub},
};

type BoxedResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Clone, Copy, Debug)]
enum MapTile {
    Empty,
    Robot,
    Box,
    Wall,
}

#[derive(Debug)]
struct MissingRepresentationError;

impl fmt::Display for MissingRepresentationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "missing representation".fmt(f)
    }
}

impl std::error::Error for MissingRepresentationError {}

impl TryFrom<char> for MapTile {
    type Error = MissingRepresentationError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(MapTile::Empty),
            '#' => Ok(MapTile::Wall),
            '@' => Ok(MapTile::Robot),
            'O' => Ok(MapTile::Box),
            _ => Err(MissingRepresentationError),
        }
    }
}

impl From<MapTile> for char {
    fn from(value: MapTile) -> Self {
        match value {
            MapTile::Empty => '.',
            MapTile::Robot => '@',
            MapTile::Box => 'O',
            MapTile::Wall => '#',
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl TryFrom<char> for Direction {
    type Error = MissingRepresentationError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(Direction::Up),
            'v' => Ok(Direction::Down),
            '>' => Ok(Direction::Right),
            '<' => Ok(Direction::Left),
            _ => Err(MissingRepresentationError),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Vec2D {
    x: isize,
    y: isize,
}

impl Vec2D {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
    fn to_index(self) -> (usize, usize) {
        (self.y as _, self.x as _)
    }
}

impl Add for Vec2D {
    type Output = Vec2D;

    fn add(self, rhs: Vec2D) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Vec2D {
    type Output = Vec2D;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Neg for Vec2D {
    type Output = Vec2D;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y)
    }
}

impl From<Direction> for Vec2D {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => Vec2D { x: 0, y: -1 },
            Direction::Right => Vec2D { x: 1, y: 0 },
            Direction::Down => Vec2D { x: 0, y: 1 },
            Direction::Left => Vec2D { x: -1, y: 0 },
        }
    }
}

struct Labyrinth {
    map: array2d::Array2D<MapTile>,
    robot_position: Vec2D,
}

impl Labyrinth {
    fn from_map(input_map: &str) -> BoxedResult<Self> {
        let res: Result<Vec<Vec<MapTile>>, MissingRepresentationError> = input_map
            .lines()
            .map(|line| line.chars().map(|c| MapTile::try_from(c)).collect())
            .collect();

        let map = array2d::Array2D::from_rows(&res?)?;

        let robot_position =
            find_robot_position(&map).ok_or_else(|| std::io::Error::other("no robot found!"))?;
        Ok(Labyrinth {
            map,
            robot_position,
        })
    }
}

fn find_robot_position(map: &array2d::Array2D<MapTile>) -> Option<Vec2D> {
    for ((row, col), el) in map.enumerate_row_major() {
        if matches!(el, MapTile::Robot) {
            return Some(Vec2D::new(col as _, row as _));
        }
    }
    None
}

fn solve(input_map: &str, input_seq: &str) -> BoxedResult<isize> {
    let mut labyrinth = Labyrinth::from_map(input_map)?;
    let input_iter = parse_inputs(input_seq);
    for direction in input_iter {
        let base_pos = labyrinth.robot_position;
        let dir_vec: Vec2D = direction.into();
        assert!(matches!(labyrinth.map[base_pos.to_index()], MapTile::Robot));
        if move_group(&mut labyrinth, base_pos, dir_vec) {
            // moved robot!
            assert!(matches!(labyrinth.map[base_pos.to_index()], MapTile::Empty));

            labyrinth.robot_position = labyrinth.robot_position + dir_vec;
            assert!(matches!(
                labyrinth.map[labyrinth.robot_position.to_index()],
                MapTile::Robot
            ));
        }
    }

    Ok(get_result_sum(&labyrinth))
}

fn move_group(labyrinth: &mut Labyrinth, base_pos: Vec2D, dir_vec: Vec2D) -> bool {
    let el = labyrinth.map[base_pos.to_index()];

    match el {
        MapTile::Empty => true,
        MapTile::Wall => false,
        MapTile::Box | MapTile::Robot => {
            let next_idx = base_pos + dir_vec;
            if move_group(labyrinth, next_idx, dir_vec) {
                assert!(matches!(labyrinth.map[next_idx.to_index()], MapTile::Empty));
                labyrinth.map[next_idx.to_index()] = labyrinth.map[base_pos.to_index()];
                labyrinth.map[base_pos.to_index()] = MapTile::Empty;
                true
            } else {
                false
            }
        }
    }
}

fn get_result_sum(labyrinth: &Labyrinth) -> isize {
    labyrinth
        .map
        .enumerate_row_major()
        .filter_map(|((row, col), el)| {
            if matches!(el, MapTile::Box) {
                Some(row as isize * 100 + col as isize)
            } else {
                None
            }
        })
        .sum()
}

fn parse_inputs(
    input_seq: &str,
) -> std::iter::FilterMap<std::str::Chars<'_>, impl FnMut(char) -> Option<Direction>> {
    input_seq
        .chars()
        .filter_map(|c| Direction::try_from(c).ok())
}

const INPUT_MAP: &str = include_str!("../input/15_map.txt");
const INPUT_SEQ: &str = include_str!("../input/15_seq.txt");

fn main() -> BoxedResult<()> {
    let res = solve(INPUT_MAP, INPUT_SEQ)?;

    println!("Result: {}", res);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn big_example() -> BoxedResult<()> {
        const INPUT_MAP: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########\n";
        const INPUT_SEQ: &str =
            "<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^\n";

        let res = solve(INPUT_MAP, INPUT_SEQ)?;

        assert_eq!(res, 10092);

        Ok(())
    }
    #[test]
    fn small_example() -> BoxedResult<()> {
        const INPUT_MAP: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########\n";
        const INPUT_SEQ: &str = "<^^>>>vv<v>>v<<\n";

        let res = solve(INPUT_MAP, INPUT_SEQ)?;

        assert_eq!(res, 2028);

        Ok(())
    }
}
