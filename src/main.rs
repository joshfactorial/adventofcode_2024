use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

const CHARS_OF_INTEREST: [char;4] = ['X', 'M', 'A', 'S'];

fn main() {
    let input_file = "/home/joshfactorial/code/inputs/inputs.txt";
    let lines = read_lines(input_file).unwrap();
    let running_total = parse_lines(lines);

    println!("Running total = {running_total}")
}

fn parse_lines(lines: io::Lines<BufReader<File>>) -> u32 {
    let mut matrix = Vec::new();
    let mut row_num = 0;
    let mut x_positions = Vec::new();
    for line in lines {
        let mut row = Vec::new();
        match line {
            Ok(l) => {
                let row_chars: Vec<char> = l.chars().collect();
                for col_num in 0..row_chars.len() {
                    let current_c = row_chars[col_num];
                    if CHARS_OF_INTEREST.contains(&current_c) {
                        if current_c == 'X' {
                            x_positions.push((row_num, col_num));
                        }
                    }
                    row.push(current_c.clone())

                }
                matrix.push(row.clone());
            },
            Err(e) => panic!("Error reading file: {}", e),
        }
        row_num += 1;
    }

    trace_nodes(x_positions, matrix)
}


fn trace_nodes(x_positions: Vec<(usize,usize)>, matrix: Vec<Vec<char>>) -> u32 {
    let mut count = 0;
    let neighbors: [[(i8,i8);3];8] = [
        [( 0, -1), ( 0, -2), ( 0, -3)], // left
        [( 0,  1), ( 0,  2), ( 0,  3)], // right

        [(-1, -1), (-2, -2), (-3, -3)], // up/left
        [(-1,  0), (-2,  0), (-3,  0)], // up
        [(-1,  1), (-2,  2), (-3,  3)], // up/right

        [( 1,  1), ( 2,  2), ( 3,  3)], // down/right
        [( 1,  0), ( 2,  0), ( 3,  0)], // down
        [( 1, -1), ( 2, -2), ( 3, -3)], // down/left
    ];
    let num_rows = matrix.len();
    let num_cols = matrix[0].len();
    println!("Checking {} x's", x_positions.len());
    for (row, col) in &x_positions {
        println!("Checking X at {}, {}", row, col);
        count += {
            let mut subtotal = 0;
            'neigh: for (delta_x, delta_y) in neighbors {
                let x = match delta_x {
                    0 => *row,
                    1 => {
                        if *row < (num_rows - 1) {
                            *row + 1
                        } else {
                            // dead end try another neighbor
                            continue 'neigh
                        }
                    },
                    // -1 is the only other possibility
                    _ => {
                        if *row > 0 {
                            *row - 1
                        } else {
                            // dead end, try another neighbor
                            continue 'neigh
                        }
                    },
                };
                let y = match delta_y {
                    0 => *col,
                    1 => {
                        if *col < (num_cols - 1) {
                            *col + 1
                        } else {
                            // dead end
                            continue 'neigh
                        }
                    },
                    // -1 is the only other possibility
                    _ => {
                        if *col > 0 {
                            *col - 1
                        } else {
                            // dead end
                            continue 'neigh
                        }
                    },
                };
                if matrix[x][y] == 'M' {
                    match delta_x {
                        0 => {
                            match delta_y {
                                -1 => {
                                    if *col >= 3 {
                                        if matrix[*row][*col-2] == 'A' &&
                                            matrix[*row][*col-3] == 'S' {
                                            // hit
                                            println!("Hit ending on {}, {}", *row, *col-3);
                                            subtotal += 1;
                                        }
                                    }
                                },
                                // stands for +1
                                _ => {
                                    if *col < (num_cols - 3) {
                                        if matrix[*row][*col+2] == 'A' &&
                                            matrix[*row][*col+3] == 'S' {
                                            // hit
                                            println!("Hit ending on {}, {}", *row, *col+3);
                                            subtotal += 1;
                                        }
                                    }
                                },
                            }
                        },
                        1 => {
                            match delta_y {
                                -1 => {
                                    if *row < (num_rows - 3) &&
                                        *col >= 3 {
                                        if matrix[*row+2][*col-2] == 'A' &&
                                            matrix[*row+3][*col-3] == 'S' {
                                            // hit
                                            println!("Hit ending on {}, {}", *row+3, *col-3);
                                            subtotal += 1;
                                        }
                                    }
                                },
                                0 => {
                                    if *row < (num_rows - 3) {
                                        if matrix[*row+2][*col] == 'A' &&
                                            matrix[*row+3][*col] == 'S' {
                                            // hit
                                            println!("Hit ending on {}, {}", *row+3, *col);
                                            subtotal += 1;
                                        }
                                    }
                                },
                                // 1 by gentleman's agreement
                                _ => {
                                    if *row < (num_rows - 3) &&
                                        *col < (num_cols - 3) {
                                        if matrix[*row+2][*col+2] == 'A' &&
                                            matrix[*row+3][*col+3] == 'S' {
                                            // hit
                                            println!("Hit ending on {}, {}", *row+3, *col+3);
                                            subtotal += 1;
                                        }
                                    }
                                },
                            }
                        },
                        // stands for -1
                        _ => {
                            match delta_y {
                                -1 => {
                                    if *row >= 3 &&
                                        *col >= 3 {
                                        if matrix[*row-2][*col-2] == 'A' &&
                                            matrix[*row-3][*col-3] == 'S' {
                                            // hit
                                            println!("Hit ending on {}, {}", *row-3, *col-3);
                                            subtotal += 1;
                                        }
                                    }
                                },
                                0 => {
                                    if *row >= 3 {
                                        if matrix[*row-2][*col] == 'A' &&
                                            matrix[*row-3][*col] == 'S' {
                                            // hit
                                            println!("Hit ending on {}, {}", *row-3, *col);
                                            subtotal += 1;
                                        }
                                    }
                                },
                                // 1 by GA
                                _ => {
                                    if *row >= 3 &&
                                        *col < (num_cols - 3) {
                                        if matrix[*row-2][*col+2] == 'A' &&
                                            matrix[*row-3][*col+3] == 'S' {
                                            // hit
                                            println!("Hit ending on {}, {}", *row+3, *col+3);
                                            subtotal += 1;
                                        }
                                    }
                                },
                            }
                        },
                    }
                }
            }
            subtotal
        }
    }
    count
}

pub fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    // This creates a buffer to read lines
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
