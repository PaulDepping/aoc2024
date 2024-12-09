use std::{
    collections::{HashMap, HashSet},
    error,
};

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct AntennaIdentifier(char);

#[derive(Clone, Copy)]
enum MapState {
    Empty,
    Antenna(AntennaIdentifier),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Position {
    row: usize,
    column: usize,
}

const INPUT: &str = include_str!("../input/08.txt");

fn main() -> Result<(), Box<dyn error::Error>> {
    let raw_map: Vec<Vec<MapState>> = INPUT
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    ident @ ('a'..='z' | 'A'..='Z' | '0'..='9') => {
                        MapState::Antenna(AntennaIdentifier(ident))
                    }
                    '.' => MapState::Empty,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    let map = array2d::Array2D::from_rows(&raw_map)?;

    drop(raw_map);

    let mut grouped_antennas: HashMap<AntennaIdentifier, HashSet<Position>> = HashMap::new();

    for ((row, column), el) in map.enumerate_row_major() {
        if let MapState::Antenna(antenna_identifier) = el {
            grouped_antennas
                .entry(*antenna_identifier)
                .or_default()
                .insert(Position { row, column });
        }
    }

    let mut valid_positions: HashSet<Position> = HashSet::new();

    for (_antenna_ident, antenna_set) in grouped_antennas {
        let iter = antenna_set.iter();

        add_positions_for_pairs(
            &mut valid_positions,
            map.column_len().try_into()?,
            map.row_len().try_into()?,
            iter,
        )?;
    }

    println!("total amount of positions: {}", valid_positions.len());

    Ok(())
}

fn add_positions_for_pairs<'a>(
    valid_positions: &mut HashSet<Position>,
    row_max: isize,
    column_max: isize,
    mut iter: impl Iterator<Item = &'a Position> + Clone,
) -> Result<(), Box<dyn error::Error>> {
    let Some(position) = iter.next() else {
        return Ok(());
    };
    let next_iter = iter.clone();

    for other_position in iter {
        let row_dif = other_position.row as isize - position.row as isize;
        let column_dif = other_position.column as isize - position.column as isize;
        {
            let mut row = other_position.row as isize;
            let mut column = other_position.column as isize;
            loop {
                if !(0..row_max).contains(&row) || !(0..column_max).contains(&column) {
                    break;
                }

                valid_positions.insert(Position {
                    row: row.try_into()?,
                    column: column.try_into()?,
                });

                row = row + row_dif;
                column = column + column_dif;
            }
        }

        {
            let mut row = other_position.row as isize;
            let mut column = other_position.column as isize;
            loop {
                row = row - row_dif;
                column = column - column_dif;
                if !(0..row_max).contains(&row) || !(0..column_max).contains(&column) {
                    break;
                }

                valid_positions.insert(Position {
                    row: row.try_into()?,
                    column: column.try_into()?,
                });
            }
        }
    }

    add_positions_for_pairs(valid_positions, row_max, column_max, next_iter)
}
