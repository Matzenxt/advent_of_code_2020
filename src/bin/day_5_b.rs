use std::fs::File;
use std::io::{BufReader, BufRead};
use std::cmp::Ordering;

#[derive(Debug)]
struct BoardingPass {
    row: usize,
    column: usize,
    seat_id: usize,
}

impl BoardingPass {
    fn new(row: usize, column: usize) -> Self {
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

    // Because the are missing rows in the front and back, lookout for seat id that isn't incremental from the previous one

    let empty_seats: Vec<usize> = compute(boarding_passes);
    println!("Empty seats:");
    for empty_seat in empty_seats {
        println!(" - Seat ID: {}", empty_seat)
    }
}

fn compute(boarding_passes: Vec<String>) -> Vec<usize> {
    let mut free_seats: Vec<usize> = Vec::new();
    let mut seats: Vec<Vec<bool>> = vec![vec![false; 8]; 128];

    for boarding_pass in boarding_passes {
        let current = BoardingPass::compute(boarding_pass);

        seats[current.row][current.column] = true;
    }

    for seats in seats.iter().enumerate() {
        for seat in seats.1.iter().enumerate() {
            let temp = seat.1.to_owned();
            if !temp {
                free_seats.push(seats.0 * 8 + seat.0);
            }
        }
    }

    free_seats
}
