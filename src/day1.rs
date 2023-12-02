use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;

fn main() {
    match read_input("resources/day1.txt") {
        Ok(input) => {
            //let values: Vec<i32> = calibration_values(input.clone());
            //let part1: i32 = values.iter().sum();
            //println!("Part 1: {}", part1);
            println!("{:?}", input);
            let out: Vec<String>= input.iter().map(|s| replace_spelled_letters(s)).collect();
            //println!("{:?}", out);
            let values: Vec<i32> = calibration_values(out);
            println!("{:?}", values);
            let part2: i32 = values.iter().sum();
            println!("Part 2: {}", part2);
        }
        Err(err) => eprintln!("Error while reading the file: {}", err)
    }
}

fn calibration_values(input: Vec<String>) -> Vec<i32> {
    let numbers: Vec<Vec<&str>>= input
        .iter()
        .map(|s| s.split("").filter(|&c| is_number(c)).collect())
        .collect();
    println!("{:?}", numbers);
    numbers.iter().map(|l| calibration_value(l.to_vec())).collect()
}

fn calibration_value(list: Vec<&str>) -> i32 {
    let val = format!("{}{}", list[0], list[list.len() - 1]);
    val.parse::<i32>().expect("Could not parse the number!")
}

fn is_number(s: &str) -> bool {
    s.parse::<i32>().is_ok()
}

fn replace_spelled_letters(input: &str) -> String {
    let letters = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut map: HashMap<&str, String> = HashMap::new();

    for (i, &letter) in letters.iter().enumerate() {
        let value = (i+1).to_string();
        map.insert(letter, value);
    }

    let mut input_str = String::from("");
    let mut order_list: Vec<(usize, &str)> = Vec::new();

    for letter in letters.iter() {
        if input.contains(letter) {
            //println!("letter = {}", letter);
            let mut start_index: usize = 0;
            while let Some((index, _)) = input[start_index..].match_indices(letter).next() {
                let absolute_index = start_index + index;
                let Some(value) = map.get(letter) else { todo!() };
                order_list.push((absolute_index, value));
                start_index = absolute_index + letter.len();
            }
            //println!("start_index={}", start_index);
            //let Some(index) = input[start_index..].find(letter) else { todo!() };
            //let Some(value) = map.get(letter) else { todo!() };
            //println!("index={}", index);
            //start_index = index;
            //order_list.push((index, value));
            //println!("value = {}", value);
            //input_str = input.replace(letter, value);
        }
    }
    order_list.sort_by(|a,b| a.0.cmp(&b.0));
    //println!("{:?}", order_list);
    let input_list: Vec<&str> = input.split("").filter(|s| !s.is_empty()).collect();
    //println!("{:?}", input_list);
    for (i, c) in input_list.into_iter().enumerate() {
      //  println!("i={}, c={}", i, c);
        if order_list.iter().any(|&(index, _)| index == i){
            let Some(&(index, value)) = order_list.iter().find(|&&(index, _)| index == i) else { todo!() };
            input_str.push(value.parse().unwrap());
        }
        input_str.push(c.parse().unwrap());
      //  println!("{:?}", input_str);
    }
    //for (index, letter) in order_list {
    //    println!("index = {}", index);
    //    println!("letter = {}", letter);
    //    println!("input_str = {}", input_str);
    //    let Some(value) = map.get(letter) else { todo!() };
    //    input_str = input.replace(letter, value);
    //}
    input_str
}


fn read_input(filename: &str) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let input: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    Ok(input)
}