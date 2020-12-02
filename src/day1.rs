use std::fs::File;
use std::io;

pub mod part1;

pub fn run(lines: io::Lines<io::BufReader<File>>) {
    // let mut input_values: Vec<(i32, i32)> = Vec::new();
    let elems: Vec<(i32, i32)> = lines.into_iter().map(
        |line| -> (i32, i32) {
            let int = line.unwrap().parse::<i32>().unwrap_or(0);
            (int, 2020 - int)
        }).collect();

    // println!("{:?}", elems);

    let cpy = elems.to_vec();
    for (i, v) in cpy {
        if elems.contains(&(v, i)) {
            println!("found ({}, {}); answer: {}", v, i, i * v);
            break;
        }
    }
    println!("woop!")
}
