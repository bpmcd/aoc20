use std::collections::HashSet;
use std::iter::FromIterator;

pub fn find_entry_product(elems: Vec<i32>) {
    let set = HashSet::<i32>::from_iter(elems.iter().copied());

    if let Some(result) = set.iter().find_map(|first| {
        set.iter()
            .filter(|&second| first != second)
            .find_map(|second| set.get(&(2020 - first - second)).map(|third| first * second * third))
    }) {
        println!("Answer: {}", result);
    }
}
