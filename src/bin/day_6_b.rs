use std::fs::File;
use std::io::{BufReader, BufRead};
use array_tool::vec::Intersect;

fn main() {
    let file = File::open("inputs/day_6.txt").unwrap();

    let count = compute(BufReader::new(file));
    println!("Answer: {}", count);
}

fn compute(reader: BufReader<File>) -> usize {
    let mut counter = 0;

    let mut current: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();

        if line.is_empty() {

            let mut temp = current.first().unwrap().to_owned();

            for answer in current.clone() {
                temp = temp.intersect(answer);
            }

            counter += temp.len();

            current.clear();

        } else {
            let mut answer: Vec<char> = Vec::new();
            for char in line.chars() {
                answer.push(char);
            }
            current.push(answer);
        }
    }

    let mut temp = current.first().unwrap().to_owned();

    for answer in current.clone() {
        temp = temp.intersect(answer);
    }

    counter += temp.len();

    counter
}

#[test]
fn test() {
    let file = File::open("inputs/day_6_test.txt").unwrap();

    assert_eq!(compute(BufReader::new(file)), 6);
}
