use std::fs::File;
use std::io::{BufReader, BufRead};
use std::cmp::Ordering;

#[derive(Debug)]
struct BoardingPass {
    row: i32,
    column: i32,
    seat_id: i32,
}

impl BoardingPass {
    fn new(row: i32, column: i32) -> Self {
        BoardingPass {
            row,
            column,
            seat_id: row * 8 + column
        }
    }

    fn compute(input: String) -> Self {
        let mut row_lower = 0;
        let mut row_upper = 127;

        let mut column_lower = 0;
        let mut column_upper = 7;

        for char in input.chars() {
            let row_middle = (row_lower + row_upper)/ 2;
            let column_middle = (column_lower + column_upper)/ 2;

            match char {
                'B' => {
                    row_lower = row_middle + 1;
                },
                'F' => {
                    row_upper = row_middle;
                },
                'L' => {
                    column_upper = column_middle;
                },
                'R' => {
                    column_lower = column_middle + 1;
                },
                _ => {
                    panic!("Unknown");
                }
            }
        }

        BoardingPass::new(row_upper, column_lower)
    }
}

impl Ord for BoardingPass {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.seat_id > other.seat_id {
            Ordering::Greater
        } else if self.seat_id < other.seat_id {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for BoardingPass {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for BoardingPass { }


impl PartialEq for BoardingPass {
    fn eq(&self, other: &Self) -> bool {
        (self.row, self.column, self.seat_id) == (other.row, other.column, other.seat_id)
    }
}

fn main() {
    let file = File::open("inputs/day_5.txt").unwrap();
    let reader = BufReader::new(file);

    let mut boarding_passes: Vec<String> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();

        boarding_passes.push(line);
    }

    let boarding_pass = compute(boarding_passes);
    println!("Row: {}, Column: {}, Seat ID: {}", boarding_pass.row, boarding_pass.column, boarding_pass.seat_id);
}

fn compute(boarding_passes: Vec<String>) -> BoardingPass {
    let mut boarding_pass_max = BoardingPass::new(0, 0);

    for boarding_pass in boarding_passes {
        let current = BoardingPass::compute(boarding_pass);

        if boarding_pass_max < current {
            boarding_pass_max = current;
        }
    }

    boarding_pass_max
}

#[test]
fn test() {
    assert_eq!(compute(vec!["FBFBBFFRLR".to_string()]), BoardingPass::new(44, 5));
    assert_eq!(compute(vec!["BFFFBBFRRR".to_string()]), BoardingPass::new(70, 7));
    assert_eq!(compute(vec!["FFFBBBFRRR".to_string()]), BoardingPass::new(14, 7));
    assert_eq!(compute(vec!["BBFFBBFRLL".to_string()]), BoardingPass::new(102, 4));
}
