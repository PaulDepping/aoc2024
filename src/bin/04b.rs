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

    for i in 0..140 - 2 {
        for j in 0..140 - 2 {
            if input[i + 1][j + 1] != b'A' {
                continue;
            }
            if !((input[i][j] == b'M' && input[i + 2][j + 2] == b'S')
                || (input[i][j] == b'S' && input[i + 2][j + 2] == b'M'))
            {
                continue;
            }
            if !((input[i + 2][j] == b'M' && input[i][j + 2] == b'S')
                || (input[i + 2][j] == b'S' && input[i][j + 2] == b'M'))
            {
                continue;
            }
            total_count += 1;
        }
    }

    println!("Total count : {}", total_count);
}
