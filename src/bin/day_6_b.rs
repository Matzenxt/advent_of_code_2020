use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::{HashMap, HashSet};
use std::collections::hash_map::RandomState;

fn main() {
    let file = File::open("inputs/day_6.txt").unwrap();

    let a: HashSet<char> = vec!['a', 'b', 'c'].into_iter().collect();
    let b: HashSet<char> = ['v', 'b', 'c'].iter().cloned().collect();
    let c: HashSet<char> = ['v', 'b', 'b'].iter().cloned().collect();

    let mut intersection = a.intersection(&b);

    let mut temp = HashSet::new();
    for test in intersection.clone() {
        println!("{}", test);
        temp.insert(test.clone());
    }

    println!("ASDF {}", intersection.size_hint().1.unwrap());

    let mut intersection = temp.intersection(&c);

    for test in intersection.clone() {
        println!("{}", test);
    }
    println!("ASDF {}", intersection.size_hint().1.unwrap());

    //let count = compute(BufReader::new(file));
    //println!("Answer: {}", count);
}

fn compute(reader: BufReader<File>) -> usize {
    let mut counter = 0;

    let mut pre_answer: Option<HashSet<char>> = None;

    for line in reader.lines() {
        let line = line.unwrap();

        if line.is_empty() {

            let temp = &pre_answer.unwrap();

            counter += temp.len();

            &pre_answer.unwrap().clear();
        } else {
            let mut answer: HashSet<char> = HashSet::new();
            for char in line.chars() {
                answer.insert(char);
            }

            match pre_answer.as_mut() {
                None => {
                    pre_answer = Some(answer);
                }
                Some(a) => {
                    let mut intersection = a.intersection(&answer);

                    a.clear();

                    for entry in intersection {
                        a.insert(entry.clone());
                    }
                }
            }
        }
    }

    counter
}

#[test]
fn test() {
    let file = File::open("inputs/day_6_test.txt").unwrap();

    assert_eq!(compute(BufReader::new(file)), 6);
}
