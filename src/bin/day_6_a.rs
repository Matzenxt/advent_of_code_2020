use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let file = File::open("inputs/day_6.txt").unwrap();

    let count = compute(BufReader::new(file));
    println!("Answer: {}", count);
}

fn compute(reader: BufReader<File>) -> usize {
    let mut counter = 0;

    let mut answers = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();

        if line.is_empty() {
            answers.sort();
            answers.dedup();
            counter += answers.len();
            answers = Vec::new();
        } else {
            for char in line.chars() {
                answers.push(char);
            }
        }
    }

    answers.sort();
    answers.dedup();
    counter += answers.len();

    counter
}

#[test]
fn test() {
    let file = File::open("inputs/day_6_test.txt").unwrap();

    assert_eq!(compute(BufReader::new(file)), 11);
}
