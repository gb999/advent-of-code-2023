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

fn find_reflection_horizontal(block: &Vec<&str>) -> Option<usize> {
    // let lines: Vec<&str> = block;    

    'outer: for (i, _line) in block.iter().enumerate().skip(1) { // choose mirror pos
        let mirror_pos = min(block.len() - i, i); 

        for j in 1..=mirror_pos { // check if reflections are correct
            if block[i + j - 1] != block[i - j] {
                continue 'outer;
            }
        }
        return Some(i);
    }
    None
}

fn find_reflection_vertical(block: &Vec<&str>) -> Option<usize> {
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

    'outer: for (i, _line) in lines.iter().enumerate().skip(1) { // choose mirror pos
        let mirror_pos = min(lines.len() - i, i); 

        for j in 1..=mirror_pos { // check if reflections are correct
            if lines[i + j - 1] != lines[i - j] {
                continue 'outer;
            }
        }
        return Some(i);
    }
    None
}