use std::fs;

fn main() -> Result<(), Box<std::error::Error>> {
    let input = fs::read_to_string("input")?;
    let sum: i64 = input.lines().flat_map(|line| line.parse::<i64>().ok()).sum();
    println!("{}", sum);
    Ok(())
}
