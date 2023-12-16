pub mod day9 {
    use std::fs::File;
    use std::io::{BufReader, BufRead, Result};
    use std::vec;
    
    const INPUT: &str = "C:\\Users\\docwa\\projects\\adventofcode2023\\src\\day9\\input.txt";

    pub fn part_one() -> Result<()> {
        let data: Vec<Vec<i64>> = get_data();

        let mut result: i64 = 0;

        for history in data {
            let (mut diffs, mut matching) = get_diffs(&history);
            let mut sum: i64 = diffs[diffs.len() - 1] + history[history.len() - 1];
            while !matching {
                (diffs, matching) = get_diffs(&diffs);
                sum += diffs[diffs.len() - 1];
            }
            result += sum;
        }

        println!("{}", result);

        Ok(())
    }

    pub fn part_two() -> Result<()>{
        let data = get_data();

        let mut result: i64 = 0;

        for history in data {
            let mut first_entries: Vec<i64> = vec![];
            first_entries.push(history[0]);
            let (mut diffs, mut matching) = get_diffs(&history);
            first_entries.push(diffs[0]);
            while !matching {
                (diffs, matching) = get_diffs(&diffs);
                first_entries.push(diffs[0]);
            }
            let mut top_entry: i64 = 0;
            let mut last_entry: i64 = 0;
            for i in (0..first_entries.len()).rev() {
                top_entry = first_entries[i] - last_entry;
                last_entry = top_entry;
            }
            result += top_entry;
        }

        println!("{}", result);

        Ok(())
    }

    fn get_diffs(data: &Vec<i64>) -> (Vec<i64>, bool) {
        let mut result: Vec<i64> = vec![];

        let mut last_data: i64 = data[0];
        let mut match_last: bool = true;
        for (i, d) in data.iter().enumerate() {
            if i > 0 { 
                let diff = *d - last_data;
                result.push(diff); 
                last_data = *d;
                if match_last && (i > 2) && (diff != result[i - 2]) { match_last = false; }
            }
        }

        (result, match_last)
    }

    fn get_data() -> Vec<Vec<i64>> {
        let file = File::open(INPUT)
            .expect("What a terrible thing to have happened");
        let reader = BufReader::new(file);
        let mut result: Vec<Vec<i64>> = vec![];
        
        for line in reader.lines() {
            let line: String = line.expect("YEEOOOOWCH!");
            result.push(line.split(' ')
                .map(|x| str::parse::<i64>(x).unwrap())
                .collect());
        }

        result
    }
}