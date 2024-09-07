use std::collections::HashMap;

use crate::utils;

pub fn solve() {
    let filepath = "resources/day1.txt";
    let part1 = solve_part1(filepath);
    println!("Part 1: {part1}");
    let part2 = solve_part2(filepath);
    println!("Part 2: {part2}")
}

fn solve_part1(filename: &str) -> i32 {
    let mut out = -1;
    match utils::read_input(filename) {
        Ok(input) => {
            let values: Vec<i32> = calibration_values(&input);
            out = values.iter().sum();
        }
        Err(err) => eprintln!("Error while reading the file: {}", err),
    }
    out
}

fn solve_part2(filename: &str) -> i32 {
    let mut out = -1;
    match utils::read_input(filename) {
        Ok(input) => {
            let input: Vec<String> = input.iter().map(|s| replace_spelled_letters(s)).collect();
            let values: Vec<i32> = calibration_values(&input);
            out = values.iter().sum();
        }
        Err(err) => eprintln!("Error while reading the file: {}", err),
    }
    out
}

fn calibration_values(input: &[String]) -> Vec<i32> {
    let numbers: Vec<Vec<&str>> = input
        .iter()
        .map(|s| s.split("").filter(|&c| is_number(c)).collect())
        .collect();

    numbers
        .iter()
        .map(|l| calibration_value(l.to_vec()))
        .collect()
}

fn calibration_value(list: Vec<&str>) -> i32 {
    let val = format!("{}{}", list[0], list[list.len() - 1]);
    val.parse::<i32>().expect("Could not parse the number!")
}

fn is_number(s: &str) -> bool {
    s.parse::<i32>().is_ok()
}

fn replace_spelled_letters(input: &str) -> String {
    let numbers: HashMap<&str, &str> = [
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]
    .iter()
    .cloned()
    .collect();

    let mut out = String::from("");
    let mut order_list: Vec<(usize, &str)> = Vec::new();

    for number in numbers.keys() {
        if input.contains(number) {
            let mut start_index: usize = 0;
            while let Some((index, _)) = input[start_index..].match_indices(number).next() {
                let absolute_index = start_index + index;
                let Some(value) = numbers.get(number) else {
                    panic!("Error")
                };
                order_list.push((absolute_index, value));
                start_index = absolute_index + number.len();
            }
        }
    }
    order_list.sort_by(|a, b| a.0.cmp(&b.0));
    let input_list: Vec<&str> = input.split("").filter(|s| !s.is_empty()).collect();
    for (i, c) in input_list.into_iter().enumerate() {
        if order_list.iter().any(|&(index, _)| index == i) {
            let Some(&(_, value)) = order_list.iter().find(|&&(index, _)| index == i) else {
                panic!("Error")
            };
            out.push(value.parse().unwrap());
        }
        out.push(c.parse().unwrap());
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1("resources/day1_t1.txt"), 142);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2("resources/day1_t2.txt"), 281);
    }
}
