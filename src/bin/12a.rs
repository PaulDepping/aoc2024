use std::collections::HashSet;

use array2d::Array2D;

const INPUT: &str = include_str!("../input/12.txt");

type BoxedResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct MapTile(u8);

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Position {
    row: usize,
    column: usize,
}

impl Position {
    fn neighbours(self, row_max: usize, column_max: usize) -> [Option<Position>; 4] {
        let row_decr = self.row.checked_sub(1);
        let column_decr = self.column.checked_sub(1);
        let row_incr = {
            let new_row = self.row + 1;
            if new_row >= row_max {
                None
            } else {
                Some(new_row)
            }
        };
        let column_incr = {
            let new_column = self.column + 1;
            if new_column >= column_max {
                None
            } else {
                Some(new_column)
            }
        };
        let l1 = if let Some(row) = row_decr {
            Some(Position {
                row,
                column: self.column,
            })
        } else {
            None
        };
        let l2 = if let Some(row) = row_incr {
            Some(Position {
                row,
                column: self.column,
            })
        } else {
            None
        };
        let l3 = if let Some(column) = column_decr {
            Some(Position {
                row: self.row,
                column,
            })
        } else {
            None
        };
        let l4 = if let Some(column) = column_incr {
            Some(Position {
                row: self.row,
                column,
            })
        } else {
            None
        };
        [l1, l2, l3, l4]
    }
}

fn get_input(input_str: &str) -> Result<Array2D<MapTile>, array2d::Error> {
    let raw_input: Vec<_> = input_str
        .lines()
        .map(|line| line.bytes().map(|byte| MapTile(byte)).collect())
        .collect();
    Array2D::from_rows(&raw_input)
}

fn get_regions(input: &Array2D<MapTile>) -> Vec<HashSet<Position>> {
    let mut regions = Vec::new();
    let mut seen = HashSet::new();

    for ((row, column), &element) in input.enumerate_row_major() {
        let pos = Position { row, column };

        if seen.contains(&pos) {
            continue;
        }

        let mut to_check = vec![pos];
        let mut region = HashSet::new();
        while let Some(val) = to_check.pop() {
            region.insert(val);
            seen.insert(val);

            for n in val.neighbours(input.num_rows(), input.num_columns()) {
                let Some(n) = n else {
                    continue;
                };
                if seen.contains(&n) {
                    continue;
                }
                if element != *input.get(n.row, n.column).unwrap() {
                    continue;
                }
                to_check.push(n);
            }
        }
        regions.push(region);
    }
    regions
}

fn main() -> BoxedResult<()> {
    let input = get_input(INPUT)?;

    let regions = get_regions(&input);

    let result = regions
        .into_iter()
        .map(|region| {
            // for region in regions {
            let area = region.len();

            let mut fence = 0;
            for &el in &region {
                for n in el.neighbours(input.num_rows(), input.num_columns()) {
                    let Some(n) = n else {
                        fence += 1;
                        continue;
                    };
                    if !region.contains(&n) {
                        fence += 1;
                    }
                }
            }
            area * fence
        })
        .fold(0, |l, r| l + r);

    println!("total cost: {}", result);
    Ok(())
}
