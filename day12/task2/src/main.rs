use std::{fs, collections::{HashSet, HashMap}};
fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read file.");

        let mut sum = 0;
    for line in contents.lines() {
        
        let (states, numbers) = parse_line(line);

        sum += count_arrangements_wrapper(&states, &numbers);
    }
    println!("{sum}");
}


#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum SpringState {
    Operational,
    Damaged,
    Unknown,
}

impl SpringState {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Operational,
            '#' => Self::Damaged,
            '?' => Self::Unknown,
            _ => panic!("Expected . ? or #"),
        }
    }
}

fn parse_line(line: &str) -> (Box<[SpringState]>, Box<[usize]>) {
    let mut iter = line.split(" ");
    let spring_states: Vec<SpringState> = iter
        .next()
        .unwrap()
        .chars()
        .map(SpringState::from_char)
        .collect();

    let expected_numbers: Vec<usize> = iter
        .next()
        .unwrap()
        .split(",")
        .map(|num| num.parse::<usize>().expect("Expected valid input"))
        .collect();

    let spring_states = [
        spring_states.clone(), 
        spring_states.clone(), 
        spring_states.clone(), 
        spring_states.clone(), 
        spring_states.clone(),].join(&SpringState::Unknown);

    let expected_numbers = [
        expected_numbers.clone(),
        expected_numbers.clone(),
        expected_numbers.clone(),
        expected_numbers.clone(),
        expected_numbers.clone()
    ].concat();
    (spring_states.into_boxed_slice(), expected_numbers.into_boxed_slice())
}


fn count_arrangements_wrapper(states: &[SpringState], numbers: &[usize]) -> u128 {
    let mut cache: HashMap<(&[SpringState], &[usize]), u128> = HashMap::new();
    count_arrangements(states, numbers, &mut cache)
}

// Stolen from https://www.youtube.com/watch?v=g3Ms5e7Jdqo
fn count_arrangements<'a>(states: &'a [SpringState], numbers: &'a [usize], cache: &mut HashMap<(&'a [SpringState], &'a [usize]), u128>) -> u128 {
    if states.len() == 0 {
        if numbers.len() == 0 { return 1 }
        else {return 0 }
    }
    if numbers.len() == 0 {
        if states.contains(&SpringState::Damaged) {return  0 }
        return 1 
    }
    let key = &(states, numbers);
    if cache.contains_key(key) {
        return cache[key];
    }

    let mut count = 0;
    // . or ? (? means we suppose a . is in its place) 
    if states[0] != SpringState::Damaged { 
        count += count_arrangements(&states[1..], numbers, cache);
    }

    // # or ? (? means we suppose a # is in its place) 
    if states[0] != SpringState::Operational {
        if numbers[0] <= states.len() 
        && !states[..numbers[0]].contains(&SpringState::Operational) 
        && (numbers[0] == states.len() 
            || states[numbers[0]] != SpringState::Damaged) {
            // Avoid indexing beyond end of array
            let states_idx = if states.len() == numbers[0] {numbers[0]} else {numbers[0] + 1};
            count += count_arrangements(&states[states_idx..], &numbers[1..], cache)
        }
    }
    cache.insert(key.clone(),count);
    count
}