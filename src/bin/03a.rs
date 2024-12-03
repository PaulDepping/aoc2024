use std::error;

const INPUT: &str = include_str!("../input/03.txt");

type BasicResult<T> = Result<T, Box<dyn error::Error>>;

fn main() -> BasicResult<()> {
    let regex = regex::Regex::new(r#"mul\(([0-9]+),([0-9]+)\)"#)?;
    let result = regex
        .captures_iter(INPUT)
        .map(|res| {
            let (_, [l, r]) = res.extract();
            let l: u64 = l.parse()?;
            let r: u64 = r.parse()?;
            BasicResult::<u64>::Ok(l * r)
        })
        .try_fold(0, |acc, el| BasicResult::<u64>::Ok(acc + el?))?;
    println!("Total sum: {}", result);
    Ok(())
}
