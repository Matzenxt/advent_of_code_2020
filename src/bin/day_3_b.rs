use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let file = File::open("inputs/day_3.txt").unwrap();
    let reader = BufReader::new(file);

    let mut map: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut temp: Vec<char> = Vec::new();

        for char in line.chars() {
            temp.push(char);
        }

        map.push(temp);
    }

    let encounters = compute(map);
    println!("Tree encounters: {}", encounters);
}

fn compute(map: Vec<Vec<char>>) -> i64 {
    let mut product: i64 = 1;

    let map_height = map.len();
    let tile_width = map.get(0).unwrap().len();

    println!("Height: {}, width: {}", map_height, tile_width);

    let slopes = vec![
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2)
    ];

    for slope in slopes {
        let mut pos_x = 0;
        let mut pos_y = 0;

        let mut counter = 0;

        while pos_y < map_height {
            if map[pos_y][pos_x] == '#' {
                counter += 1;
            }

            pos_y += slope.1;
            pos_x = (pos_x + slope.0) % tile_width;
        }

        product *= counter;
    }

    product
}

#[test]
fn test() {
    let input = r#"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"#;

    let mut map: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        let mut temp: Vec<char> = Vec::new();

        for char in line.chars() {
            temp.push(char);
        }

        map.push(temp);
    }

    assert_eq!(compute(map), 336);
}
