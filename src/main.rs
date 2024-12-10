use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::ops::{Add, Mul};
use std::io::{BufRead, BufReader, Write};

fn add_expr(lists: Vec<Vec<std::ops>>) -> Vec<Vec<std::ops>> {
    let mut daughter_lists = Vec::new();
    for i in 0..lists.len() {
        let mut daughter = lists[i].clone();
        daughter.push(usize::add);
        daughter_lists.push(daughter.clone());
        let mut daughter = lists[i].clone();
        daughter.push(usize::mul);
        daughter_lists.push(daughter.clone())
    }
    daughter_lists
}

fn main() {
    let file = read_lines("/home/joshfactorial/code/inputs/inputs.txt").unwrap();
    let file_map = parse_lines(file);
    for (key, vector_list) in file_map {
        let num_expr = vector_list.len() - 1;
        let mut expr_lists = Vec::with_capacity(2_usize.pow(num_expr as u32));
        let expr_row = Vec::new();
        expr_lists.push(expr_row);
        let daughter_expr = add_expr(expr_lists);

        println!("daughter list: {:?}", daughter_expr)
    }
}

fn parse_map(file_map: HashMap<usize, Vec<usize>>) -> usize {
    let v = 0;
    for (key, value) in file_map {
        if value.len() < 2 { continue }
        let mut tally = 0;
        let num_ops = value.len() - 1;
    }
    v
}

fn parse_lines(lines: io::Lines<BufReader<File>>) -> HashMap<usize, Vec<usize>> {
    let mut value_map: HashMap<usize, Vec<usize>> = HashMap::new();
    for line in lines {
        let safe_line = line.unwrap().clone();
        let k_v_split: Vec<&str> = safe_line.split(":").collect::<Vec<&str>>().clone();
        let key = k_v_split[0].parse::<usize>().unwrap();
        let value_list: Vec<usize> = k_v_split[1]
            .split_whitespace()
            .map(|v| v.parse::<usize>().unwrap())
            .collect();
        value_map.insert(key, value_list);
    }
    value_map
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
    fn test_setup() {
        let file = read_lines("/home/joshfactorial/code/inputs/inputs.txt");
        let input = parse_lines(file.unwrap());
        assert_eq!(Vec::from([10, 19]), input[&190]);
        assert_eq!(Vec::from([11, 6, 16, 20]), input[&292]);
    }
    // ((9 + 7) + 18) + 13
    // ((9 * 7) + 18) + 13
    // ((9 + 7) * 18) + 13
}