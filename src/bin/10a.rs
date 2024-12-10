use std::error;

use array2d::Array2D;

const INPUT: &str = include_str!("../input/10.txt");

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct MapTile(u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Position {
    row: usize,
    column: usize,
}

fn get_input() -> Result<Array2D<MapTile>, Box<dyn error::Error>> {
    let list: Result<Vec<Vec<MapTile>>, _> = INPUT
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| -> Result<MapTile, std::io::Error> {
                    Ok(MapTile(
                        c.to_digit(10)
                            .ok_or_else(|| std::io::Error::other("failed to parse input!"))?
                            as _,
                    ))
                })
                .collect()
        })
        .collect();
    Ok(Array2D::from_rows(&list?)?)
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let input = get_input()?;

    //  dbg!(&input);

    let res = input
        .indices_row_major()
        .map(|(row, column)| {
            let start_value = MapTile(0);
            let mut list_of_elements = Vec::new();
            check_entry(&mut list_of_elements, &input, row, column, start_value);
            list_of_elements.sort_unstable();
            list_of_elements.dedup();
            list_of_elements.len()
        })
        .fold(0, |l, r| l + r);

    println!("total score: {}", res);

    Ok(())
}

fn check_entry(
    output: &mut Vec<Position>,
    input: &Array2D<MapTile>,
    row: usize,
    column: usize,
    wanted_value: MapTile,
) {
    let v = input.get(row, column).unwrap();
    if *v != wanted_value {
        return;
    }
    if wanted_value.0 == 9 {
        output.push(Position { row, column });
        return;
    }
    let new_value = MapTile(wanted_value.0 + 1);

    let row_incr = row + 1;
    let column_incr = column + 1;
    let row_decr = row.checked_sub(1);
    let column_decr = column.checked_sub(1);

    if row_incr < input.column_len() {
        check_entry(output, input, row_incr, column, new_value);
    }
    if let Some(row_decr) = row_decr {
        check_entry(output, input, row_decr, column, new_value);
    }
    if column_incr < input.row_len() {
        check_entry(output, input, row, column_incr, new_value);
    }
    if let Some(column_decr) = column_decr {
        check_entry(output, input, row, column_decr, new_value);
    }
}
