pub mod day1 {
    use std::collections::HashMap;

    use crate::my_reader;

    type WordToValue<'a> = (&'a str, u32);

    const WORD_VALUE: [WordToValue; 10] = [
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    pub fn part_one() {
        let mut reader = my_reader::BufReader::open("C:\\Users\\docwa\\projects\\adventofcode2023\\src\\day1\\input.txt")
            .expect("Some shit happened");
        let mut buffer = String::new();
        let mut sum: u32 = 0;
    
        while let Some(line) = reader.read_line(&mut buffer) {
            sum += get_num_for_part_one(line.expect("Failed to read line").trim());
        }
    
        println!("{}", sum);
    }

    pub fn part_two() {
        let mut first_letters: HashMap<char, Vec<WordToValue>> = HashMap::new();
        first_letters.insert('z', vec![WORD_VALUE[0]]);
        first_letters.insert('o', vec![WORD_VALUE[1]]);
        first_letters.insert('t', vec![WORD_VALUE[2], WORD_VALUE[3]]);
        first_letters.insert('f', vec![WORD_VALUE[4], WORD_VALUE[5]]);
        first_letters.insert('s', vec![WORD_VALUE[6], WORD_VALUE[7]]);
        first_letters.insert('e', vec![WORD_VALUE[8]]);
        first_letters.insert('n', vec![WORD_VALUE[9]]);

        let mut last_letters: HashMap<char, Vec<WordToValue>> = HashMap::new();
        last_letters.insert('o', vec![WORD_VALUE[2], WORD_VALUE[0]]);
        last_letters.insert('e', vec![WORD_VALUE[1], WORD_VALUE[5], WORD_VALUE[9], WORD_VALUE[3]]);
        last_letters.insert('r', vec![WORD_VALUE[4]]);
        last_letters.insert('x', vec![WORD_VALUE[6]]);
        last_letters.insert('n', vec![WORD_VALUE[7]]);
        last_letters.insert('t', vec![WORD_VALUE[8]]);

        let mut reader = my_reader::BufReader::open("C:\\Users\\docwa\\projects\\adventofcode2023\\src\\day1\\input.txt")
            .expect("Some shit happened");
        let mut buffer = String::new();
        let mut sum: u32 = 0;

        while let Some(line) = reader.read_line(&mut buffer) {
            let l = line
                .expect("failed to read line")
                .trim();
            let num = get_num_for_part_two(l, &first_letters, &last_letters);
            println!("{}: {}", num, l);
            sum += num;
        }

        println!("{}", sum);
    }

    fn get_num_for_part_one(s: &str) -> u32 {
        let mut num: u32 = 0;
        for c in s.chars() {
            if c.is_numeric() {
                num = c.to_digit(10).unwrap() * 10;
                break;
            }
        }
    
        for c in s.chars().rev() {
            if c.is_numeric() {
                num += c.to_digit(10).unwrap();
                break;
            }
        }
    
        num
    }

    fn get_num_for_part_two(s: &str, first_letters: &HashMap<char, Vec<WordToValue>>, last_letters: &HashMap<char, Vec<WordToValue>>) -> u32 {
        let mut num: u32 = 0;
        'outer: for (i, c) in s.chars().enumerate() {
            if c.is_numeric() {
                num = c.to_digit(10).unwrap() * 10;
                break;
            }
            
            if first_letters.contains_key(&c) {
                let to_check = Some(first_letters.get(&c))
                    .expect("lmao how the fuck")
                    .unwrap();
                for word in to_check {
                    let end_index = i + word.0.len();
                    if end_index > s.len() {
                        continue;
                    }
                    if word.0.eq(&s[i..end_index]) {
                        num = word.1 * 10;
                        break 'outer;
                    }
                }
            }
        }
    
        'outer: for (i, c) in s.chars().rev().enumerate() {
            if c.is_numeric() {
                num += c.to_digit(10).unwrap();
                break;
            }
            
            if last_letters.contains_key(&c) {
                let to_check = Some(last_letters.get(&c))
                    .expect("lmao how the fuck")
                    .unwrap();
                for word in to_check {
                    let end_index = s.len() - i; // end_index is exclusive in a slice, so no -1
                    if word.0.len() + 1 > end_index {
                        continue;
                    }
                    let start_index = end_index - word.0.len();
                    if word.0.eq(&s[start_index..end_index]) {
                        num += word.1;
                        break 'outer;
                    }
                }
            }
        }

        num
    }
}