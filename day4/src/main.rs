use std::{fs};

fn parse_numbers(slice: &str) -> Vec<u32> {
    let mut current_num = 0;
    let mut res: Vec<u32> = Vec::new();
    for ch in slice.chars() {
        if let Some(dig) = ch.to_digit(10) {
            current_num *= 10;
            current_num += dig;
        } else {
            if current_num != 0 {
                res.push(current_num);
                current_num = 0;

            }
        }
    }
    res.push(current_num); // push last element

    res
}
fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read file...");
    

    let N = contents.lines().count();
    let mut cards: Vec<u128> = Vec::with_capacity(N);
    for _ in 0..N {
        cards.push(0);
    }
    
    let mut count: u128 = 0;
    for (l,line) in contents.lines().enumerate() {
        count += 1 + cards[l] ; // count original card + copies
        let winning = parse_numbers(line[10..40].trim());
        let numbers = parse_numbers(line[42..].trim());

        let mut matches = 0;
        for (i, n) in winning.iter().enumerate() {
            if numbers.contains(n) {
                matches += 1;
            }
        }

        for i in 1..=matches {
            cards[l + i] += 1 + cards[l];
        }
    }
   

    println!("{:?}", count);
}
