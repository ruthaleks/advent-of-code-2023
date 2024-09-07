use std::fs::File;
use std::io;
use std::io::BufRead;

pub fn read_input(filename: &str) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let input: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    Ok(input)
}
