use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let file = File::open("inputs/day_1.txt").unwrap();
    let reader = BufReader::new(file);

    let mut numbers: Vec<i32> = Vec::new();

    for line in reader.lines() {
        numbers.push(line.unwrap().parse::<i32>().unwrap());
    }

    compute(numbers);
}

fn compute(numbers: Vec<i32>) -> i32 {
    for number1 in &numbers {
        for number2 in &numbers {
            if (number1 + number2) == 2020 {
                println!("{}", number1 * number2);
                return number1 * number2;
            }
        }
    }

    0
}

#[test]
fn test() {
    let numbers: Vec<i32> = vec![1721, 979, 366, 299, 675, 1456];

    assert_eq!(compute(numbers), 514579);
}
