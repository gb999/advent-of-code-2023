use rusttype::{Point, Vector};
use std::collections::VecDeque;
fn main() { 

    let contents = std::fs::read_to_string("input.txt").unwrap();

    let mut matrix:Vec<Vec<char>> = Vec::new();
    for (i, line) in contents.lines().enumerate() {
        matrix.push(Vec::new());
        for c in line.chars()  {
            matrix[i].push(c);    
        }
    }
    // task1: 
    println!("task1: {}", flood_fill(&matrix, Point{x:-1, y:0}, Vector{x:1,y:0}));
    
    // task2: 
    let mut max = 0;
    for i in 0..matrix.len() {
        let t = flood_fill(&matrix, Point{x:i as i32, y:-1}, Vector{x:0,y:1});
        if t > max {
            max = t
        } 
        let t = flood_fill(&matrix, Point{x:i as i32, y:matrix.len() as i32}, Vector{x:0,y:-1});
        if t > max {
            max = t
        } 
        let t = flood_fill(&matrix, Point{x:-1, y:i as i32}, Vector{x:1,y:0});
        if t > max {
            max = t
        } 
        let t = flood_fill(&matrix, Point{x:matrix.len() as i32, y:i as i32}, Vector{x:-1,y:0});
        if t > max {
            max = t
        } 
    }  

    println!("task2: {max}");

    
}


fn flood_fill(matrix: &Vec<Vec<char>>, startpos: Point<i32>, startdir: Vector<i32>) -> usize {
    let mut filled: Vec<Vec<[bool;4]>> = Vec::new(); 
    for line in matrix {
        let row = vec![[false; 4]; line.len()];
        filled.push(row);
    }

    // flood_fill_recursion(matrix, &mut filled, startpos, startdir);
    
    let mut queue:VecDeque<(Point<i32>, Vector<i32>)> = VecDeque::new();
    queue.push_back((startpos, startdir));

    while let Some((pos, direction)) = queue.pop_front() {

        let newpos = pos + direction;

        if newpos.x < 0 || newpos.y < 0 || newpos.x >= matrix.len() as i32 || newpos.y >= matrix.len() as i32 {
            continue;
        }

        let dir_idx = dir_to_idx(&direction);
        if filled[newpos.y as usize][newpos.x as usize][dir_idx] {
            continue;
        }

        filled[newpos.y as usize][newpos.x as usize][dir_idx] = true;

        match matrix[newpos.y as usize][newpos.x as usize] {
            '\\' => {
                let newdir = Vector{ x: direction.y, y: direction.x};
                queue.push_back((newpos, newdir));
            },
            '/' => {
                let newdir = Vector{ x: -direction.y, y: -direction.x};
                queue.push_back((newpos, newdir));
            },
            '-' => {
                if direction.y != 0 {
                    let newdir1 = Vector { x: direction.y, y: direction.x};
                    queue.push_back((newpos, newdir1));
                    let newdir2 = Vector {x: -direction.y, y: -direction.x};
                    queue.push_back((newpos, newdir2));
                } else {
                    queue.push_back((newpos, direction));
                }
            },
            '|' => {
                if direction.x != 0 {
                    let newdir1 = Vector { x: direction.y, y: direction.x};
                    queue.push_back((newpos, newdir1));
                    let newdir2 = Vector {x: -direction.y, y: -direction.x};
                    queue.push_back((newpos, newdir2));
                } else {
                    queue.push_back((newpos, direction));
                }
            }
            '.' => {
                queue.push_back((newpos, direction));
            },
            _ => panic!(""),
        }
    }

    // count filled
    let mut sum = 0;
    for i in filled {
        for j in i {
            if j.contains(&true) {
                sum +=1;
            } 
        }
    }
    sum
}

fn dir_to_idx(dir: &Vector<i32>) -> usize {
    match *dir {
        Vector {x: 0, y: 1} => 0,
        Vector {x: 1, y: 0} => 1,
        Vector {x: 0, y: -1} => 2,
        Vector {x: -1, y: 0} => 3,
        _ => panic!("invalid direction")
    }
}