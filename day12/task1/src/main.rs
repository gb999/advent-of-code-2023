use std::{fs, cell::RefCell, rc::Rc};

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read file.");

        let mut sum = 0;
    for line in contents.lines() {
        let data = parse_line(line);
        // brute force
        sum += count_valid_spring_arrangements(data);
    }
    println!("{sum}");
}


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

fn parse_line(line: &str) -> (Vec<SpringState>, Vec<u32>) {
    let mut iter = line.split(" ");
    let spring_states: Vec<SpringState> = iter
        .next()
        .unwrap()
        .chars()
        .map(SpringState::from_char)
        .collect();

    let expected_numbers: Vec<u32> = iter
        .next()
        .unwrap()
        .split(",")
        .map(|num| num.parse::<u32>().expect("Expected valid input"))
        .collect();

    (spring_states, expected_numbers)
}

fn count_valid_spring_arrangements(
    (spring_states, expected_numbers): (Vec<SpringState>, Vec<u32>)
) -> u32 
{
    let spring_states = spring_states.clone();
    let mut unknown_spring_indexes: Vec<(usize, _)> = spring_states.iter().enumerate()
        .filter(|s| *s.1 == SpringState::Unknown)
        .collect();

    // Produce every arrangement, count valid
    let mut count: u32 = 0; 

    let n = unknown_spring_indexes.len();
    // Use nested loops to iterate through all possible combinations
    for i in 0..2_usize.pow(n as u32) {
        let mut arrangement = spring_states.clone(); 

        for j in 0..n {
            if i & (1 << j) == 0 {
                arrangement[unknown_spring_indexes[j].0] = SpringState::Damaged;
            } else {
                arrangement[unknown_spring_indexes[j].0] = SpringState::Operational;
            }
        }
        // Process the generated combination
        if is_arrangement_valid((&arrangement, &expected_numbers)) { count +=1; }
    }

    count
}

fn is_arrangement_valid(
    (spring_states, expected_numbers): (&Vec<SpringState>, &Vec<u32>)
) -> bool 
{
    let mut current_num: u32 = 0;
    let mut actual_numbers: Vec<u32> = Vec::new();
    
    for state in spring_states {
        match state {
            SpringState::Operational => {
                if current_num != 0 {
                    actual_numbers.push(current_num);
                }
                current_num = 0;
                
            },
            SpringState::Damaged => {
                current_num += 1;
            },
            SpringState::Unknown => panic!("Expected an arrangement"),
        }
    }
    // Add last element
    if current_num != 0 {
        actual_numbers.push(current_num);
    }
    
    *expected_numbers == actual_numbers
} 