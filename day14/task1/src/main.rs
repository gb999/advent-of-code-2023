use std::fs;


fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read file");

    // iterate colums 
    // let c := number of 0-s after each #
    // let i := row of # (counted from bottom to top)
    // A segment is a # and the 0-s pushing it
    // load of a segment = (i - 1) + = (i - 2) + ... + (i - c)


    // store columns as rows
    let mut columns: Vec<Vec<char>> = Vec::new();
    let l = contents.lines().next().unwrap().len();
    for _ in 0..l {
        columns.push(Vec::new());
    }

    for (_, line) in contents.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            columns[j].push(c);
        }
    }

    let mut sum = 0;

    for column in columns {
        let mut lastcube = 0;
        let mut rockcount = 0;
        for (j, char) in column.iter().enumerate() {
            match char {
                '#' => {lastcube = j + 1; rockcount = 0;},
                'O' => {
                    rockcount += 1; 
                    sum += l - rockcount - lastcube + 1;
                },
                '.' => (),
                _ => panic!("Invalid input")
            }
        }
    }

    dbg!(sum);
    



}


