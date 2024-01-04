use std::fs::File;
use std::io;
use std::io::BufRead;

static FILE: &str = "resources/day6.txt";

pub fn solve() {
    match read_input(FILE) {
        Ok(input) => {
            println!("{:?}", input);
            let part1: Vec<i128> = input.iter().map(|&x| calc_options(x)).collect();
            println!("Part 2 = {:?}", part1.iter().product::<i128>());
        }
        Err(err) => eprintln!("Error while reading the file: {}", err),
    }
}

fn calc_options((time, record): (i128, i128)) -> i128 {
    println!("({},{})", time, record);
    let mut out: i128 = 0;
    for t in 0..time+1 {
        let remaining_time: i128 = time - t;
        let distance = remaining_time * t;
        if distance > record {
            out = out + 1;
        }
    }
    out
}

fn read_input(filename: &str) -> io::Result<Vec<(i128, i128)>> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let raw: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    let time: Vec<i128> = raw[0].split(" ").filter(|s| !s.is_empty() && !s.starts_with("Time")).map(|s| s.parse::<i128>().unwrap()).collect();
    let distance: Vec<i128> = raw[1].split(" ").filter(|s| !s.is_empty() && !s.starts_with("Distance")).map(|s| s.parse::<i128>().unwrap()).collect();
    let merged: Vec<(i128, i128)> = time.into_iter().zip(distance.into_iter()).collect();

    Ok(merged)
}
