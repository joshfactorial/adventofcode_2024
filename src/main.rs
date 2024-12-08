use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

type Position = (usize, usize);

type Obstacles = Vec<Position>;

fn main() {
    let input_file = "/home/joshfactorial/code/inputs/inputs.txt";
    let lines = read_lines(input_file).unwrap();
    let (obstacles, guard_pos, max_pos) = parse_lines(lines);
    let mut loop_obstacles = 0;
    println!("Num rows = {}", max_pos);
    for row in 0..max_pos {
        for col in 0..max_pos {
            let mut test_obstacles = obstacles.clone();
            test_obstacles.push((row, col));
            let (x, y, status, cells_covered) = move_guard(
                &test_obstacles, &guard_pos, &max_pos
            );
            match status.as_str() {
                "loop" => {
                    loop_obstacles += 1;
                },
                "unk" => {
                    println!("Not sure what happened here. {}, {}, {:?}", row, col, test_obstacles)
                },
                _ => continue,
            }
        }
    }
    println!("part 2 obstacles: {}", loop_obstacles);
}

fn parse_lines(lines: io::Lines<BufReader<File>>) -> (Obstacles, (usize, usize, char), usize) {
    let mut obstacles: Vec<(usize, usize)> = Vec::new();
    let mut guard_position: (usize, usize, char) = (0, 0, '^');
    let mut num_rows = 0;
    for (row, line) in lines.enumerate() {
        num_rows += 1;
        for (col, char) in line.unwrap().chars().enumerate() {
            match char {
                '#' => { obstacles.push((row, col)); },
                '^'|'>'|'v'|'<' => { guard_position = (row, col, char); },
                _ => { continue },
            }
        }
    }
    (obstacles, guard_position, num_rows)
}

fn move_guard(
    obstacles: &Obstacles, initial_guard_pos: &(usize, usize, char), max_pos: &usize
) -> (Option<usize>, Option<usize>, String, usize) {
    // move guard
    // add the starting pos.
    let mut cells_entered = HashSet::new();
    let directions: HashMap<char, (i32, i32)> = HashMap::from([
        ('^', (-1,  0)),
        ('>', ( 0,  1)),
        ('v', ( 1,  0)),
        ('<', ( 0, -1)),
    ]);
    let mut current_direction = initial_guard_pos.2;
    let mut guard_pos = (initial_guard_pos.0.clone(), initial_guard_pos.1.clone());
    cells_entered.insert(guard_pos.clone());

    let mut repeat_cell = false;
    let mut turns = 0;
    let mut prev_pattern: Vec<(usize,usize)> = Vec::new();
    let mut curr_pattern: Vec<(usize,usize)> = Vec::new();
    let mut safety_check = 0;

    while (0..*max_pos).contains(&guard_pos.0) &&
        (0..*max_pos).contains(&guard_pos.1) {
        safety_check += 1;
        if safety_check > 100 {
            panic!("Too many LOOOOPS")
        }
        let current_delta = directions[&current_direction];
        let guard_x = guard_pos.0 as i32;
        let guard_y = guard_pos.1 as i32;
        let dest = {
            let x = if guard_x + current_delta.0 < 0 {
                // out of bounds
                return (None, Some(guard_pos.1), "exit".to_string(), cells_entered.len())
            } else {
                (guard_x + current_delta.0) as usize
            };
            let y = if guard_y + current_delta.1 < 0 {
                // out of bounds
                return (Some(guard_pos.0), None, "exit".to_string(), cells_entered.len())
            } else {
                (guard_y + current_delta.1) as usize
            };
            (x, y)
        };
        if obstacles.contains(&dest) {
            // turn
            current_direction = match current_direction {
                '^' => '>',
                '>' => 'v',
                'v' => '<',
                '<' => '^',
                _ => panic!("unknown direction")
            };
            if repeat_cell {
                turns += 1;
            }
        } else if dest.0 < *max_pos &&
            dest.1 < *max_pos {
            // move
            if cells_entered.contains(&dest) && !repeat_cell {
                repeat_cell = true;
                if turns == 4 {
                    if prev_pattern == curr_pattern {
                        return (
                            None,
                            None,
                            "loop".to_string(),
                            cells_entered.len()
                        )
                    }
                    // check new pattern
                    prev_pattern = curr_pattern.clone();
                    curr_pattern = Vec::new();
                    turns = 0;
                }
                curr_pattern.push(dest.clone());
            }
            cells_entered.insert(dest.clone());
            guard_pos = dest.clone()
        } else {
            // final destination
            guard_pos = dest.clone();
        }
    }
    (Some(guard_pos.0), Some(guard_pos.1), "exit".to_string(), cells_entered.len())
}

fn part2() -> u32 {
    todo!()
}

pub fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    // This creates a buffer to read lines
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_guard() {
        //  . # .
        //  . . .
        //  . ^ .
        let obstacles = Vec::from([(0, 1)]);
        let guard_pos= (2, 1, '^');
        let max_pos = 3;
        assert_eq!((Some(1), Some(3), "exit".to_string(), 3), move_guard(&obstacles, &guard_pos, &max_pos))
    }

    #[test]
    fn test_move_guard_2() {
        // . # . .
        // . . . #
        // . ^ . .
        // . . . .
        let obstacles = Vec::from([(0, 1), (1, 3)]);
        let guard_pos= (2, 1, '^');
        let max_pos = 4;
        assert_eq!((Some(4), Some(2), "exit".to_string(), 5), move_guard(&obstacles, &guard_pos, &max_pos))
    }

    #[test]
    fn test_move_guard_3() {
        // . # . .
        // . . . #
        // . ^ . .
        // . . # .
        let obstacles = Vec::from([(0, 1), (1, 3), (3, 2)]);
        let guard_pos= (2, 1, '^');
        let max_pos = 4;
        assert_eq!((Some(2), None, "exit".to_string(), 5), move_guard(&obstacles, &guard_pos, &max_pos))
    }

    #[test]
    fn test_move_guard_loop() {
        // . # . .
        // . . . #
        // # ^ . .
        // . . # .
        let obstacles = Vec::from([(0, 1), (1, 3), (3, 2), (2, 0)]);
        let guard_pos= (2, 1, '^');
        let max_pos = 4;
        assert_eq!((None, None, "loop".to_string(), 4), move_guard(&obstacles, &guard_pos, &max_pos))
    }
}