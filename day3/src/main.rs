use std::fs;
const FILE_PATH : &'static str = "input.txt";

enum State {
    Digit {current_number: u32, valid: bool},
    NonDigit
}

impl State {
    fn next(self, index_in_line: usize, current_line: &String, prev_line: &Option<&String>, next_line: &Option<&String>) -> (State, u32) {
        let ch = Self::char_at(index_in_line, &current_line);
        let is_digit = ch.is_ascii_digit();
        let next_state: State;
        let mut return_val = 0;
        match (self, is_digit) {
            (State::Digit { current_number, valid }, true)  => {
                // Check top + bottom
                let valid = valid || Self::check_top_bottom(index_in_line, prev_line, next_line);
                next_state = State::Digit {current_number: current_number * 10 + ch.to_digit(10).unwrap(), valid};
            },
            (State::Digit {current_number, valid }, false) => {
                // Check right + diagonals + top + bottom
                // need to start check from index - 1 !!!
                let valid = valid || Self::check_top_bottom(index_in_line - 1, prev_line, next_line)
                                        || Self::check_right_side(index_in_line - 1, current_line, prev_line, next_line);
                // if valid return value
                next_state = State::NonDigit;                
                if valid { return_val = current_number};
            },
            (State::NonDigit, true) => {
                // Check left + diagonals + top + bottom
                let valid = Self::check_top_bottom(index_in_line, prev_line, next_line)
                                || Self::check_left_side(index_in_line, &current_line, &prev_line, &next_line);

                next_state = State::Digit {current_number: ch.to_digit(10).unwrap(), valid}

            }
            (State::NonDigit, false) => {
                next_state = State::NonDigit;
            }
        };
        return (next_state, return_val);
    }

    

    fn char_at(i: usize, str: &String) -> char {
        str.as_bytes()[i] as char
    }

    fn is_symbol(ch: char) -> bool {
        !ch.is_ascii_digit() && ch != '.'
    }
    fn check_top_bottom(index: usize, prev_line: &Option<&String>, next_line: &Option<&String>) -> bool {
        if let Some(str) = next_line {
            if Self::is_symbol(Self::char_at(index, str)) {return true};
        }
        if let Some(str) = prev_line {
            if Self::is_symbol(Self::char_at(index, str)) {return true};
        }
        false
    }

    fn check_left_side(index: usize, current_line: &String, prev_line: &Option<&String>, next_line: &Option<&String>) -> bool {
        if index == 0 { return false };

        if Self::is_symbol(Self::char_at(index - 1, current_line)) {return true};

        if let Some(str) = next_line {
            if Self::is_symbol(Self::char_at(index - 1, str)) {return true};
        }

        if let Some(str) = prev_line {
            if Self::is_symbol(Self::char_at(index - 1, str)) {return true};
        }
        false
    }
    fn check_right_side(index: usize, current_line: &String, prev_line: &Option<&String>, next_line: &Option<&String>) -> bool {
        if index == current_line.len() - 1 { return false };

        if Self::is_symbol(Self::char_at(index + 1, current_line)) {return true};

        if let Some(str) = next_line {
            if Self::is_symbol(Self::char_at(index + 1, str)) {return true};
        }

        if let Some(str) = prev_line {
            if Self::is_symbol(Self::char_at(index + 1, str)) {return true};
        }
        false
    }
}

fn main() {
    let contents = fs::read_to_string(FILE_PATH).unwrap();
    let mut state = State::NonDigit;
    let mut sum = 0;

    let mut prev_line: Option<&String>;
    let mut next_line: Option<&String>;
    let lines: Vec<String> = contents.lines().map(|s|s.to_string()).collect();

    for (l, line) in lines.iter().enumerate() {
        prev_line = if l > 1 {lines.get(l - 1)} else { None};
        next_line = if l < lines.len() - 1  {lines.get(l + 1)} else {None};
        for i in 0..line.len() {
            let n;
            (state, n) = state.next(i, &line.to_string(), &prev_line, &next_line);
            sum += n;
            if n != 0 {println!("{n}")};
        }
        state = State::NonDigit;

    }

    println!("{sum}");
}
