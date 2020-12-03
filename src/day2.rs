use std::fs::File;
use std::io;

use regex::Regex;

#[derive(Debug)]
struct PasswordPolicy {
    letter: String,
    min_occurs: i32,
    max_occurs: i32,
    password: String,
}

pub fn run(lines: io::Lines<io::BufReader<File>>) {
    let re = Regex::new(r"(\d+)-(\d+) ([a-z]+): ([a-z]+)").unwrap();
    let policies: Vec<Option<PasswordPolicy>> = lines.into_iter().map(|line|
        re.captures(line.unwrap().as_str())
            .map(|cap| PasswordPolicy {
                letter: String::from(&cap[3]),
                min_occurs: cap[1].parse::<i32>().unwrap(),
                max_occurs: cap[2].parse::<i32>().unwrap(),
                password: String::from(&cap[4]),
            })).collect();

    // println!("policies: {:?}", policies);

    println!("valid passwords part 1: {:?}",
             policies.iter().filter(|&policy| is_valid(policy)).count());
    println!("valid passwords part 2: {:?}",
             policies.iter().filter(|&policy| is_valid2(policy)).count());
}

fn is_valid(policy: &Option<PasswordPolicy>) -> bool {
    match policy {
        Some(p) => {
            let re = Regex::new(p.letter.as_str()).unwrap();
            let matches = re.find_iter(p.password.as_str()).count();
            p.min_occurs as usize <= matches && matches <= p.max_occurs as usize
        }
        None => false
    }
}

fn is_valid2(policy: &Option<PasswordPolicy>) -> bool {
    match policy {
        Some(p) => {
            let first_char = p.password.chars().nth((p.min_occurs - 1) as usize);
            let last_char = p.password.chars().nth((p.max_occurs - 1) as usize);
            let policy_char = p.letter.chars().nth(0);
            
            if policy_char.is_none() {
                println!("unkown letter: {}", p.letter);
                false
            } else {
                (first_char == policy_char && last_char != policy_char) || (first_char != policy_char && last_char == policy_char)
            }
        }
        None => {
            println!("Unable to parse a password policy entry.");
            false
        }
    }
}
