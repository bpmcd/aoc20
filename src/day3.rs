use std::fs::File;
use std::io;

struct Position { x: i32, y: i32 }

pub fn run(input: io::Lines<io::BufReader<File>>) {
    let map: Vec<String> = input
        .into_iter()
        .map(|line| line.unwrap())
        .collect();

    let direction = Position { x: 3, y: 1 };
    let mut pos = Position { x: 0, y: 0 };
    let mut tree_count = 0;

    while let Ok(tree) = is_tree(&map, pos.x as usize, pos.y as usize) {
        if tree {
            tree_count += 1;
        }
        pos = move_toboggan(pos, &direction);
    }
    println!("Tree count: {}", tree_count);

    let slopes = vec!(Position { x: 1, y: 1 }, Position { x: 3, y: 1 }, Position { x: 5, y: 1 }, Position { x: 7, y: 1 }, Position { x: 1, y: 2 });
    let mut tree_counts: Vec<i64> = Vec::new();
    for slope in slopes {
        pos = Position {x: 0, y: 0};
        tree_count = 0;
        while let Ok(tree) = is_tree(&map, pos.x as usize, pos.y as usize) {
            if tree {
                tree_count += 1;
            }
            pos = move_toboggan(pos, &slope);
        }
        println!("Tree count: {}", tree_count);
        tree_counts.push(tree_count);
    }
    println!("Tree count product: {}", tree_counts.iter().product::<i64>())
}

fn is_tree(map: &Vec<String>, x: usize, y: usize) -> Result<bool, String> {
    if let Some(y_row) = &map.get(y) {
        let width = y_row.len();
        return match y_row.as_bytes()[(x % width)] as char {
            '#' => Ok(true),
            '.' => Ok(false),
            _ => Err(format!("unknown map symbol: {}", y_row.as_bytes()[x % width] as char))
        };
    }
    Err(format!("index out of bounds. y coord: {}", y))
}

fn move_toboggan(pos: Position, dir: &Position) -> Position {
    Position {
        x: pos.x + dir.x,
        y: pos.y + dir.y,
    }
}