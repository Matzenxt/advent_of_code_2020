use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let file = File::open("inputs/day_4.txt").unwrap();

    let valid_passports = compute(BufReader::new(file));
    println!("Valid passports: {}", valid_passports);
}

fn compute(reader: BufReader<File>) -> i32 {
    const VALID_FIELDS_NEEDED: i32 = 7; // 8 fields in total, -1 because cid is ignored
    let mut valid_passports = 0;
    let mut field_counter = 0;

    for line in reader.lines() {
        let line = line.unwrap();

        if line.is_empty() {
            println!("Passport has {} valid fields", field_counter);

            if field_counter >= VALID_FIELDS_NEEDED {
                valid_passports += 1;
            }

            field_counter = 0;
        } else {
            let pairs = line.split_whitespace();

            for pair in pairs {
                let mut key_value = pair.split(":");

                field_counter += match key_value.next().unwrap() {
                    "byr" => {
                        1
                    },
                    "iyr" => {
                        1
                    },
                    "eyr" => {
                        1
                    },
                    "hgt" => {
                        1
                    },
                    "hcl" => {
                        1
                    },
                    "ecl" => {
                        1
                    },
                    "pid" => {
                        1
                    },
                    "cid" => {
                        0
                    },
                    _ => {
                        println!("Unknown key");
                        0
                    }
                }

            }
        }
    }

    valid_passports
}

#[test]
fn test() {
    let file = File::open("inputs/day_4_test.txt").unwrap();

    assert_eq!(compute(BufReader::new(file)), 2);
}
