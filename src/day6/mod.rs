pub mod day6 {
    use std::fs::File;
    use std::io::{BufReader, BufRead, Result};
    
    const INPUT: &str = "C:\\Users\\docwa\\projects\\adventofcode2023\\src\\day6\\input.txt";

    pub fn part_one() -> Result<()> {
        let (times, distances) = get_data_p1();
        let mut result: u64 = 1;

        for i in 0..times.len() {
            result *= get_number_of_wins(times[i], distances[i]);
        }

        println!("{}", result);

        Ok(())
    }

    pub fn part_two() -> Result<()>{
        let (time, distance) = get_data_p2();

        println!("{}", get_number_of_wins(time, distance));

        Ok(())
    }

    fn get_data_p1() -> (Vec<u64>, Vec<u64>) {
        let file = File::open(INPUT)
            .expect("What a terrible thing to have happened");
        let mut reader = BufReader::new(file);
        let mut line: String = String::new();
        
        let _ = reader.read_line(&mut line);
        let times: Vec<u64> = line["Time:".len()..].trim().split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.trim().parse().unwrap())
            .collect();

        line.clear();
        let _ = reader.read_line(&mut line);
        let distances: Vec<u64> = line["Distance:".len()..].trim().split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.trim().parse().unwrap())
            .collect();

        (times, distances)
    }

    fn get_data_p2() -> (u64, u64) {
        let file = File::open(INPUT)
            .expect("What a terrible thing to have happened");
        let mut reader = BufReader::new(file);
        let mut line: String = String::new();
        
        let _ = reader.read_line(&mut line);
        let time: u64 = line["Time:".len()..].trim().replace(r" ", "").parse().unwrap();

        line.clear();
        let _ = reader.read_line(&mut line);
        let distance: u64 = line["Distance:".len()..].trim().replace(r" ", "").parse().unwrap();

        (time, distance)
    }

    fn get_number_of_wins(time: u64, distance: u64) -> u64 {
        // The calculated distance for this problem is always the same:
        // Distance = (TotalTime - TimeHeld) * TimeHeld.
        // This means that the total number of different distances is the same on
        // either side of the slope approaching max distance or going away from it.
        let max_index: u64 = time / 2;
        let mut right_index: u64 = max_index;
        // We need to solve for
        // Distance < (TotalTime - TimeHeld) * TimeHeld
        let mut left_index: u64 = 0;
        let mut i = time / 4;
        let mut d: u64;

        loop {
            d = get_total_distance(i, time);
            if d > distance {
                if i == 0 || get_total_distance(i - 1, time) <= distance {
                    break;
                }
                right_index = i;
                i = (right_index - left_index) / 2;
            } else {
                if i == right_index || get_total_distance(i + 1, time) > distance {
                    i += 1;
                    break;
                }
                left_index = i;
                i = (right_index + left_index).div_ceil(2);
            }
        }

        time - (2 * i) + 1
    }

    fn get_total_distance(time_held: u64, race_time: u64) -> u64 {
        (race_time - time_held) * time_held
    }
}