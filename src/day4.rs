use std::fs::File;
use std::io;
use std::io::BufRead;

static FILE: &str = "resources/day4.txt";

pub fn solve() {
    match read_input(FILE) {
        Ok(input) => {
            let common_numbers: Vec<Vec<i32>> = input.iter().map(|s| into_vec(s)).collect();
            let part1: Vec<i32> = common_numbers
                .iter()
                .map(|x| winning_point(x.to_vec()))
                .collect();
            println!("Part 1 = {:?}", part1.iter().sum::<i32>());
            let part2: i32 = copy_cards(common_numbers);
            println!("Part 2 = {}", part2);
        }
        Err(err) => eprintln!("Error while reading the file: {}", err),
    }
}

fn copy_cards(indata: Vec<Vec<i32>>) -> i32 {
    let size = indata.len();
    let mut copies = vec![0; size];
    for i in 0..indata.len() {
        let card: &Vec<i32> = &indata[i];
        let start = i+1;
        for y in start..start+card.len() {
            if y < size {
                copies[y] = copies[y] + 1 * (copies[i] + 1);
            }
        }
    }
    copies.iter().sum::<i32>() + size as i32
}

fn into_vec(indata: &String) -> Vec<i32> {
    let two_lists: Vec<&str> = indata.split("|").collect();
    let mut first_list: Vec<&str> = two_lists[0].split(" ").filter(|s| !s.is_empty()).collect();
    let _card_number = first_list[1].replace(":", "").parse::<i32>().unwrap();
    first_list.remove(0);
    first_list.remove(0);
    let second_list: Vec<&str> = two_lists[1].split(" ").filter(|s| !s.is_empty()).collect();
    let common_numbers = find_commons(first_list, second_list);
    common_numbers
}

fn find_commons(list1: Vec<&str>, list2: Vec<&str>) -> Vec<i32> {
    let commons: Vec<&str> = list1
        .iter()
        .filter(|&s| list2.contains(s))
        .cloned()
        .collect();
    commons.iter().map(|s| s.parse().unwrap()).collect()
}

fn winning_point(common_numbers: Vec<i32>) -> i32 {
    if common_numbers.is_empty() {
        return 0;
    }
    let mut score: i32 = 1;
    for _ in 0..common_numbers.len() - 1 {
        score = score * 2;
    }
    return score;
}

fn read_input(filename: &str) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let input: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    Ok(input)
}
