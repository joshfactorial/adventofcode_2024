use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

type Rules = HashMap<String, Vec<String>>;
type Tests = Vec<Vec<String>>;

fn main() {
    let input_file = "/home/joshfactorial/code/inputs/inputs.txt.full";
    let lines = read_lines(input_file).unwrap();
    let (rules, tests) = parse_lines(lines);
    let (total, wrongs) = part1(&rules, &tests);
    println!("part 2 count: {}", part2(&rules, &wrongs));
}

fn parse_lines(lines: io::Lines<BufReader<File>>) -> (Rules, Tests) {
    let mut rules: Rules = HashMap::new();
    let mut pages = Vec::new();
    let mut section2 = false;
    for line in lines {
        if !section2 {
            let test = line.unwrap();
            if test == "" {
                section2 = true
            } else {
                let mut split_test = test.split("|");
                // not good code, maybe because I'm taking or granted the input is clean
                let (low, high) = (
                    split_test.next().unwrap_or("file format error").to_string(),
                    split_test.next().unwrap_or("file format error").to_string()
                );
                rules.entry(low.clone()).or_insert(Vec::new());
                rules.get_mut(&low).unwrap().push(high.clone());
            }
        } else {
            let split_line = line
                .unwrap()
                .split(",")
                .map(|m| m.to_string())
                .collect();
            pages.push(split_line);
        }
    }
    (rules, pages)
}

fn part1(rules: &Rules, tests: &Tests) -> (u32, Tests) {
    let mut final_score: u32 = 0;
    let mut wrongs = Vec::new();
    let keys_list: Vec<String> = rules.keys()
        .map(|k| k.to_owned())
        .collect();
    'outer: for list in tests {
        for i in (1..list.len()).rev() {
            let current_page = list[i].clone();
            // no applicable rules
            if !keys_list.contains(&current_page) { continue };
            let prev_pages = Vec::from(&list[..i]);
            let curr_p_list = rules[&current_page].clone();
            for page in prev_pages {
                if curr_p_list.contains(&page) {
                    // rule violation
                    wrongs.push(list.clone());
                    continue 'outer
                }
            }
        }
        // valid test
        let midpoint = list.len()/2usize;
        final_score += list[midpoint].clone().parse::<u32>().unwrap();
    }
    (final_score, wrongs)
}

fn part2(rules: &Rules, tests: &Tests) -> u32 {
    // fix bad tests then count
    let mut tally = 0;
    let keys_list: Vec<String> = rules.keys()
        .map(|k| k.to_owned())
        .collect();
    for fail in tests {
        let mut revised = fail.clone();
        let test_len = fail.len();
        'outer: loop {
            for mut i in (1..test_len).rev() {
                let current_page = revised[i].clone();
                if !keys_list.contains(&current_page) { continue };
                let prev_pages = Vec::from(&revised[..i]);
                let curr_p_list = rules[&current_page].clone();
                for (index, page) in prev_pages.iter().enumerate() {
                    if curr_p_list.contains(&page) {
                        // rule violation
                        revised[i] = page.to_owned();
                        revised[index] = current_page.to_owned();
                        i = test_len;
                        continue 'outer
                    }
                }
            }
            // tally midpoint
            let midpoint = test_len/2usize;
            tally += revised[midpoint].clone().parse::<u32>().unwrap();
            break 'outer
        }
    }
    tally
}

pub fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    // This creates a buffer to read lines
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
