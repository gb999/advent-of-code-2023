fn main() {
    // let contents = std::fs::read_to_string("example.txt").unwrap();
    let contents = std::fs::read_to_string("input.txt").unwrap();
    let strings: Vec<&str> = contents.split(",").collect();
    let mut boxes:[Vec<Lens>; 256] = std::iter::repeat_with(|| Vec::new())
        .take(256).collect::<Vec<_>>().try_into().unwrap();
    
    for s in strings {

        let mut wordsum: u128 = 0;
        let chars = s.chars(); 

        for (i, ch) in chars.enumerate() {
            match ch {
                '-' => {
                    let lens = Lens::new(
                        &s[0..i], 0
                    );
                    if let Some(pos) = boxes[wordsum as usize]
                        .iter()
                        .position(|l| *l == lens) {
                            boxes[wordsum as usize].remove(pos);
                        }
                        
                        
                },
                '=' => {
                    let lens = Lens::new(
                        &s[0..i], s
                            .chars()
                            .nth(i+1)
                            .unwrap()
                            .to_digit(10)
                            .unwrap() as u8
                    );
                    if let Some(pos) = boxes[wordsum as usize]
                        .iter()
                        .position(|l| *l == lens) {
                        boxes[wordsum as usize][pos] = lens;
                    } else {
                        boxes[wordsum as usize].push(lens);  

                    }

                    break;
                },
                _ => wordsum = hash(ch, wordsum),
            }

        }
    }
    let mut sum = 0;
    for (i, b) in boxes.iter().enumerate() {
        for (slot, lens) in b.iter().enumerate() {
            sum += (i + 1) * (slot + 1) * lens.f_length as usize;
        }
    }
    dbg!(sum);
    // println!("{:?}", boxes);
}


fn hash(ch: char, start: u128) -> u128 {
    (start + ch as u128) * 17 % 256
}

#[derive(Debug, Eq)]
struct Lens<'a> {
    label:  &'a str,
    f_length: u8,
}

impl Lens<'_> {
    fn new(label: &str, f_length: u8) -> Lens {
        Lens {label, f_length}
    }
}

impl PartialEq for Lens<'_> { // Equality only by label
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label
    }
}
