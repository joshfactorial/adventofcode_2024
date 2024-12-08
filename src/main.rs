use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Write};
use std::collections::HashSet;

type Position = (usize, usize);

type Obstacles = Vec<Position>;

fn main() {
    let input_file = "/home/joshfactorial/code/inputs/inputs.txt.full";
    let lines = read_lines(input_file).unwrap();
    let (obstacles, guard_pos, max_pos) = parse_lines(lines);
    let mut loop_obstacles = 0;
    let mut loops = 0;
    let mut log = File::create("/home/joshfactorial/code/test.log").unwrap();

    let (_, _, _, cells_covered) = move_guard(
        &obstacles, &guard_pos, &max_pos
    );

    println!("Num positions = {}", max_pos.0 * max_pos.1);
    for row in 0..max_pos.0 {
        for col in 0..max_pos.1 {
            if (row, col) == (guard_pos.0, guard_pos.1) || obstacles.contains(&(row, col)) ||
                !cells_covered.contains(&(row, col)) {
                continue
            };
            let mut test_obstacles = obstacles.clone();
            test_obstacles.push((row, col));
            let (x, y, status, cells_covered) = move_guard(
                &test_obstacles, &guard_pos, &max_pos
            );
            if x != None && y != Some(8) && cells_covered.len() != 4778 {
                log.write_all(&format!(
                    "Loop {loops}: {:?}, {:?} covered {} cells\n", x, y, cells_covered.len()
                ).into_bytes()).unwrap();
            }
            loops += 1;
            match status.as_str() {
                "loop" => {
                    loop_obstacles += 1;
                },
                _ => continue,
            }
        }
    }
    println!("part 2 obstacles: {}", loop_obstacles);
}

fn parse_lines(lines: io::Lines<BufReader<File>>) -> (
    Obstacles, (usize, usize, char), (usize, usize)
) {
    let mut obstacles: Vec<(usize, usize)> = Vec::new();
    let mut guard_position: (usize, usize, char) = (0, 0, '^');
    let mut num_rows = 0;
    let mut num_cols = 0;
    for (row, line) in lines.enumerate() {
        num_rows += 1;
        for (col, char) in line.unwrap().chars().enumerate() {
            if row == 0 { num_cols += 1 }
            match char {
                '#' => { obstacles.push((row, col)); },
                '^'|'>'|'v'|'<' => { guard_position = (row, col, char); },
                _ => { continue },
            }
        }
    }
    (obstacles, guard_position, (num_rows, num_cols))
}

fn move_guard(
    obstacles: &Obstacles, initial_guard_pos: &(usize, usize, char), max_pos: &(usize, usize),
) -> (Option<usize>, Option<usize>, String, HashSet<(usize, usize)>) {
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

    while (0..max_pos.0).contains(&guard_pos.0) &&
        (0..max_pos.1).contains(&guard_pos.1) {
        if safety_check > 1000 {
            // println!("Doing that safety dance");
            return (None, None, "loop".to_string(), cells_entered)
        }
        let current_delta = directions[&current_direction];
        let guard_x = guard_pos.0 as i32;
        let guard_y = guard_pos.1 as i32;
        let dest = {
            let x = if guard_x + current_delta.0 < 0 {
                // out of bounds
                return (None, Some(guard_pos.1), "exit".to_string(), cells_entered)
            } else {
                (guard_x + current_delta.0) as usize
            };
            let y = if guard_y + current_delta.1 < 0 {
                // out of bounds
                return (Some(guard_pos.0), None, "exit".to_string(), cells_entered)
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
                safety_check += 1;
            }
        } else if dest.0 < max_pos.0 &&
            dest.1 < max_pos.1 {
            // move
            if cells_entered.contains(&dest) {
                repeat_cell = true;
                if turns == 4 {
                    if prev_pattern == curr_pattern {
                        // println!("Stuck in a loop");
                        return (None, None, "loop".to_string(), cells_entered)
                    }
                    // check new pattern
                    prev_pattern = curr_pattern.clone();
                    curr_pattern = Vec::new();
                    turns = 0;
                    safety_check += 1;
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
    (Some(guard_pos.0), Some(guard_pos.1), "exit".to_string(), cells_entered)
}

pub fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    // This creates a buffer to read lines
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
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
        let max_pos = (3, 3);
        let (row, col, status, set) = move_guard(&obstacles, &guard_pos, &max_pos);
        assert_eq!((Some(4), Some(2), "exit".to_string(), 5), (row, col, status, set.len()))
    }

    #[test]
    fn test_move_guard_2() {
        // . # . .
        // . . . #
        // . ^ . .
        // . . . .
        let obstacles = Vec::from([(0, 1), (1, 3)]);
        let guard_pos= (2, 1, '^');
        let max_pos = (4, 4);
        let (row, col, status, set) = move_guard(&obstacles, &guard_pos, &max_pos);
        assert_eq!((Some(4), Some(2), "exit".to_string(), 5), (row, col, status, set.len()))
    }

    #[test]
    fn test_move_guard_3() {
        // . # . .
        // . . . #
        // . ^ . .
        // . . # .
        let obstacles = Vec::from([(0, 1), (1, 3), (3, 2)]);
        let guard_pos= (2, 1, '^');
        let max_pos = (4, 4);
        let (row, col, status, set) = move_guard(&obstacles, &guard_pos, &max_pos);
        assert_eq!((Some(2), None, "exit".to_string(), 5), (row, col, status, set.len()))
    }

    #[test]
    fn test_move_guard_loop() {
        // . # . .
        // . . . #
        // # ^ . .
        // . . # .
        let obstacles = Vec::from([(0, 1), (1, 3), (3, 2), (2, 0)]);
        let guard_pos= (2, 1, '^');
        let max_pos = (4, 4);
        let (row, col, status, set) = move_guard(&obstacles, &guard_pos, &max_pos);
        assert_eq!((None, None, "exit".to_string(), 4), (row, col, status, set.len()))
    }

    #[test]
    fn test_move_guard_full() {
        // . . . . # . . . . .
        // . . . . . . . . . #
        // . . . . . . . . . .
        // . . # . . . . . . .
        // . . . . . . . # . .
        // . . . . . . . . . .
        // . # . . ^ . . . . .
        // . . . . . . . . # .
        // # . . . . . . . . .
        // . . . . . . # . . .
        let obstacles = Vec::from([
            (0, 4), (1, 9), (3, 2), (4, 7), (6, 1), (7, 8), (8, 0), (9, 6)
        ]);
        let guard_pos= (6, 4, '^');
        let max_pos = (10, 10);
        let (row, col, status, set) = move_guard(&obstacles, &guard_pos, &max_pos);
        assert_eq!((Some(10), Some(7), "exit".to_string(), 41), (row, col, status, set.len()))
    }

    #[test]
    fn test_move_guard_new_1() {
        //   0 1 2 3 4 5 6 7 8 9
        // 0 . . . . # . . . . .
        // 1 . . . . . . . . . #
        // 2 . . . . . . . . . .
        // 3 . . # . . . . . . .
        // 4 . . . . . . . # . .
        // 5 . . . . . . . . . .
        // 6 . # . O ^ . . . . .
        // 7 . . . . . . . . # .
        // 8 # . . . . . . . . .
        // 9 . . . . . . # . . .
        let obstacles = Vec::from([
            (0, 4), (1, 9), (3, 2), (4, 7), (6, 1), (6, 3), (7, 8), (8, 0), (9, 6)
        ]);
        let guard_pos= (6, 4, '^');
        let max_pos = (10, 10);
        let (row, col, status, set) = move_guard(&obstacles, &guard_pos, &max_pos);
        assert_eq!((None, None, "loop".to_string(), 18), (row, col, status, set.len()))
    }

    #[test]
    fn test_move_guard_new_2() {
        //   0 1 2 3 4 5 6 7 8 9
        // 0 . . . . # . . . . .
        // 1 . . . . . . . . . #
        // 2 . . . . . . . . . .
        // 3 . . # . . . . . . .
        // 4 . . . . . . . # . .
        // 5 . . . . . . . . . .
        // 6 . # . . ^ . . . . .
        // 7 . . . . . . O . # .
        // 8 # . . . . . . . . .
        // 9 . . . . . . # . . .
        let obstacles = Vec::from([
            (0, 4), (1, 9), (3, 2), (4, 7), (6, 1), (7, 6), (7, 8), (8, 0), (9, 6)
        ]);
        let guard_pos= (6, 4, '^');
        let max_pos = (10, 10);
        let (row, col, status, set) = move_guard(&obstacles, &guard_pos, &max_pos);
        assert_eq!((None, None, "loop".to_string(), 26), (row, col, status, set.len()))
    }
}