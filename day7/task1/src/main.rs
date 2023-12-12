use std::{fs, cmp::Ordering};
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
struct Hand {
    hand_type: HandType,
    cards: [CardType; 5],
    bid: u32
}
impl Hand {
    fn new(cards_str: String,
        bid: u32) -> Self {
            let hand_type = HandType::from_cards(&cards_str);
            let mut t_cards = cards_str.chars().map(|ch| CardType::from_char(ch));
            let mut cards : [CardType; 5] = [CardType::Two; 5];
            for i in 0..5 {
                cards[i]= t_cards.next().unwrap();
            }
            Hand {cards, hand_type, bid}
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Copy, Clone)]
enum CardType {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A
}
impl CardType {
    fn from_char(ch: char) -> CardType {
        match ch {
            '2' => CardType::Two,
            '3' => CardType::Three,
            '4' => CardType::Four,
            '5' => CardType::Five,
            '6' => CardType::Six,
            '7' => CardType::Seven,
            '8' => CardType::Eight,
            '9' => CardType::Nine,
            'T' => CardType::T,
            'J' => CardType::J,
            'Q' => CardType::Q,
            'K' => CardType::K,
            'A' => CardType::A,
            _ =>panic!()
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}
impl HandType {
    fn from_cards(cards: &String) -> HandType {
        // Order cards
        let mut chars: Vec<_> = cards.chars().collect();
        chars.sort_by(|a, b| a.cmp(b));
        let mut iter = chars.iter();
        let mut lastchar = iter.next().unwrap();

        let mut occurences: Vec<u32> = Vec::new();
        let mut i = 0;
        occurences.push(1); 

        for ch in iter {
            if ch == lastchar { occurences[i] +=1; }
            else { i+=1; occurences.push(1); }
            lastchar = ch;
        }
        let hand_type:HandType;
        
        if occurences.contains(&5) {
            hand_type = HandType::FiveOfAKind;
        } else if occurences.contains(&4) {
            hand_type = HandType::FourOfAKind;
        } else if occurences.contains(&3) && occurences.contains(&2){
            hand_type = HandType::FullHouse;
        } else if occurences.contains(&3) && occurences.iter().filter(|&a| *a == 1).count() == 2 {
            hand_type = HandType::ThreeOfAKind;
        } else if occurences.iter().filter(|&a| *a == 2).count() == 2 {
            hand_type = HandType::TwoPair;
        } else if occurences.contains(&2) {
            hand_type = HandType::OnePair;
        } else {
            hand_type = HandType::HighCard;
        }
        hand_type
    }
    
}


fn main() {
    let contents = fs::read_to_string("input.txt").expect("should be a ble to read file");
    
    let mut hands: Vec<Hand> = contents.lines()
        .map(|l| Hand::new(l[..5].to_string(), l[6..].parse::<u32>().unwrap()))
        .collect();
        
    // Sort by cards
    hands.sort_by(|a, b| {
        for i in 0..5 {
            if a.cards[i] == b.cards[i] {continue;}
            else {return a.cards[i].cmp(&b.cards[i]); }
             
        }
        
        Ordering::Equal
    });
    hands.sort_by(|a, b|  a.hand_type.cmp(&b.hand_type)); // Then sort by hand type
    let mut sum = 0;
    let mut rank = 1;
    for h in hands {
        sum += rank * h.bid;
        rank += 1;
    }
    dbg!(sum);
}
