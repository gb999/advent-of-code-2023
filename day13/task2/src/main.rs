use std::{fs, cmp::min};
fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    let mut sum = 0;
    
    let mut block: Vec<&str> = Vec::new();

    for line in contents.lines() { 
        if line == "" {
            let mut a = false;
            let mut b = false;
            if let Some(hor) = find_reflection_horizontal(&block) {
                sum += hor * 100; 
            } else {a = true}
            if let Some(ver) = find_reflection_vertical(&block) {
                sum += ver;
            } else {b = true}

            if a && b { println!("{:?}", block)};

            
            block.clear();
        } else {
            block.push(line);
        }    
    }
    dbg!(sum);
        
}

fn fixable(s1: &str, s2: &str) -> bool {
    s1.chars().zip(s2.chars()).filter(|(c1, c2)| *c1 != *c2).count() == 1
}

fn find_reflection_horizontal(block: &Vec<&str>) -> Option<usize> {
    'outer: for (i, _line) in block.iter().enumerate().skip(1) { // choose mirror pos
        let mut was_fixed = false;
        let mirror_pos = min(block.len() - i, i); 

        'inner: for j in 1..=mirror_pos { // check if reflections are correct
            if block[i + j - 1] != block[i - j] {
                if !was_fixed && fixable(block[i + j - 1], block[i - j]) { 
                    // treat exactly one mirror as fixed 
                    was_fixed = true;
                    continue 'inner;
                } else {
                    continue 'outer;
                }
            } 
        }
        if was_fixed {
            return Some(i);
        } 
    }
    None
}

fn find_reflection_vertical(block: &Vec<&str>) -> Option<usize> {
    // convert vertical lines into horizontal
    let mut lines: Vec<Vec<char>> = Vec::new();
    
    let l = block.iter().next().unwrap().len();

    for _ in 0..l {
        lines.push(Vec::new());
    }
    

    for (_, line) in block.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            lines[j].push(c);
        }
    }
    let lines = lines.iter().map(|v| v.iter().collect::<String>()).collect::<Vec<_>>();

    let lines = lines.iter().map(|s| &s[..]).collect::<Vec<_>>();
    return  find_reflection_horizontal(&lines);
    
}