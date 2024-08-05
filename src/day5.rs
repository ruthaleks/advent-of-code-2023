use std::fs::File;
use std::io;
use std::io::BufRead;

static FILE: &str = "resources/day5_t1.txt";

pub fn solve() {
    match read_input(FILE) {
        Ok(input) => {
            //println!("{:?}", input);
            println!("{:?}", parse_maps(input));
            let res: i32 = map_interpreter(55, [(50, 98, 2), (52, 50, 48)].to_vec());
            println!("res = {}", res);
        }
        Err(err) => eprintln!("Error while reading the file: {}", err),
    }
}

// seeds [79, 14, 55, 13]
// seed-to-soil-map: [(50, 98, 2), (52, 50, 48)]

fn map_interpreter(seed: i32, maps: Vec<(i32, i32, i32)>) -> i32 {
    for (dest, source, range) in maps {
        if in_range(dest, range, seed) {
            return dest - source + seed;
        }
    }
    return seed;
}

fn in_range(dest: i32, range: i32, seed: i32) -> bool {
    seed >= dest && seed < dest + range
}

fn parse_maps(input: Vec<&str>) -> Vec<Vec<(i32, i32, i32)>> {
    let mut maps: Vec<Vec<(i32, i32, i32)>> = Vec::new();
    for i in 1..input.len() {
        println!("i = {} line = {}", i, input[i]);
        if input[i].contains("map:") {
            let mut idx = i;
            let mut map: Vec<(i32, i32, i32)> = Vec::new();
            while idx <= input.len() && !input[idx].contains("map:") {
                let submap: Vec<i32> = input[idx].split(" ").map(|s| s.parse::<i32>().unwrap()).collect();
                println!("submap = {:?}", submap);
                map.push((submap[0], submap[1], submap[2]));
                idx = idx + 1;
            }
            maps.push(map);
        }
    }
    maps
}


fn read_input(filename: &str) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let input: Vec<String> = reader.lines().filter_map(|res| match res {
        Ok(s) if !s.trim().is_empty() => Some(s),
        Ok(_) => None,
        Err(_) => panic!("Something wrong")
    }).collect::<Vec<String>>();

    
    Ok(input)
}
