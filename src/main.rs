use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

type ParsedInput = HashMap<(i16, i16), char>;

fn main() {
    let input_file = "/home/joshfactorial/code/inputs/inputs.txt.full";
    let lines = read_lines(input_file).unwrap();
    let input = parse_lines(lines);
    let output = part2(&input);
    println!("Output = {output}")
}

fn parse_lines(lines: io::Lines<BufReader<File>>) -> ParsedInput {
    let mut output = HashMap::new();

    for (y, line) in lines.enumerate() {
        for (x, c) in line.unwrap().char_indices() {
            output.insert((x as i16, y as i16), c);
        }
    }

    output
}

fn part1(input: &ParsedInput) -> u32 {
    let mut count = 0;
    let neighbors: [[(i16,i16);3];8] = [
        [( 0, -1), ( 0, -2), ( 0, -3)], // left
        [( 0,  1), ( 0,  2), ( 0,  3)], // right

        [(-1, -1), (-2, -2), (-3, -3)], // up/left
        [(-1,  0), (-2,  0), (-3,  0)], // up
        [(-1,  1), (-2,  2), (-3,  3)], // up/right

        [( 1,  1), ( 2,  2), ( 3,  3)], // down/right
        [( 1,  0), ( 2,  0), ( 3,  0)], // down
        [( 1, -1), ( 2, -2), ( 3, -3)], // down/left
    ];

    for ((x, y), chr) in input {
        if *chr != 'X' { continue }
        for [(mx, my), (ax, ay), (sx, sy)] in neighbors {
            if input.get(&(x + mx, y + my)) == Some(&'M') &&
                input.get(&(x + ax, y + ay)) == Some(&'A') &&
                input.get(&(x + sx, y + sy)) == Some(&'S') {
                count += 1;
            }
        }
    }
    count
}

fn part2(input: &ParsedInput) -> u32 {
    let mut count = 0;
    // (1) M . M  (2) M . S (3) S . S (4)  S . M
    //     . A .      . A .     . A .      . A .
    //     S . S      M . S     M . M      S . M
    //
    let neighbors: [[(i16,i16);4];4] = [
        [(-1, -1), (1,  -1), ( 1,  1), (-1,  1)], // M, M, S, S
        [(-1, -1), (-1,  1), ( 1, -1), ( 1,  1)], // M, M, S, S
        [(-1,  1), ( 1,  1), (-1, -1), ( 1, -1)], // M, M, S, S
        [( 1,  1), ( 1, -1), (-1, -1), (-1,  1)], // M, M, S, S
    ];

    for ((x, y), chr) in input {
        if *chr != 'A' { continue }
        for [(mx, my), (ax, ay), (sx, sy), (tx, ty)] in neighbors {
            if input.get(&(x + mx, y + my)) == Some(&'M') &&
                input.get(&(x + ax, y + ay)) == Some(&'M') &&
                input.get(&(x + sx, y + sy)) == Some(&'S') &&
                input.get(&(x + tx, y + ty)) == Some(&'S') {
                println!("Hit on {}, {}", x, y);
                count += 1;
            }
        }
    }
    count
}

pub fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    // This creates a buffer to read lines
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
