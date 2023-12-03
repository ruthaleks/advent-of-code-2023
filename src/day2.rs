use std::fs::File;
use std::io;
use std::io::BufRead;

fn main() {
    match read_input("resources/day2.txt") {
        Ok(input) => {
            println!("{:?}", input);
            //let res: Vec<i32> = input
             //   .iter()
             //   .filter_map(|s| {
             //       let (id, is_valid) = game_check_part1(&s);
             //       if is_valid {
             //           Some(id)
             //       } else {
            //            None
            //        }
            //    }).collect();
            //println!("{:?}", res);
            //println!("part 1 = {}", res.iter().sum::<i32>())

            let out: Vec<i32> = input.iter().map(|s| game_check_part2(s)).collect();
            println!("out = {:?}", out.iter().sum::<i32>());


        }
        Err(err) => eprintln!("Error while reading the file: {}", err)
    }
}

// "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
fn game_check_part1(indata: &str) -> (i32, bool) {
    let id = get_id(indata);
    let mut games:Vec<&str> = indata.split(":").collect();
    games.remove(0);
    println!("games = {:?}", games);

    let sets: Vec<&str> = games[0].split(";").collect();
    println!("sets = {:?}", sets);

    let colors: Vec<(&str, i32)> = vec![("red", 12), ("blue", 14), ("green", 13)];
    for (color, limit) in colors.iter() {
        let res: Vec<bool> = sets.iter().map(|s| check_if_valid(color, limit, s)).collect();
        if res.contains(&false) {
            return (id, false)
        }
        println!("res = {:?}", res);
    }

    (id, true)
}


// "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
fn game_check_part2(indata: &str) -> i32 {
    let mut games:Vec<&str> = indata.split(":").collect();
    games.remove(0);
    println!("games = {:?}", games);

    let sets: Vec<&str> = games[0].split(";").collect();
    println!("sets = {:?}", sets);

    let mut cubes: Vec<i32> = Vec::new();

    let colors: Vec<&str> = vec!["red", "green", "blue"];
    for (i, color) in colors.iter().enumerate() {
        let res: Vec<i32> = sets.iter().map(|s| num_of_cubes(color, s)).collect();
        cubes.insert(i, res.iter().cloned().max().expect("Fail"));
        println!("res = {:?}", res);
        println!("cubes = {:?}", cubes);
    }

    cubes.iter().fold(1, |acc, &x| acc * x)
}

//"3 blue, 4 red"
fn check_if_valid(color: &str, limit: &i32, indata: &str) -> bool {
    if indata.contains(color) {
        println!("color = {}", color);
        let l:Vec<&str> = indata.split(" ").collect();
        println!("l = {:?}", l);
        let color_with_coma = format!("{},", color);
        let Some(index) = l.iter().position(|&x| x == color || x == color_with_coma ) else { todo!() };
        let value: i32 = l[index-1].parse().expect("Failed to parse number");
        println!("index = {}", index);
        println!("value = {}", value);
        return value <= *limit
    }
    true
}

fn num_of_cubes(color: &str, indata: &str) -> i32 {
    if indata.contains(color) {
        println!("color = {}", color);
        let l:Vec<&str> = indata.split(" ").collect();
        println!("l = {:?}", l);
        let color_with_coma = format!("{},", color);
        let Some(index) = l.iter().position(|&x| x == color || x == color_with_coma ) else { todo!() };
        let value: i32 = l[index-1].parse().expect("Failed to parse number");
        println!("index = {}", index);
        println!("value = {}", value);
        return value
    }
    0
}

//"Game 1"
fn get_id(indata: &str) -> i32 {
    let game: &str = indata.split(":").collect::<Vec<_>>()[0].split(" ").collect::<Vec<_>>()[1];
    game.parse().expect("Could not parse the number")
    //println!("game = {:?}", game);

}

fn read_input(filename: &str) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let input: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    Ok(input)
}