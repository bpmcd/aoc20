

pub fn find_entry_product(elems: Vec<i32>) {
    let pairs: Vec<(i32, i32)> = elems.into_iter().map(|e: i32| (e, 2020 - e)).collect();
    let cpy = pairs.to_vec();
    for (i, v) in cpy {
        if pairs.contains(&(v, i)) {
            println!("found ({}, {}); answer: {}", v, i, i * v);
            break;
        }
    }
}
