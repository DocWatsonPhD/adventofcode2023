pub mod day2 {
    use std::collections::HashMap;
    use crate::my_reader;

    const RED: &str = "red";
    const GREEN: &str = "green";
    const BLUE: &str = "blue";

    pub fn part_one() {
        let mut color_limits: HashMap<&str, u32> = HashMap::new();
        color_limits.insert(RED, 12);
        color_limits.insert(BLUE, 14);
        color_limits.insert(GREEN, 13);

        let mut reader = my_reader::BufReader::open("C:\\Users\\docwa\\projects\\adventofcode2023\\src\\day2\\input.txt")
            .expect("Some shit happened");
        let mut buffer = String::new();
        let mut sum: u32 = 0;
    
        while let Some(line) = reader.read_line(&mut buffer) {
            let l = line
                .expect("failed to read line")
                .trim();
            sum += get_num_for_part_one(l, &color_limits);
        }
    
        println!("{}", sum);
    }

    pub fn part_two() {
        let mut color_index: HashMap<&str, usize> = HashMap::new();
        color_index.insert(RED, 0);
        color_index.insert(GREEN, 1);
        color_index.insert(BLUE, 2);

        let mut reader = my_reader::BufReader::open("C:\\Users\\docwa\\projects\\adventofcode2023\\src\\day2\\input.txt")
            .expect("Some shit happened");
        let mut buffer = String::new();
        let mut sum: u32 = 0;
    
        while let Some(line) = reader.read_line(&mut buffer) {
            let l = line
                .expect("failed to read line")
                .trim();
            sum += get_num_for_part_two(l, &color_index);
        }
    
        println!("{}", sum);
    }

    fn get_num_for_part_one(s: &str, limits: &HashMap<&str, u32>) -> u32 {
        let mut spl = s["Game ".len()..].split(": ");
        let id: u32 = str::parse(spl.next().unwrap()).unwrap();
        // spl looks something like Some("1 red, 2 blue, 3 green; 4 green, 5 red...")
        let sets: Vec<&str> = spl.next().unwrap().split("; ").collect();
        // sets is now ["1 red, 2 blue, 3 green", "4 green, 5 red", ...]
        for set in sets {
            let t = set.split(", ");
            for color_pair in t {
                // Can't use an iteration on a split because of ownership issues and blah blah
                let color_split: Vec<&str> = color_pair.split(&[' ', ',']).collect();
                let amount: u32 = str::parse(color_split[0]).unwrap();
                let color: &str = color_split[1];
                if limits.get(color).unwrap() < &amount {
                    return 0;
                }
            }
        }

        id
    }

    fn get_num_for_part_two(s: &str, color_index: &HashMap<&str, usize>) -> u32 {
        let mut rgb_max: [u32; 3] = [0, 0, 0];
        let mut spl = s["Game ".len()..].split(": ");
        spl.next(); // advance past the ID
        // spl looks something like Some("1 red, 2 blue, 3 green; 4 green, 5 red...")
        let sets: Vec<&str> = spl.next().unwrap().split("; ").collect();
        // sets is now ["1 red, 2 blue, 3 green", "4 green, 5 red", ...]
        for set in sets {
            let t = set.split(", ");
            for color_pair in t {
                // Can't use an iteration on a split because of ownership issues and blah blah
                let color_split: Vec<&str> = color_pair.split(&[' ', ',']).collect();
                let amount: u32 = str::parse(color_split[0]).unwrap();
                let color: &str = color_split[1];
                let index: usize = *color_index.get(color).unwrap();
                if amount > rgb_max[index] {
                    rgb_max[index] = amount;
                }
            }
        }

        let mut power: u32 = 1;
        for ele in rgb_max {
            if ele > 0 { power *= ele; }
        }

        power
    }
}