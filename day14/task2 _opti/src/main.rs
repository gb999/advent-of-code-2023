use std::{fs, collections::HashMap};
use std::ops::{Index, IndexMut};

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Matrix {
    data: [Rock; 10000]
}

impl Matrix {
    fn len(&self) -> usize {
        100
    }
}

impl Index<usize> for Matrix {
    type Output = [Rock];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index * 100..(index+1)*100]
    }
}
impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index * 100..(index+1)*100]

    }
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read file");

    let mut data = [Rock::Nothing; 10000];

    let mut i:usize = 0;   
    for line in contents.lines() {
        for c in line.chars() {
            data[i] = Rock::from_char(c);
            i += 1;
        }
    }

    let mut matrix = Matrix { data };
    
    let mut i = 0;
    let mut cache: HashMap<Matrix, (usize, Matrix)> = HashMap::new();
    let mut key: Matrix = matrix.clone();
    while !cache.contains_key(&key) {
        key = matrix.clone();
        cycle(&mut matrix);
        cache.insert(key, (i, matrix));
        key = matrix.clone();
        i += 1;
    }
    
    
    let remainingcycles = (1000000000 - i) % (i - cache[&key].0);
    println!("{remainingcycles}");
    for j in 0..remainingcycles {
        cycle(&mut matrix);
    }

    let mut sum = 0;
    let mut output: String = String::from("");
    for i in 0..100 {
        for j in 0..100 {
            let ch:char; 
            match matrix[i][j] {
                Rock::Square => ch ='#',
                Rock::Circle => {sum += 100 - i; ch = 'O';},
                Rock::Nothing => ch = '.',
            };
            output.push(ch)
        } 
        output.push('\n');
    }
    println!("sum: {sum}");

    let _ = fs::write("output.txt", output);


}

enum Direction {
    NORTH,
    WEST,
    SOUTH,
    EAST,
}


#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Rock {
    Square,
    Circle,
    Nothing
}

impl Rock {
    fn from_char(c: char) -> Self {
        match c {
            'O' => Rock::Circle,
            '#' => Rock::Square,
            '.' => Rock::Nothing,
            _ => panic!("Invalid rock"),
        }
    }
}

fn apply_gravity(dir: Direction, matrix: &mut Matrix) {
    let n = matrix.len();
    // Use indexing
    for i in 0..n {
        let startsquare: i32 = match dir {
            Direction::NORTH | Direction::WEST => -1,
            Direction::SOUTH | Direction::EAST => n as i32,
        };
        let mut lastsquare = startsquare;
        let mut rockcount = 0;
        
        for j in 0..n {
            let (i,j) = match dir {
                Direction::NORTH => (j, i),
                Direction::WEST => (i, j),
                Direction::SOUTH => (n - j - 1, i),
                Direction::EAST => (i, n - j - 1),
            };

            match matrix[i][j] {
                Rock::Square => {
                    lastsquare = match dir {
                        Direction::NORTH | Direction::SOUTH => i as i32,
                        Direction::WEST | Direction::EAST => j as i32,
                    };
                    rockcount = 0;
                },
                Rock::Circle => {
                    rockcount += 1;
                    matrix[i][j] = Rock::Nothing;
                    let cell_to_swap = match dir {
                        Direction::NORTH => {
                            &mut matrix[(lastsquare + rockcount) as usize][j] 
                        },
                        Direction::WEST => {
                            &mut matrix[i][(lastsquare + rockcount) as usize]
                        },
                        Direction::SOUTH => {
                            &mut matrix[(lastsquare - rockcount) as usize][j] 
                        },
                        Direction::EAST => {
                            &mut matrix[i][(lastsquare - rockcount) as usize] 
                        },
                    };
                    *cell_to_swap = Rock::Circle;
                },
                Rock::Nothing => ()
            }

        }
    }
}

fn cycle(matrix: &mut Matrix) {
    apply_gravity(Direction::NORTH, matrix);
    apply_gravity(Direction::WEST, matrix);
    apply_gravity(Direction::SOUTH, matrix);
    apply_gravity(Direction::EAST, matrix);
}