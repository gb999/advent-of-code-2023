use std::{fs, collections::HashMap};

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read file");

    let mut matrix: Vec<Vec<Rock>> = contents.lines()
        .map(|line| line.chars()
            .map(Rock::from_char)
            .collect::<Vec<Rock>>())
        .collect();    
    
    let mut cache: HashMap<Vec<Vec<Rock>>, (u128, Vec<Vec<Rock>>)> = HashMap::new();
    for _ in 0..1000000000 {

        cycle(&mut matrix);

    }

    let mut output: String = String::from("");
    for row in matrix {
        for column in row {
            let ch = match column {
                Rock::Square => '#',
                Rock::Circle => 'O',
                Rock::Nothing => '.',
            };
            output.push(ch)
        } 
        output.push('\n');
    }

    let _ = fs::write("output.txt", output);


}

enum Direction {
    NORTH,
    WEST,
    SOUTH,
    EAST,
}


#[derive(Debug, PartialEq, Eq, Hash)]
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

fn apply_gravity(dir: Direction, matrix: &mut Vec<Vec<Rock>>) {
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

fn cycle(matrix: &mut Vec<Vec<Rock>>) {
    apply_gravity(Direction::NORTH, matrix);
    apply_gravity(Direction::WEST, matrix);
    apply_gravity(Direction::SOUTH, matrix);
    apply_gravity(Direction::EAST, matrix);
}