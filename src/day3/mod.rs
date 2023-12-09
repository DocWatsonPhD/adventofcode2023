pub mod day3 {
    use std::fs::File;
    use std::io::{BufReader, BufRead, Result};
    // Now THIS is pod racing
    const LINE_BLANK: &str = "............................................................................................................................................";

    fn is_part(c: char) -> bool {
        !(c.is_alphanumeric() || c == '.')
    }

    fn get_gear_num(cache: &[String; 3], i: usize) -> u32 {
        let mut count: u8 = 0;
        let mut res: u32 = 1;
        let mut adj_top: u8 = 0;
        let mut adj_mid: u8 = 0;
        let mut adj_bot: u8 = 0;
        let start: usize = if i > 0 { i - 1 } else { i };
        let end: usize = if i + 1 < cache[1].len() { i + 1 } else { i };
        // check immediate left and right
        if cache[1].chars().nth(start).unwrap().is_digit(10) { adj_mid = 0b100; count += 1; };
        if cache[1].chars().nth(end).unwrap().is_digit(10) { adj_mid |= 1; count += 1; };
        // check above and below, but account for longer numbers that could
        // have adjacency with 2 or more digits of the same number
        for i in start..(end + 1) {
            adj_top = adj_top << 1;
            adj_bot = adj_bot << 1;
            if cache[0].chars().nth(i).unwrap().is_digit(10) { adj_top |= 1 };
            if cache[2].chars().nth(i).unwrap().is_digit(10) { adj_bot |= 1 };
        }
        // 0b000 means no adjacencies, and 0b101 means 2. All other cases mean 1 adjacent number.
        if adj_top != 0 && adj_top != 5 { count += 1; }
        if adj_bot != 0 && adj_bot != 5 { count += 1; }
        if adj_top == 5 { count += 2; }
        if adj_bot == 5 { count += 2; }

        if count != 2 { return 0; }
        // Cases for all one row
        if adj_top == 5 { return get_num_at_index(&cache[0], i - 1) * get_num_at_index(&cache[0], i + 1); }
        if adj_mid == 5 { return get_num_at_index(&cache[1], i - 1) * get_num_at_index(&cache[1], i + 1); }
        if adj_bot == 5 { return get_num_at_index(&cache[2], i - 1) * get_num_at_index(&cache[2], i + 1); }

        // Top cases
        if adj_top & 0b100 > 0 { res *= get_num_at_index(&cache[0], i - 1); }
        else if adj_top & 0b010 > 0 { res *= get_num_at_index(&cache[0], i); }
        else if adj_top & 0b001 > 0 { res *= get_num_at_index(&cache[0], i + 1); }
        // Mid cases
        if adj_mid & 0b100 > 0 { res *= get_num_at_index(&cache[1], i - 1); }
        else if adj_mid & 0b001 > 0 { res *= get_num_at_index(&cache[1], i + 1); }
        // Bottom cases
        if adj_bot & 0b100 > 0 { res *= get_num_at_index(&cache[2], i - 1); }
        else if adj_bot & 0b010 > 0 { res *= get_num_at_index(&cache[2], i); }
        else if adj_bot & 0b001 > 0 { res *= get_num_at_index(&cache[2], i + 1); }

        res
    }

    fn get_num_at_index(s: &str, start: usize) -> u32 {
        let mut first_digit_idx: usize = start;
        for j in (0..start).rev() {
            let c = s.chars().nth(j).unwrap();
            if c.is_digit(10) {
                first_digit_idx = j;
                continue;
            } else {
                break;
            }
        }

        find_num_right(s, first_digit_idx).0
    }

    fn find_num_right(s: &str, start_idx: usize) -> (u32, u32) {
        // Error handling is for slowbros. I am Speed (and I already 
        // checked for is_digit before calling this)
        let c = s.chars().nth(start_idx).unwrap();
        let mut num: u32 = c.to_digit(10).unwrap();
        let mut bits: u32 = 1;
        for j in (start_idx + 1)..s.len() {
            let c = s.chars().nth(j).unwrap();
            if c.is_digit(10) {
                let n: u32 = c.to_digit(10).unwrap();
                num *= 10;
                num += n;
                bits += 1;
            } else {
                break;
            }
        }
        (num, bits)
    }

    pub fn part_one() -> Result<()> {
        let file = File::open("C:\\Users\\docwa\\projects\\adventofcode2023\\src\\day3\\input.txt")
            .expect("What a terrible thing to have happened");
        let reader = BufReader::new(file);
        let mut sum: u32 = 0;
        let mut cache: [String; 3] = [LINE_BLANK.to_owned(), LINE_BLANK.to_owned(), LINE_BLANK.to_owned()];
        
        for line in reader.lines() {
            let line: String = line?;
            cache[0] = cache[1].to_owned();
            cache[1] = cache[2].to_owned();
            cache[2] = line;
            sum += get_num_for_part_one(&mut cache);
        }
        // After reading the last line, cache will have lines [n-2, n-1, n].
        // Advance the cache by 1 and run again to get the straggler line
        cache[0] = cache[1].to_owned();
        cache[1] = cache[2].to_owned();
        cache[2] = LINE_BLANK.to_owned();
        sum += get_num_for_part_one(&mut cache);
    
        println!("{}", sum);

        Ok(())
    }

    pub fn part_two() {
        let file = File::open("C:\\Users\\docwa\\projects\\adventofcode2023\\src\\day3\\input.txt")
            .expect("What a terrible thing to have happened");
        let reader = BufReader::new(file);
        let mut sum: u32 = 0;
        let mut cache: [String; 3] = [LINE_BLANK.to_owned(), LINE_BLANK.to_owned(), LINE_BLANK.to_owned()];
            // [String::from(".........."), String::from(".........."), String::from("..........")];
        
        for line in reader.lines() {
            let line: String = line.expect("blah").clone();
            cache[0] = cache[1].to_owned();
            cache[1] = cache[2].to_owned();
            cache[2] = line;
            sum += get_num_for_part_two(&cache);
        }
        // After reading the last line, cache will have lines [n-2, n-1, n].
        // Advance the cache by 1 and run again to get the straggler line
        cache[0] = cache[1].to_owned();
        cache[1] = cache[2].to_owned();
        cache[2] = LINE_BLANK.to_owned();
        sum += get_num_for_part_two(&cache);
    
        println!("{}", sum);
    }

    fn get_num_for_part_one(cache: &[String; 3]) -> u32 {
        let mut num: u32 = 0;
        let mut skip: u32 = 0;
        'outer: for (i, c) in cache[1].chars().enumerate() {
            if skip > 0 {
                skip -= 1;
                continue;
            }
            if c.is_digit(10) {
                let (n, b) = find_num_right(&cache[1], i);
                skip = b - 1; // account for the digit we're on already
                let start: usize;
                let end: usize;
                if i > 0 {
                    start = i - 1;
                    if is_part(cache[1].chars().nth(start).unwrap()) {
                        num += n;
                        continue;
                    }
                } else { 
                    start = i;
                };

                if (i + (b as usize)) < cache[1].len() { 
                    end = i + (b as usize);
                    if is_part(cache[1].chars().nth(end).unwrap()) {
                        num += n;
                        continue;
                    }
                }
                else { 
                    end = i + (b as usize) - 1; 
                };
                
                for j in start..(end + 1) {
                    if is_part(cache[0].chars().nth(j).unwrap()) {
                        num += n;
                        continue 'outer;
                    }
                    if is_part(cache[2].chars().nth(j).unwrap()) {
                        num += n;
                        continue 'outer;
                    }
                }
            }
        }

        num
    }

    fn get_num_for_part_two(cache: &[String; 3]) -> u32 {
        let mut num: u32 = 0;
        // println!("\nCache\n{:?}\n{:?}\n{:?}", cache[0], cache[1], cache[2]);

        for (i, c) in cache[1].chars().enumerate() {
            if c == '*' {
                num += get_gear_num(cache, i);
            }
        }

        num
    }
}