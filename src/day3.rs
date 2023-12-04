use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::ops::Add;

fn main() {
    match read_input("resources/day3.txt") {
        Ok(input) => {
            println!("{:?}", input);
            let (map, values, mut gear_map) = parse_input(input);
            for &v in values.iter() {
                populate_gear_map(v, &mut gear_map)
            }
            let mut gear_ratios: Vec<i32> = Vec::new();
            for (_, val) in gear_map.iter() {
                //let val: Vec<i32> = gear_map.get(c).unwrap().clone();
                if val.len() == 2 {
                    gear_ratios.push(val[0] * val[1]);
                }
            }
            println!("gear_map={:?}", gear_map);
            let part_number_structs: Vec<(i32, (i32, i32), i32)> = values.iter().filter(|&&v| !check_adjacent(v, map.clone())).cloned().collect();
            let part_numbers: Vec<i32> = part_number_structs.iter().map(|&v| get_number(v)).collect();
            println!("part 1: {:?}", part_numbers.iter().sum::<i32>());
            println!("part 2: {}", gear_ratios.iter().sum::<i32>());
        }
        Err(err) => eprintln!("Error while reading the file: {}", err)
    }
}

fn populate_gear_map((value, (x, y), len): (i32, (i32, i32), i32), map: &mut HashMap<(i32, i32), Vec<i32>>) {
    let adj: Vec<(i32, i32)> = get_adjacent((value, (x, y), len));

    for c in adj.clone().into_iter() {
        if map.contains_key(&c) {
            let mut values: Vec<i32> = map.get(&c).unwrap().clone();
            values.push(value);
            map.insert(c, values);
        }
    }
}

//(467, (0, 0), 3)
fn check_adjacent((value, (x, y), len): (i32, (i32, i32), i32), mut map: HashMap<(i32, i32), String>) -> bool {
    let adj: Vec<(i32, i32)> = get_adjacent((value, (x, y), len));

    for c in adj.clone().into_iter() {
        let sign = map.entry(c).or_insert(".".parse().unwrap());
        if sign != "." {
            return false
        }
    }
    true
}

//(467, (0, 0), 3)
fn get_adjacent((_, (x, y), len): (i32, (i32, i32), i32)) -> Vec<(i32, i32)>{
    let mut out: Vec<(i32, i32)> = Vec::new();
    // left
    out.push((x-1, y-1));
    out.push((x-1, y));
    out.push((x-1, y+1));
    for i in 0..len {
        // above
        out.push((x+i,y-1));
        //below
        out.push((x+i,y+1));
    }
    // right
    out.push((x+len, y-1));
    out.push((x+len, y));
    out.push((x+len, y+1));

    out
}

fn get_number((value, (_, _), _): (i32, (i32, i32), i32)) -> i32 {
    value
}

fn parse_input(indata: Vec<String>) -> (HashMap<(i32, i32), String>,  Vec<(i32, (i32, i32), i32)>, HashMap<(i32, i32), Vec<i32>>){
    let mut map: HashMap<(i32, i32), String> = HashMap::new();
    let mut gear_mao: HashMap<(i32, i32), Vec<i32>> = HashMap::new();
    let mut values: Vec<(i32, (i32, i32), i32)> = Vec::new(); // value, (x, y), len
    for (y, line) in indata.iter().enumerate() {
        //println!("line={}", line);
        let row: Vec<&str> = line.split("").filter(|&s| !s.is_empty()).collect();
        //println!("y={}",y);
        //println!("row={:?}", row);
        for (x, c) in row.clone().into_iter().enumerate() {
          //  println!("x={}, c={}", x, c);
            let coord: (i32, i32) = (x.try_into().unwrap(),y.try_into().unwrap());
            map.insert(coord, c.parse().unwrap());
            if is_number(c) {
                match parse_number(row.clone(), x) {
                    Some((value, len)) => {
                        values.push((value, coord, len))
                    },
                    _ => {}
                }
            } else if c == "*" {
                gear_mao.insert(coord, Vec::new());
            }
        }
    }
    (map, values, gear_mao)
}
//["4", "6", "7", ".", ".", "1", "1", "4", ".", "."] -> (value, len)
fn parse_number(indata: Vec<&str>, start_index: usize) -> Option<(i32, i32)> {
    if start_index > 0 && is_number(indata[start_index-1]) {
        return None;
    }
    let mut value: String = String::new();
    value.push(indata[start_index].parse().unwrap());
    let mut i: usize = start_index+1;
    while i < indata.len() && is_number(indata[i]) {
        value.push(indata[i].parse().unwrap());
        i = i + 1;
    }
    Some((value.parse().unwrap(), value.len() as i32))
}

fn is_number(s: &str) -> bool {
    s.parse::<i32>().is_ok()
}

fn read_input(filename: &str) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let input: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    Ok(input)
}