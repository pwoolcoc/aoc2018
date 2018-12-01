use std::collections::BTreeSet;
use std::fs;

fn find(input: &str) -> Result<i64, Box<std::error::Error>> {
    let mut totals = BTreeSet::new();
    let mut total = 0i64;
    totals.insert(total);
    let numbers = input.lines().flat_map(|line| line.parse::<i64>().ok()).cycle();
    for number in numbers {
        total += number;
        if totals.contains(&total) {
            break
        }
        totals.insert(total);
    }
    Ok(total)
}

fn main() -> Result<(), Box<std::error::Error>> {
    let input = fs::read_to_string("input")?;
    println!("{}", find(&input)?);
    Ok(())
}

#[test]
fn test() {
    assert_eq!(find("+1\n-1\n").unwrap(), 0i64);
    assert_eq!(find("+3\n+3\n+4\n-2\n-4\n").unwrap(), 10i64);
    assert_eq!(find("-6\n+3\n+8\n+5\n-6\n").unwrap(), 5i64);
    assert_eq!(find("+7\n+7\n-2\n-7\n-4\n").unwrap(), 14i64);
}
