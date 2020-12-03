use std::fs::File;
use std::io;

pub mod part1;
pub mod part2;

pub fn run(lines: io::Lines<io::BufReader<File>>, members: i32) {
    let elems: Vec<i32> = lines.into_iter()
        .map(|line| line.unwrap().parse::<i32>().unwrap_or(0))
        .collect();
    match members {
        2 => {
            part1::find_entry_product(elems)
        },
        3 => {
            part2::find_entry_product(elems)
        },
        _ => {
            println!("ERROR! Invalid number of members: {}", members);
        }
    }

    println!("woop!")
}
