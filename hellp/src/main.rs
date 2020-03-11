use std::io::Write;
use std::str::FromStr;

fn main() {
    let mut numbers = Vec::new();

    for arg in std::env::args().skip(1) {
        numbers.push(isize::from_str(&arg).expect("error parsing argument"));
    }

    if numbers.len() == 0 {
        writeln!(std::io::stderr(), "Usage: add NUMBER ...").unwrap();
        std::process::exit(1);
    }

    let mut added_numbers = Vec::new();

    for n in &numbers {
        added_numbers.push(add(*n));
    }

    for n in &added_numbers {
        println!("{}", *n)
    }
}

fn add(a: isize) -> isize {
    1 + a
}

#[test]
fn test_add() {
    assert_eq!(add(2), 3);
    assert_eq!(add(0), 1);
    assert_eq!(add(-1), 0);
    assert_eq!(add(-2), -1);
}
