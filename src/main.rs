use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;
mod day1;
mod day2;
mod day3;


fn main() {
    let args: Vec<String> = env::args().collect();

    // File hosts must exist in current path before this produces output
    if let Ok(input) = read_lines(&args[2]) {
        match args[1].as_str() {
            "1" => {
                day1::run(input, args[3].parse::<i32>().expect("Unable to parse members argument"))
            },
            "2" => {
                day2::run(input)
            }
            "3" => {
                day3::run(input)
            }
            _ => {
                println!("wrong function name");
            }
        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

