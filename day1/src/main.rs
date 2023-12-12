use std::fs;
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    
    const NUMBERS : [&'static str; 9]  = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut sum : u32 = 0;
    for line in contents.lines() {
        // (number, position)
        let mut digits : Vec<(u32, u32)> = Vec::new(); 
        
        let mut i = 0;
        for num in NUMBERS {
            i += 1;
            match line.find(num) {
                Some(pos) => {digits.push((i, pos as u32)); },
                None => {}, 
            }
            match line.rfind(num) { // Need to use rfind to avoid cases like nine1nine
                Some(pos) => {digits.push((i, pos as u32)); },
                None => {}, 
            }
        }

        let mut j = 0;
        for ch in line.chars() {
            j+=1;
            match ch.to_digit(10) {
                None => continue,
                Some(digit) => {digits.push((digit, j-1)); } ,
            }
        }

        // Sort by positions
        digits.sort_by(|a, b| a.1.cmp(&b.1));

        sum += digits[0].0 * 10 + digits[digits.len() - 1].0;
    } 
    print!("{sum}");

}
