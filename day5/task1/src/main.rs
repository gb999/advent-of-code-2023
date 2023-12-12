use std::{fs, ops::Range};
use regex::Regex;

/**
 * Parses numbers from a line containing numbers.
 * Behaviour when given empty string is undefined.
 */
fn parse_numbers(slice: &str) -> Vec<u64> {
    let mut res = Vec::new(); 
    let re = Regex::new(r"[0-9]+").unwrap();
    let mut i = 0;
    let mut current_slice = slice;
    while i < slice.len() {
        if let Some(num) = re.find(current_slice) {
            res.push(num.as_str().parse::<u64>().unwrap());
            
            i += num.end();
            current_slice = &slice[i..];
        } else { break; };
    }
    res
}



#[derive(Debug)]
struct Mapping { // Store a bunch of redundant data
    src_ranges: Vec<Range<u64>>,
    dst_starts: Vec<u64>,
    lengths: Vec<u64>,
}

impl Mapping {
    fn new() -> Self {
        Mapping { src_ranges: Vec::new(), dst_starts: Vec::new(), lengths: Vec::new() }
    }
    fn add(&mut self, src_start: u64, dst_start: u64, len: u64) {
        self.src_ranges.push(src_start..(src_start+len));
        self.dst_starts.push(dst_start);
        self.lengths.push(len);
    }
    fn map_value(&self, val: &mut u64) {
        for (i, range) in self.src_ranges.iter().enumerate() {
            if range.contains(val) {
                let offset = *val - range.start;
                *val = self.dst_starts[i] + offset;
                return;
            }
        }
    }

}



fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read file...");
    let mut lines = contents.lines();
    let mut seeds = parse_numbers(lines.next().unwrap());

    let mut mappings: Vec<Mapping> = Vec::new();

    let mut current_mapping = Mapping::new();
    for (_, line) in lines.enumerate() {
        if line == "" { // Empty line means new mapping
            mappings.push(current_mapping);
            current_mapping = Mapping::new();
        } else {
            let res = parse_numbers(&line);
            if res.len() == 3 { // Not nice
                current_mapping.add(res[1], res[0], res[2]);
            }            
        }
    }
    mappings.push(current_mapping);

    for mapping in mappings {
        for seed in &mut seeds {
            mapping.map_value(seed);
        }
    }
     println!("{}", seeds.iter().min().unwrap());

    
}
