#[macro_use]
extern crate itertools;

use std::fs;

fn extract_chars(left: &str, right: &str) -> String {
    let mut result = String::new();
    let left = left.chars();
    let right = right.chars();
    for (l, r) in left.zip(right) {
        if l == r {
            result.push(l);
        }
    }
    result
}

fn diff(left: &str, right: &str) -> usize {
    let left = left.chars();
    let right = right.chars();
    let mut diff = 0;
    for (l, r) in left.zip(right) {
        if l != r {
            diff += 1;
        }
    }
    diff
}

fn main() -> Result<(), Box<std::error::Error>> {
    let input = fs::read_to_string("input")?;
    let liter = input.lines();
    let riter = input.lines();
    let mut chars = String::new();
    for (l, r) in iproduct!(liter, riter) {
        if diff(l, r) == 1 {
            chars = extract_chars(l, r);
            break
        }
    }
    println!("chars: {}", chars);
    Ok(())
}

#[test]
fn test_diff() {
    let l = "abcde";
    let r = "axcye";
    assert_eq!(diff(l, r), 2);
    let l = "fghij";
    let r = "fguij";
    assert_eq!(diff(l, r), 1);
}

#[test]
fn test_extract() {
    let l = "fghij";
    let r = "fguij";
    let result = extract_chars(l, r);
    assert_eq!(&result, "fgij");
}
