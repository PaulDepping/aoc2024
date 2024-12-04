const INPUT: &str = include_str!("../input/04.txt");

fn main() {
    let mut input = [[0u8; 140]; 140];
    for (i, l) in INPUT.lines().enumerate() {
        for (j, c) in l.chars().enumerate() {
            input[i][j] = match c {
                'X' => b'X',
                'M' => b'M',
                'A' => b'A',
                'S' => b'S',
                _ => unreachable!(),
            };
        }
    }

    // remove mut
    let input = input;

    let mut total_count = 0;
    for line in input {
        for i in 0..140 - 3 {
            if line[i] == b'X' && line[i + 1] == b'M' && line[i + 2] == b'A' && line[i + 3] == b'S'
            {
                total_count += 1;
            }
        }
    }

    for line in input {
        for i in 0..line.len() - 3 {
            if line[i] == b'S' && line[i + 1] == b'A' && line[i + 2] == b'M' && line[i + 3] == b'X'
            {
                total_count += 1;
            }
        }
    }

    for i in 0..140 - 3 {
        for j in 0..140 {
            if input[i][j] == b'X'
                && input[i + 1][j] == b'M'
                && input[i + 2][j] == b'A'
                && input[i + 3][j] == b'S'
            {
                total_count += 1;
            }
        }
    }

    for i in 0..140 - 3 {
        for j in 0..140 {
            if input[i][j] == b'S'
                && input[i + 1][j] == b'A'
                && input[i + 2][j] == b'M'
                && input[i + 3][j] == b'X'
            {
                total_count += 1;
            }
        }
    }

    for i in 0..140 - 3 {
        for j in 0..140 - 3 {
            if input[i][j] == b'X'
                && input[i + 1][j + 1] == b'M'
                && input[i + 2][j + 2] == b'A'
                && input[i + 3][j + 3] == b'S'
            {
                total_count += 1;
            }
        }
    }

    for i in 0..140 - 3 {
        for j in 0..140 - 3 {
            if input[i][j] == b'S'
                && input[i + 1][j + 1] == b'A'
                && input[i + 2][j + 2] == b'M'
                && input[i + 3][j + 3] == b'X'
            {
                total_count += 1;
            }
        }
    }

    for i in 0..140 - 3 {
        for j in 0..140 - 3 {
            if input[i + 3][j] == b'X'
                && input[i + 2][j + 1] == b'M'
                && input[i + 1][j + 2] == b'A'
                && input[i][j + 3] == b'S'
            {
                total_count += 1;
            }
        }
    }

    for i in 0..140 - 3 {
        for j in 0..140 - 3 {
            if input[i + 3][j] == b'S'
                && input[i + 2][j + 1] == b'A'
                && input[i + 1][j + 2] == b'M'
                && input[i][j + 3] == b'X'
            {
                total_count += 1;
            }
        }
    }

    println!("Result: {}", total_count);
}
