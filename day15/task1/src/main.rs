fn main() {
    // let contents = std::fs::read_to_string("example.txt").unwrap();
    let contents = std::fs::read_to_string("input.txt").unwrap();
    let strings: Vec<&str> = contents.split(",").collect();

    let mut sum = 0;
    for s in strings {
        let mut wordsum = 0;
        for ch in s.chars() {
            wordsum = hash(ch, wordsum);
        }
        sum += wordsum;
    }
    dbg!(sum);
}


fn hash(ch: char, start: u128) -> u128 {
    (start + ch as u128) * 17 % 256
}