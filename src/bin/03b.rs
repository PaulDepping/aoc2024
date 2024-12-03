use std::error;

const INPUT: &str = include_str!("../input/03.txt");

type BasicResult<T> = Result<T, Box<dyn error::Error>>;

// could implement this using one big complex regex, but instead i go for three smaller regexes and a custom struct to contain the alternate possibilitiees

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum MatchType {
    Mul(u64, u64),
    Do,
    Dont,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Match {
    offset: usize,
    regex_match: MatchType,
}

impl Match {
    fn new_mul(offset: usize, l: u64, r: u64) -> Self {
        Self {
            offset,
            regex_match: MatchType::Mul(l, r),
        }
    }

    fn new_do(offset: usize) -> Self {
        Self {
            offset,
            regex_match: MatchType::Do,
        }
    }

    fn new_dont(offset: usize) -> Self {
        Self {
            offset,
            regex_match: MatchType::Dont,
        }
    }
}

fn main() -> BasicResult<()> {
    let regex_mul = regex::Regex::new(r#"mul\(([0-9]+),([0-9]+)\)"#)?;
    let regex_do = regex::Regex::new(r#"do\(\)"#)?;
    let regex_dont = regex::Regex::new(r#"don't\(\)"#)?;

    let regex_mul_iter = regex_mul.captures_iter(INPUT).map(|el| {
        let offset = el
            .get(0)
            .expect("guaranteed to not be none according to documentation")
            .start();
        let (_, [l, r]) = el.extract();
        Match::new_mul(offset, l.parse().unwrap(), r.parse().unwrap())
    });

    let regex_do_iter = regex_do.find_iter(INPUT).map(|el| {
        let offset = el.start();
        Match::new_do(offset)
    });
    let regex_dont_iter = regex_dont.find_iter(INPUT).map(|el| {
        let offset = el.start();
        Match::new_dont(offset)
    });

    let mut matches: Vec<Match> = regex_mul_iter
        .chain(regex_do_iter)
        .chain(regex_dont_iter)
        .collect();

    matches.sort();

    let mut mul_on = true;
    let mut sum = 0;
    for m in matches {
        match m.regex_match {
            MatchType::Mul(l, r) => {
                if mul_on {
                    sum += l * r
                }
            }
            MatchType::Do => mul_on = true,
            MatchType::Dont => mul_on = false,
        };
    }

    println!("result is {}", sum);

    Ok(())
}
