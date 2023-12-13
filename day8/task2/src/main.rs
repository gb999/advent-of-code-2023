use std::{fs, collections::HashMap};
use num::integer::lcm;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let mut iter = contents.lines();
    let directions = iter.next().unwrap();
    iter.next();

    let mut map: HashMap<&str ,  (&str, &str)> = HashMap::new();
    let mut starts: Vec<&str> = Vec::new();
    for line in iter {
        map.insert(&line[0..3], (&line[7..10], &line[12..15]));
        if line.chars().nth(2) == Some('A') {
            starts.push(&line[0..3]);
        }
    }

    // Task2
    let mut res: u128 = 1;
    for start in starts {
        res = lcm(follow_path(directions, &map, start), res);
    }

    dbg!(res);
}

// Now we find loop size
fn follow_path(directions: &str, map: &HashMap<&str ,  (&str, &str)>, start: &str) -> u128 {
    let mut steps = 0;

    let mut directions = directions.chars().cycle(); 
    let mut pos = start;
    while pos.chars().nth(2) != Some('Z') {
        pos = match directions.next().unwrap() {
            'L' => map[pos].0, 
            'R' => map[pos].1, 
            _=> panic!("Invalid input!") };
        steps += 1;
    }
    println!("{steps}");
    steps
}
