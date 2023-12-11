pub mod day7 {
    use core::num;
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::{BufReader, BufRead, Result};
    use lazy_static::lazy_static;
    
    const INPUT: &str = "C:\\Users\\docwa\\projects\\adventofcode2023\\src\\day7\\input.txt";

    lazy_static! {
        static ref CARD_RANKS: HashMap<char, u8> = {
            let mut map = HashMap::new();
            map.insert('2', 2);
            map.insert('3', 3);
            map.insert('4', 4);
            map.insert('5', 5);
            map.insert('6', 6);
            map.insert('7', 7);
            map.insert('8', 8);
            map.insert('9', 9);
            map.insert('T', 10);
            map.insert('J', 11);
            map.insert('Q', 12);
            map.insert('K', 13);
            map.insert('A', 14);
            map
        };
    }

    lazy_static! {
        static ref CARD_RANKS_JOKER: HashMap<char, u8> = {
            let mut map = HashMap::new();
            map.insert('J', 1);
            map.insert('2', 2);
            map.insert('3', 3);
            map.insert('4', 4);
            map.insert('5', 5);
            map.insert('6', 6);
            map.insert('7', 7);
            map.insert('8', 8);
            map.insert('9', 9);
            map.insert('T', 10);
            map.insert('Q', 12);
            map.insert('K', 13);
            map.insert('A', 14);
            map
        };
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
    pub enum HandRank {
        HighCard,
        OnePair,
        TwoPair,
        ThreeOfAKind,
        FullHouse,
        FourOfAKind,
        FiveOfAKind
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
    struct Card {
        hand_rank: HandRank,
        hand: [u8; 5],
        bid: u64
    }

    impl Card {
        fn new(s: &str, b: u64, joker: bool) -> Card {
            let r: &HashMap<char, u8> = if joker { &CARD_RANKS_JOKER } else { &CARD_RANKS };
            let values: Vec<u8> = s.chars()
                .map(|c| *r.get(&c).unwrap())
                .collect();
            let mut sorted: Vec<u8> = values.clone().to_owned();
            sorted.sort_by(|a, b| b.cmp(a));
            // I wrote the joker hand ranking assuming a *forward* sorting...whoops
            let ranking = if joker { get_hand_rank_jokers([sorted[4], sorted[3], sorted[2], sorted[1], sorted[0]]) }
                                    else { get_hand_rank([sorted[0], sorted[1], sorted[2], sorted[3], sorted[4]]) };
            Card {
                hand: [values[0], values[1], values[2], values[3], values[4]],
                hand_rank: ranking,
                bid: b
            }
        }
    }

    // Assumes a sorted array (forward or reverse)
    fn get_hand_rank(c: [u8; 5]) -> HandRank {
        // 5 of a kind case (1st char matches 5th)
        if c[0] == c[4] 
            { return HandRank::FiveOfAKind; }
        // 4 of a kind (2nd matches 5th or 1st matches 4th)
        if c[0] == c[3] || c[1] == c[4]
            { return HandRank::FourOfAKind; }
        // 3 of a kind from the left -- could be 3OAK or full house
        if c[0] == c[1] && c[1] == c[2] {
            if c[3] == c[4] { return HandRank::FullHouse; }
            else { return HandRank::ThreeOfAKind; }
        }
        // 3 of a kind from the right -- could be 3OAK or full house
        if c[2] == c[4] {
            if c[0] == c[1] { return HandRank::FullHouse; }
            else { return HandRank::ThreeOfAKind; }
        }
        // 3 of a kind in the middle -- can only be 3OAK
        if c[1] == c[3] { return HandRank::ThreeOfAKind; }
        // Pair with 1 and 2 -- could be 1 pair or 2 (in 3+4 or 4+5)
        if c[0] == c[1] {
            if c[2] == c[3] || c[3] == c[4] { return HandRank::TwoPair; }
            else { return HandRank::OnePair; }
        }
        // Pair with 2 and 3 -- could be 1 pair or 2 (in 4+5)
        if c[1] == c[2] {
            if c[3] == c[4] { return HandRank::TwoPair; }
            else { return HandRank::OnePair; }
        }
        // Last remaining pair cases
        if c[2] == c[3] || c[3] == c[4] { return HandRank::OnePair; }

        HandRank::HighCard
    }

    // Assumes a sorted array, incrementing (i.e. forward)
    // Would this be more performant as a state-machine?
    fn get_hand_rank_jokers(c: [u8; 5]) -> HandRank {
        let mut num_jokers: u8 = 0;

        for i in 0..5 { 
            if c[i] == 1 { num_jokers += 1; }
            else { break; }
        }

        if num_jokers >= 4 { return HandRank::FiveOfAKind; }
        if num_jokers == 3 {
            if c[3] == c[4] { return HandRank::FiveOfAKind; }
            return HandRank::FourOfAKind;
        }
        if num_jokers == 2 {
            // Sorted numbers mean if 3rd and 5th are equal, they are also equal to the 4th.
            // ex. JJ222
            if c[2] == c[4] { return HandRank::FiveOfAKind; }
            // 2 jokers means that every possibility  of extra pairs upgrades to at least a 4 of a kind:
            // JJ223 -> 4 2s
            // JJ233 -> 4 3s
            if c[2] == c[3] || c[3] == c[4] { return HandRank::FourOfAKind; }
            // Only remaining case is if 2nd != 3rd != 4th, which always upgrades to 3 of a kind
            // ex. JJ234 -> 3 2s or 3 3s or 3 4s
            return HandRank::ThreeOfAKind;
        }
        if num_jokers == 1 {
            // Remaining cards being the same is always a 5OAK
            // ex. J2222
            if c[1] == c[4] { return HandRank::FiveOfAKind; }
            // All but one being the same is always a 4OAK
            // ex. J2223, J2333
            if c[1] == c[3] || c[2] == c[4] { return HandRank::FourOfAKind; }
            // Full house, ex. J2233
            if c[1] == c[2] && c[3] == c[4] { return HandRank::FullHouse; }
            // If any 2 remaining cards are the same, we only have a 3 of a kind
            if c[1] == c[2] || c[2] == c[3] || c[3] == c[4] { return HandRank::ThreeOfAKind; }
            // With no remaining cards being equal, we always upgrade to a pair
            // ex. J2345 -> Pair of 2s (or 3s, 4s, 5s)
            return HandRank::OnePair;
        }

        get_hand_rank(c)
    }

    pub fn part_one() -> Result<()> {
        let mut cards: Vec<Card> = get_data(false);
        let mut result: usize = 0;

        cards.sort();
        for (i, card) in cards.iter().enumerate() {
            result += (i + 1) * (card.bid as usize);
        }

        println!("{}", result);

        Ok(())
    }

    pub fn part_two() -> Result<()>{
        let mut cards = get_data(true);
        let mut result: usize = 0;

        cards.sort();
        for (i, card) in cards.iter().enumerate() {
            result += (i + 1) * (card.bid as usize);
        }

        println!("{}", result);

        Ok(())
    }

    fn get_data(joker: bool) -> Vec<Card> {
        let file = File::open(INPUT)
            .expect("What a terrible thing to have happened");
        let reader = BufReader::new(file);
        let mut result: Vec<Card> = vec![];

        for line in reader.lines() {
            let line: String = line.expect("oops");
            let mut spl = line.split(' ');
            let hand: &str = spl.next().unwrap();
            let bid: u64 = spl.next().unwrap().parse().unwrap();
            result.push(Card::new(hand, bid, joker));
        }

        result
    }

    fn test_hand_ranks() {
        println!("Testing 5OAK");
        println!("\t 22222 {:?}", get_hand_rank([2, 2, 2, 2, 2]));
        println!("\t JJJJJ {:?}", get_hand_rank_jokers([1, 1, 1, 1, 1]));
        println!("\t JJJJA {:?}", get_hand_rank_jokers([1, 1, 1, 1, 14]));
        println!("Testing 4OAK");
        println!("\t 22223 {:?}", get_hand_rank([2, 2, 2, 2, 3]));
        println!("\t 23333 {:?}", get_hand_rank([2, 3, 3, 3, 3]));
        println!("\t JJJ23 {:?}", get_hand_rank_jokers([1, 1, 1, 2, 3]));
        println!("\t JJ223 {:?}", get_hand_rank_jokers([1, 1, 2, 2, 3]));
        println!("\t JJ233 {:?}", get_hand_rank_jokers([1, 1, 2, 3, 3]));
        println!("\t J2223 {:?}", get_hand_rank_jokers([1, 2, 2, 2, 3]));
        println!("\t J2333 {:?}", get_hand_rank_jokers([1, 2, 3, 3, 3]));
        println!("Testing Full House");
        println!("\t 22233 {:?}", get_hand_rank([2, 2, 2, 3, 3]));
        println!("\t 22333 {:?}", get_hand_rank([2, 2, 3, 3, 3]));
        println!("\t J2233 {:?}", get_hand_rank_jokers([1, 2, 2, 3, 3]));
        println!("Test 3OAK");
        println!("\t 22234 {:?}", get_hand_rank([2, 2, 2, 3, 4]));
        println!("\t 23334 {:?}", get_hand_rank([2, 3, 3, 3, 4]));
        println!("\t 23444 {:?}", get_hand_rank([2, 3, 4, 4, 4]));
        println!("\t JJ234 {:?}", get_hand_rank_jokers([1, 1, 2, 3, 4]));
        println!("\t J2234 {:?}", get_hand_rank_jokers([1, 2, 2, 3, 4]));
        println!("\t J2334 {:?}", get_hand_rank_jokers([1, 2, 3, 3, 4]));
        println!("\t J2344 {:?}", get_hand_rank_jokers([1, 2, 3, 4, 4]));
        println!("Testing pair");
        println!("\t 22345 {:?}", get_hand_rank([2, 2, 3, 4, 5]));
        println!("\t 23345 {:?}", get_hand_rank([2, 3, 3, 4, 5]));
        println!("\t 23445 {:?}", get_hand_rank([2, 3, 4, 4, 5]));
        println!("\t 23455 {:?}", get_hand_rank([2, 3, 4, 5, 5]));
        println!("\t J2345 {:?}", get_hand_rank_jokers([1, 2, 3, 4, 5]));
        println!("Testing high card");
        println!("\t 23456 {:?}", get_hand_rank([2, 3, 4, 5, 6]));
    }
}