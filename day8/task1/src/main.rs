use std::{fs, collections::HashMap};

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let mut iter = contents.lines();
    let directions = iter.next().unwrap();
    iter.next();

    let mut map: HashMap<&str ,  (&str, &str)> = HashMap::new();
    for line in iter {
        map.insert(&line[0..3], (&line[7..10], &line[12..15]));
    }

    // Task1 
    let task1 = follow_path(directions, &map, "AAA", "ZZZ");
    dbg!(task1);

}

fn follow_path(directions: &str, map: &HashMap<&str ,  (&str, &str)>, start: &str, end:&str) -> usize {
    let mut steps = 0;
    let L =directions.len();

    let mut directions = directions.chars().cycle(); 
    let mut pos = start;
    while pos != end {
        pos = match directions.next().unwrap() {
            'L' => map[pos].0, 
            'R' => map[pos].1, 
            _=> panic!("Invalid input!") };
        steps += 1;
    }
    steps
}
