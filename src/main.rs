use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Error};
use std::path::Path;
use regex::Regex;

fn main() {
    let input_file = "/home/joshfactorial/code/inputs/inputs.txt.full";
    let lines = read_lines(input_file).unwrap();
    let running_total = parse_lines(lines);

    println!("Running total = {running_total}")
}

fn parse_lines(lines: io::Lines<BufReader<File>>) -> u32 {
    let mut running_total = 0;
    let mut act = true;
    for line in lines {
        match line {
            Ok(l) => {
                let mut commands: Vec<&str> = Vec::new();
                let re = Regex::new(r"mul\([0-9][0-9]?[0-9]?,[0-9][0-9]?[0-9]?\)|don't\(\)|do\(\)").unwrap();
                for command in re.find_iter(&l) {
                    match &command.as_str()[0..3] {
                        "mul" => {
                            if act {
                                let digit_seek = Regex::new(r"[0-9][0-9]?[0-9]?").unwrap();
                                let mults: Vec<u32> = digit_seek.find_iter(command.as_str()).map(|m| m.as_str().parse::<u32>().unwrap()).collect();
                                running_total += mults[0] * mults[1];
                            }
                        },
                        "do(" => { act = true },
                        "don" => { act = false },
                        _ => continue
                    }
                }

            },
            _ => {},
        }
    }
    running_total
}

pub fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    // This creates a buffer to read lines
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn open_file(mut filename: &mut str, overwrite_file: bool) -> Result<File, Error> {
    if overwrite_file && Path::new(filename).exists() {
        File::options().create(true).write(true).open(&mut filename)
    } else {
        File::options().create_new(true).append(true).open(&mut filename)
    }
}
