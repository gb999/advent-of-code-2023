use std::fs;
const FILE_PATH : &'static str = "input.txt";

struct Game {
    id : u32,
    red : Vec<u32>,
    green : Vec<u32>,
    blue : Vec<u32>,
    n : usize,
}
impl Game {
    fn new(id : u32) -> Self {
        Self {
            id,
            red: Vec::new(),
            green: Vec::new(),
            blue: Vec::new(),
            n : 0,
        }
    }
    fn add(&mut self, r: u32, g: u32, b: u32) {
        self.red.push(r);
        self.green.push(g);
        self.blue.push(b);
        self.n += 1;
    }

    fn is_valid(&self, r : u32, g: u32, b: u32) -> bool {
        for i in 0..self.n {
            if self.red[i] > r || self.green[i] > g || self.blue[i] > b {
                return false;
            }
        }
        return true;
    }
    
    // This could be refactored somehow... 
    fn get_power(&self) -> u32 {
        let mut maxr = self.red[0];
        let mut maxg = self.green[0];
        let mut maxb = self.blue[0];
        for i in 1..self.n { 
            if self.red[i] > maxr {maxr = self.red[i]}
            if self.green[i] > maxg {maxg = self.green[i]}
            if self.blue[i] > maxb {maxb = self.blue[i]}
        }
        return maxr * maxg * maxb;

    }
}

fn get_num_from_pull(pull: &str, color : &str ) -> u32 {
    match pull.find(color) {
        Some(i) => {
            i as u32;
            let pull = pull.as_bytes(); 
            let mut num: u32 = 0;

            let mut r = 1;
            for j in (0..(i - 1)).rev() {
                if pull[j].is_ascii_digit() {
                    num += (pull[j] as char).to_digit(10).unwrap() * r; // unsafe but works
                    r *= 10;
                    
                } else {break};
            }
            num as u32
        },
        None => 0,
    }
}

fn main() {
    let contents : String = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");
    
    let mut i = 1;
    let mut sum_of_valid_ids = 0;
    let mut sum_of_powers = 0;
    for line in contents.lines() {
        let mut game = Game::new(i);
        let v: Vec<&str> = line.split(':').collect();
        let pulls: Vec<&str> = v[1].split(';').collect();

        
        for pull in pulls {
            let r = get_num_from_pull(pull, "red");
            let g = get_num_from_pull(pull, "green");
            let b = get_num_from_pull(pull, "blue");

            game.add(r,g,b);
        }
        if game.is_valid(12,13,14)  {
            sum_of_valid_ids += i ;
        }
        sum_of_powers += game.get_power();

        i += 1;
    }
    // println!("{sum_of_valid_ids}");
    println!("{sum_of_powers}");
}
