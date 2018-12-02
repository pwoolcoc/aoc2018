use std::collections::BTreeMap;
use std::fs;

fn letter_count(id: &str) -> BTreeMap<char, usize> {
    let mut counts = BTreeMap::new();
    for c in id.chars() {
        let count = counts.entry(c).or_insert(0usize);
        *count += 1;
    }
    counts
}

fn has_letter_count(id: &str, count: usize) -> bool {
    let counts = letter_count(id);
    for c in counts.values() {
        if *c == count {
            return true;
        }
    }
    false
}

fn main() -> Result<(), Box<std::error::Error>> {
    let input = fs::read_to_string("input")?;
    let mut twos = 0;
    let mut threes = 0;
    for line in input.lines() {
        if has_letter_count(&line, 2) {
            twos += 1;
        }
        if has_letter_count(&line, 3) {
            threes += 1;
        }
    }
    println!("{}", twos * threes);
    Ok(())
}

#[test]
fn test_has_letter_counts() {
    let test = "kbqwucvzgsmhpoelvnahydifuj";
    assert!(has_letter_count(test, 2));
    assert!(!has_letter_count(test, 3));
}

#[test]
fn test_letter_count() {
    let test = "kbqwucvzgsmhpoelvnahydifuj";
    let count = letter_count(&test);
    let mut expected = BTreeMap::new();
    expected.insert('a', 1);
    expected.insert('b', 1);
    expected.insert('c', 1);
    expected.insert('d', 1);
    expected.insert('e', 1);
    expected.insert('f', 1);
    expected.insert('g', 1);
    expected.insert('h', 2);
    expected.insert('i', 1);
    expected.insert('j', 1);
    expected.insert('k', 1);
    expected.insert('l', 1);
    expected.insert('m', 1);
    expected.insert('n', 1);
    expected.insert('o', 1);
    expected.insert('p', 1);
    expected.insert('q', 1);
    expected.insert('s', 1);
    expected.insert('u', 2);
    expected.insert('v', 2);
    expected.insert('w', 1);
    expected.insert('y', 1);
    expected.insert('z', 1);
    assert_eq!(count, expected);
}
