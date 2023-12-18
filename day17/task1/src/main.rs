use std::{collections::{BinaryHeap, HashSet}, cmp::Reverse};

fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("Should have been able to read file");
    
    let mut matrix:Vec<Vec<u32>> = Vec::new();
    for line in contents.lines() {
        let mut row = Vec::new();
        for c in line.chars()  {
            row.push(c.to_digit(10).unwrap());
        }
        matrix.push(row);
    }
    let res = find_lowest_heatloss(&matrix);
    dbg!(res);
}

struct Move {
    x: usize,
    y: usize,
    dirx: i32,
    diry: i32,
    n_dir: usize,
    heatloss: usize
}

impl Eq for Move {

}
impl PartialEq for Move {
    fn eq(&self, other: &Self) -> bool {
        self.heatloss == other.heatloss
    }
}
impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.heatloss.partial_cmp(&other.heatloss)
    }
}
impl Ord for Move {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.heatloss.cmp(&other.heatloss)
    }
}

// Idea from: https://www.youtube.com/watch?v=2pDSooPLLkI&t=331s
fn find_lowest_heatloss(matrix: &Vec<Vec<u32>>) -> u32 {
    // min heap
    let mut heap: BinaryHeap<Reverse<Move>> = BinaryHeap::new();
    let mut seen:HashSet<(usize,usize,i32,i32,usize)> = HashSet::new();

    
    heap.push(Reverse(Move{x:0,y:0,dirx:1,diry:0,n_dir:0,heatloss:0}));
    while let Some(Reverse(Move{x, y, dirx, diry, n_dir, heatloss})) = heap.pop() {
        if seen.contains(&(x, y, dirx, diry, n_dir)) {
            continue;
        }
        seen.insert((x, y, dirx, diry, n_dir));

        if x == matrix.len() -1 && y == matrix.len() - 1 {
            return heatloss as u32;
        }

        if n_dir < 3  { // continue going in the same direction 
            let newx = x as i32 + dirx;
            let newy = y as i32 + diry;
            
            if newx < matrix.len() as i32 && newx >= 0 
            && newy < matrix.len() as i32 && newy >= 0 {
                heap.push(Reverse(Move {
                    x: newx as usize,
                    y: newy as usize,
                    dirx, 
                    diry, 
                    n_dir: n_dir +1,
                    heatloss: heatloss + matrix[newx as usize][newy as usize] as usize
                }));
        
            }
        } 

        for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] { // Turn
            if (dirx == dx && diry  == dy) // Can't continue in same direction
            || (dirx == -dx && diry == -dy) { // Can't reverse direction
                continue;
            }
            let newx = x as i32 + dx;
            let newy = y as i32 + dy;
            if newx < matrix.len() as i32 && newx >= 0 
            && newy < matrix.len() as i32 && newy >= 0 {
                heap.push(Reverse(Move {
                    x: newx as usize,
                    y: newy as usize,
                    dirx: dx, 
                    diry: dy, 
                    n_dir: 1,
                    heatloss: heatloss + matrix[newx as usize][newy as usize] as usize
                }));
            }

        }
    }
    0
}