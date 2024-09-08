use crate::utils;

pub fn solve() {
    let filepath = "resources/day2.txt";
    let part1 = solve_part1(filepath);
    println!("Part 1: {part1}");
    let part2 = solve_part2(filepath);
    println!("Part 2: {part2}")
}

fn solve_part1(filepath: &str) -> i32 {
    let mut out = -1;
    match utils::read_input(filepath) {
        Ok(input) => {
            let input: Vec<i32> = input
                .iter()
                .filter_map(|s| {
                    let (id, is_valid) = game_checker(s);
                    if is_valid {
                        Some(id)
                    } else {
                        None
                    }
                })
                .collect();
            out = input.iter().sum::<i32>();
        }
        Err(err) => eprintln!("Error while reading the file: {}", err),
    }
    out
}

fn solve_part2(filepath: &str) -> i32 {
    let mut out = -1;
    match utils::read_input(filepath) {
        Ok(input) => {
            let input: Vec<i32> = input.iter().map(|s| game_checker_fewest(s)).collect();
            out = input.iter().sum::<i32>();
        }
        Err(err) => eprintln!("Error while reading the file: {}", err),
    }
    out
}

fn game_checker(indata: &str) -> (i32, bool) {
    let id = get_id(indata);
    let mut games: Vec<&str> = indata.split(':').collect();
    games.remove(0);

    let sets: Vec<&str> = games[0].split(';').collect();

    let colors: Vec<(&str, i32)> = vec![("red", 12), ("blue", 14), ("green", 13)];
    for (color, limit) in colors.iter() {
        let res: Vec<bool> = sets
            .iter()
            .map(|s| check_if_valid(color, limit, s))
            .collect();
        if res.contains(&false) {
            return (id, false);
        }
    }

    (id, true)
}

fn game_checker_fewest(indata: &str) -> i32 {
    let mut games: Vec<&str> = indata.split(':').collect();
    games.remove(0);

    let sets: Vec<&str> = games[0].split(';').collect();

    let mut cubes: Vec<i32> = Vec::new();

    let colors: Vec<&str> = vec!["red", "green", "blue"];
    for (i, color) in colors.iter().enumerate() {
        let res: Vec<i32> = sets.iter().map(|s| num_of_cubes(color, s)).collect();
        cubes.insert(i, res.iter().cloned().max().expect("Fail"));
    }
    cubes.iter().product::<i32>()
}

fn check_if_valid(color: &str, limit: &i32, indata: &str) -> bool {
    if indata.contains(color) {
        let l: Vec<&str> = indata.split(' ').collect();
        let color_with_coma = format!("{},", color);
        let Some(index) = l.iter().position(|&x| x == color || x == color_with_coma) else {
            todo!()
        };
        let value: i32 = l[index - 1].parse().expect("Failed to parse number");
        return value <= *limit;
    }
    true
}

fn num_of_cubes(color: &str, indata: &str) -> i32 {
    if indata.contains(color) {
        let l: Vec<&str> = indata.split(' ').collect();
        let color_with_coma = format!("{},", color);
        let Some(index) = l.iter().position(|&x| x == color || x == color_with_coma) else {
            todo!()
        };
        let value: i32 = l[index - 1].parse().expect("Failed to parse number");
        return value;
    }
    0
}

fn get_id(indata: &str) -> i32 {
    let game: &str = indata.split(':').collect::<Vec<_>>()[0]
        .split(' ')
        .collect::<Vec<_>>()[1];
    game.parse().expect("Could not parse the number")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1("resources/day2_t1.txt"), 8);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2("resources/day2_t1.txt"), 2286);
    }
}
