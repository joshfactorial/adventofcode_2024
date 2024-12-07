use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

type Position = (usize, usize);

type Obstacles = Vec<Position>;

fn main() {
    let input_file = "/home/joshfactorial/code/inputs/inputs.txt.full";
    let lines = read_lines(input_file).unwrap();
    let (obstacles, mut guard_pos, max_pos) = parse_lines(lines);
    let positions = part1(&obstacles, &mut guard_pos, &max_pos);
    println!("part 1 distinct positions: {}", positions);
}

fn parse_lines(lines: io::Lines<BufReader<File>>) -> (Obstacles, Position, Position) {
    let mut obstacles: Vec<(usize, usize)> = Vec::new();
    let mut guard_position: (usize, usize) = (0, 0);
    let mut max_coords: (usize, usize) = (0, 0);
    for (row, line) in lines.enumerate() {
        max_coords.0 += 1;
        for (col, char) in line.unwrap().chars().enumerate() {
            max_coords.1 += 1;
            match char {
                '#' => { obstacles.push((row, col)); },
                '^' => { guard_position = (row, col); },
                _ => { continue },
            }
        }
    }
    (obstacles, guard_position, max_coords)
}

fn part1(obstacles: &Obstacles, initial_guard_pos: &Position, max_pos: &Position) -> usize {
    //move guard
    // add the starting pos.
    let mut cells_entered = HashSet::new();
    let directions: HashMap<char, (i32, i32)> = HashMap::from([
        ('^', (-1,  0)),
        ('>', ( 0,  1)),
        ('v', ( 1,  0)),
        ('<', ( 0, -1)),
    ]);
    let mut current_direction = '^';
    let mut guard_pos = initial_guard_pos.clone();
    cells_entered.insert(guard_pos.clone());
    while guard_pos.0 > 0 &&
        guard_pos.1 > 0 &&
        guard_pos.0 < max_pos.0 &&
        guard_pos.1 < max_pos.1 {
        let current_delta = directions[&current_direction];
        let guard_x = guard_pos.0 as i32;
        let guard_y = guard_pos.1 as i32;
        let mut dest = (
            (guard_x + current_delta.0) as usize,
            (guard_y + current_delta.1) as usize,
        );
        if obstacles.contains(&dest) {
            // turn
            current_direction = match current_direction {
                '^' => '>',
                '>' => 'v',
                'v' => '<',
                '<' => '^',
                _ => panic!("unknown direction")
            }
        } else if (0..max_pos.0).contains(&dest.0) &&
            (0..max_pos.1).contains(&dest.1) {
            // move
            cells_entered.insert(dest.clone());
            guard_pos = dest.clone();
        } else {
            // set guard pos out of bounds to end loop
            guard_pos = dest.clone()
        }
    }
    cells_entered.len()
}

fn part2(test: u32) -> u32 {
    todo!()
}

pub fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    // This creates a buffer to read lines
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
