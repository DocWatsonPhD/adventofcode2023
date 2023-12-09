pub mod day4 {
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::{BufReader, BufRead, Result};
    use std::vec;

    pub fn part_one() -> Result<()> {
        let file = File::open("C:\\Users\\docwa\\projects\\adventofcode2023\\src\\day4\\input.txt")
            .expect("What a terrible thing to have happened");
        let reader = BufReader::new(file);
        let mut sum: u32 = 0;

        for line in reader.lines() {
            let line: String = line?;
            sum += get_num_for_part_one(&line);
        }
    
        println!("{}", sum);

        Ok(())
    }

    pub fn part_two() -> Result<()>{
        let file = File::open("C:\\Users\\docwa\\projects\\adventofcode2023\\src\\day4\\input.txt")
            .expect("What a terrible thing to have happened");
        let reader = BufReader::new(file);
        let mut card_counts: HashMap<u32, u32> = HashMap::new();
        let mut card_num: u32 = 1;
        let mut match_tally: Vec<u32> = vec![];
        let mut total_cards: u32 = 0;
        
        for line in reader.lines() {
            let line: String = line?;
            card_counts.insert(card_num, 1);
            card_num += 1;
            total_cards += 1;
            let (winners, matchers) = get_data(&line);
            match_tally.push(get_matching_num(winners, matchers));
        }

        for (i, card_value) in match_tally.iter().enumerate() {
            let card = (i + 1) as u32; // representing real card number
            let additional: u32 = *card_counts.get(&card).unwrap();
            for j in 1..(*card_value + 1) {
                total_cards += additional;
                increment_with_additional(&mut card_counts, &(card + j), additional);
            }
        }
    
        println!("{}", total_cards);

        Ok(())
    }

    fn increment_with_additional(card_counts: &mut HashMap<u32, u32>, i: &u32, additional: u32) {
        match card_counts.get(&i) {
            Some(_) => { card_counts.insert(*i, card_counts.get(&i).unwrap() + additional); }
            None => { card_counts.insert(*i, 1 + additional); }
        }
    }

    fn get_num_for_part_one(s: &str) -> u32 {
        let (w, m) = get_data(s);
        let matchers = get_matching_num(w, m);

        if matchers < 2 { return matchers; }
        1 << (matchers - 1)
    }

    fn get_matching_num(winners: Vec<u32>, matchers: Vec<u32>) -> u32 {
        let mut w_map: HashMap<u32, u32> = HashMap::new();
        winners.iter()
            .for_each(|x| { w_map.insert(*x, 0); } );
        let count: u32 = matchers.iter()
            .filter(|x| w_map.contains_key(*x))
            .count() as u32;

        count
    }

    fn get_data(s: &str) -> (Vec<u32>, Vec<u32>) {
        let mut spl = s.split(": ");
        spl.next(); // Advance past the "card X: " section

        let v: Vec<&str> = spl.next().unwrap()
                    .split(" | ")
                    .collect();

        (
            v[0]
                .split(" ")
                .filter(|x| x != &"")
                .collect::<Vec<_>>()
                .iter()
                .map(|x| str::parse(x).unwrap())
                .collect(),
            v[1]
                .split(" ")
                .filter(|x| x != &"")
                .collect::<Vec<_>>()
                .iter()
                .map(|x| str::parse(x).unwrap())
                .collect()
        )
    }
}