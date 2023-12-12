use std::fs;
const FILE_PATH : &'static str = "input.txt";


fn char_at(i: usize, str: &String) -> char {
    str.as_bytes()[i] as char
}


// Parses number from string: e.g: a123v
//                                   ^
//                                   i
// Returns 123
// i must point to a digit
fn parse_num_at(index: usize, line: &String) -> u32 {
    let mut i = index;
    // find start index of number
    while i > 0 && char_at(i - 1, line).is_ascii_digit() {
        i -= 1;
    }
    let mut num = char_at(i, line).to_digit(10).unwrap();
    i+=1;
    while i < line.len() && char_at(i, line).is_ascii_digit() {
        num *= 10;
        num += char_at(i, line).to_digit(10).unwrap();
        i+=1;
    }
    num
}

// if character is a gear return value, else 0
fn get_gear_value(index: usize, line: &String, prev_line: Option<&String>, next_line: Option<&String>) -> u32 {
    // Digits of the same number count as one number
    // Count numbers around * (potential gear)
    let mut count = 0;
    // Store them in a Vec
    let mut numbers: Vec<u32> = Vec::new();

    let i1 = if index > 0 {index - 1} else {0};
    let i2 = if index < line.len() - 1 {index + 1} else {line.len() - 1};

    for i in i1..=i2 {
        if char_at(i, line).is_ascii_digit() {
            count += 1;
            numbers.push(parse_num_at(i, line));
        };
    } 
    
    if let Some(str) = prev_line {
        let mut i = i1;
        while i <= i2 {
            if char_at(i, str).is_ascii_digit() {
                count += 1;
                numbers.push(parse_num_at(i, str));
                // Skip digits if they belong to the same number                   
                while char_at(i, str).is_ascii_digit() && i <= i2 { 
                    i += 1;
                }
            } else {
                i += 1;
            }
        } 
    }

    if let Some(str) = next_line {
        let mut i = i1;
        while i <= i2 {
            if char_at(i, str).is_ascii_digit() {
                count += 1;
                numbers.push(parse_num_at(i, str));
                // Skip digits if they belong to the same number                   
                while char_at(i, str).is_ascii_digit() && i <= i2 { 
                    i += 1;
                }
            } else {
                i += 1;
            }
        } 
    }
    if count != 2 {
        return  0;
    }
    println!("{},{}", numbers[0], numbers[1]);
    return numbers[0] * numbers[1];
    
}
fn main() {
    let contents = fs::read_to_string(FILE_PATH).unwrap();
    let mut sum:u64 = 0;

    let mut prev_line: Option<&String>;
    let mut next_line: Option<&String>;
    let lines: Vec<String> = contents.lines().map(|s|s.to_string()).collect();

    for (l, line) in lines.iter().enumerate() {
        prev_line = if l > 0 {lines.get(l - 1)} else {None};
        next_line = if l < lines.len() - 1  {lines.get(l + 1)} else {None};
        for i in 0..line.len() {
            if char_at(i, line) == '*' {
                // println!("{i}");
                sum += get_gear_value(i, line, prev_line, next_line) as u64;
            }
        }
    }

    println!("{sum}");
}
