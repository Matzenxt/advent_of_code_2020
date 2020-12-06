use std::fs::File;
use std::io::{BufReader, BufRead};
use regex::Regex;

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
                        let value = key_value.next().unwrap().parse::<i32>().unwrap();

                        if value >= 1920 && value <= 2002 {
                            1
                        } else {
                            0
                        }
                    },
                    "iyr" => {
                        let value = key_value.next().unwrap().parse::<i32>().unwrap();

                        if value >= 2010 && value <= 2020 {
                            1
                        } else {
                            0
                        }
                    },
                    "eyr" => {
                        let value = key_value.next().unwrap().parse::<i32>().unwrap();

                        if value >= 2020 && value <= 2030 {
                            1
                        } else {
                            0
                        }
                    },
                    "hgt" => {
                        let value = key_value.next().unwrap();

                        let re: Regex = Regex::new("([0-9]*)([a-z]*)").unwrap();
                        let caps = re.captures(&value).unwrap();

                        let metric = caps.get(2).unwrap().as_str();
                        let height = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
                        match metric {
                            "cm" => {
                                if height >= 150 && height <= 193 {
                                    1
                                } else {
                                    0
                                }
                            },
                            "in" => {
                                if height >= 59 && height <= 76 {
                                    1
                                } else {
                                    0
                                }
                            },
                            _ => {
                                println!("Error");
                                0
                            }
                        }

                    },
                    "hcl" => {
                        let value = key_value.next().unwrap();

                        let re: Regex = Regex::new("#[0-9a-f]{6}").unwrap();

                        if re.is_match(value) {
                            1
                        } else {
                            0
                        }
                    },
                    "ecl" => {
                        let value = key_value.next().unwrap();

                        match value {
                            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => {
                                1
                            },
                            _ => {
                                0
                            }
                        }
                    },
                    "pid" => {
                        let value = key_value.next().unwrap();

                        if value.len() == 9 && value.parse::<i32>().is_ok() {
                            1
                        } else {
                            0
                        }
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
