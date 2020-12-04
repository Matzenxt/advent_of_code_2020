use std::fs::File;
use std::io::{BufReader, BufRead};
use regex::Regex;

fn main() {
    let file = File::open("inputs/day_2.txt").unwrap();
    let reader = BufReader::new(file);

    let mut lines: Vec<String> = Vec::new();

    for line in reader.lines() {
        lines.push(line.unwrap());
    }

    let valid_passwords = compute(lines);
    println!("Valid passwords: {}", valid_passwords);
}

fn compute(lines: Vec<String>) -> i32 {
    let mut counter = 0;

    for line in lines {
        if check(line) {
            counter += 1;
        }
    }

    counter
}

fn check(line: String) -> bool {
    let re: Regex = Regex::new("([0-9]*)-([0-9]*) ([a-z]): ([a-z]*)").unwrap();

    let caps = re.captures(&line).unwrap();

    let lower = caps.get(1).unwrap().as_str();
    let lower = lower.parse::<i32>().unwrap();
    let upper = caps.get(2).unwrap().as_str();
    let upper = upper.parse::<i32>().unwrap();

    let check_char = caps.get(3).unwrap().as_str().chars().next().unwrap();
    let mut char_counter = 0;
    for char in caps.get(4).unwrap().as_str().chars() {
        if check_char == char {
            char_counter += 1;
        }
    }

    char_counter >= lower && char_counter <= upper
}

#[test]
fn test() {
    let lines: Vec<String> = vec![
        "1-3 a: abcde".to_string(),
        "1-3 b: cdefg".to_string(),
        "2-9 c: ccccccccc".to_string()
    ];

    assert_eq!(compute(lines), 2);
}
