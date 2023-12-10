pub mod day5 {
    use std::fs::File;
    use std::io::{BufReader, BufRead, Result};
    use std::u64::MAX;
    use std::vec;

    // Re-order fields to sourt by source_start
    #[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
    struct MapEntry {
        source_start: u64,
        destination_start: u64,
        source_end: u64,
        destination_end: u64,
        range_length: u64
    }

    #[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
    struct SeedEntry {
        source_start: u64,
        source_end: u64,
        range_length: u64
    }

    impl SeedEntry {
        fn new(sl: &[u64]) -> SeedEntry {
            let sstart: u64 = sl[0];
            let rlength: u64 = sl[1];
            SeedEntry {
                source_start: sstart,
                source_end: sstart + rlength - 1,
                range_length: rlength
            }
        }
    }

    impl MapEntry {
        fn new(s: &str) -> MapEntry {
            let mut spl = s.split(' ');
            let dstart = spl.next().unwrap().trim().parse().unwrap();
            let sstart = spl.next().unwrap().trim().parse().unwrap();
            let rlength = spl.next().unwrap().trim().parse().unwrap();
            MapEntry {
                destination_start: dstart,
                source_start: sstart,
                range_length: rlength,
                destination_end: dstart + rlength - 1,
                source_end: sstart + rlength - 1
            }
        }
    }

    fn read_batch(
        reader: &mut BufReader<File>,
        s_vec: &mut Vec<MapEntry>) {
            let mut line: String = String::new();
            reader.read_line(&mut line)
                .expect("Could not read line");
            while !line.trim().is_empty() {
                let entry_s: MapEntry = MapEntry::new(&line);
                match s_vec.binary_search(&entry_s) {
                    Ok(_) => {} // element already in vector @ `pos` 
                    Err(pos) => s_vec.insert(pos, entry_s),
                }
                line.clear();
                reader.read_line(&mut line)
                    .expect("Could not read line");
            }
        }

    fn fill_vecs(
        seed_to_soil: &mut Vec<MapEntry>,
        soil_to_fertilizer: &mut Vec<MapEntry>,
        fertilizer_to_water: &mut Vec<MapEntry>,
        water_to_light: &mut Vec<MapEntry>,
        light_to_temperature: &mut Vec<MapEntry>,
        temperature_to_humidity: &mut Vec<MapEntry>,
        humidity_to_location: &mut Vec<MapEntry> ) 
        -> Result<Vec<u64>> {
        let file = File::open("C:\\Users\\docwa\\projects\\adventofcode2023\\src\\day5\\input.txt")
            .expect("What a terrible thing to have happened");
        let mut reader = BufReader::new(file);

        let mut line: String = String::new();
        reader.read_line(&mut line)?;
        let seeds: Vec<u64> = line["seeds: ".len()..].split(' ')
            .map(|x| {
                str::parse(&x.trim()).unwrap()
            })
            .collect();

        reader.read_line(&mut line)?; // blank line
        reader.read_line(&mut line)?; // "seed-to-soil map:"
        line.clear();
        read_batch(&mut reader, seed_to_soil);

        reader.read_line(&mut line)?; // "soil-to-fertilizer map: "
        line.clear();
        read_batch(&mut reader, soil_to_fertilizer);
        
        reader.read_line(&mut line)?; // "fertilizer-to-water map: "
        line.clear();
        read_batch(&mut reader, fertilizer_to_water);

        reader.read_line(&mut line)?; // "water-to-light map: "
        line.clear();
        read_batch(&mut reader, water_to_light);

        reader.read_line(&mut line)?; // "light-to-temperature map: "
        line.clear();
        read_batch(&mut reader, light_to_temperature);

        reader.read_line(&mut line)?; // "temperature-to-humidity map: "
        line.clear();
        read_batch(&mut reader, temperature_to_humidity);

        reader.read_line(&mut line)?; // "humidity-to-location map: "
        line.clear();
        read_batch(&mut reader, humidity_to_location);

        Ok(seeds)
    }

    pub fn part_one() -> Result<()> {
        let mut seed_to_soil: Vec<MapEntry> = vec![];
        let mut soil_to_fertilizer: Vec<MapEntry> = vec![];
        let mut fertilizer_to_water: Vec<MapEntry> = vec![];
        let mut water_to_light: Vec<MapEntry> = vec![];
        let mut light_to_temperature: Vec<MapEntry> = vec![];
        let mut temperature_to_humidity: Vec<MapEntry> = vec![];
        let mut humidity_to_location: Vec<MapEntry> = vec![];

        let seeds = match fill_vecs(
            &mut seed_to_soil,
            &mut soil_to_fertilizer,
            &mut fertilizer_to_water,
            &mut water_to_light,
            &mut light_to_temperature,
            &mut temperature_to_humidity,
            &mut humidity_to_location) {
                Ok(val) => val,
                Err(_) => vec![]
            };

        // drill down from seed to location
        let mut min_location: u64 = MAX;
        for seed in seeds {
            let soil = get_mapped_dest_value(seed, &seed_to_soil);
            let fertilizer = get_mapped_dest_value(soil, &soil_to_fertilizer);
            let water = get_mapped_dest_value(fertilizer, &fertilizer_to_water);
            let light = get_mapped_dest_value(water, &water_to_light);
            let temperature = get_mapped_dest_value(light, &light_to_temperature);
            let humidity = get_mapped_dest_value(temperature, &temperature_to_humidity);
            let location = get_mapped_dest_value(humidity, &humidity_to_location);
            if location < min_location { min_location = location; }
        }

        println!("{}", min_location);

        Ok(())
    }

    fn get_mapped_dest_value(target: u64, vec_s: &Vec<MapEntry>) -> u64 {
        let mut min_index: usize = 0;
        let mut max_index = vec_s.len() - 1;
        let mut i: usize = max_index / 2;
        let mut s_low: u64;
        let mut s_hi: u64;
        loop {
            s_low = vec_s[i].source_start;
            s_hi = vec_s[i].source_end;
            if target < s_low {
                if i == 0 { break; }
                max_index = i - 1;
                i = (max_index - min_index) / 2;
                continue;
            }
            if target > s_hi {
                min_index = i;
                i = (max_index + min_index).div_ceil(2);
                if max_index == min_index || i == vec_s.len() { break; }
                continue;
            }

            return vec_s[i].destination_start + (target - s_low); 
        }

        target
    }

    pub fn part_two() -> Result<()>{
        let mut seed_to_soil: Vec<MapEntry> = vec![];
        let mut soil_to_fertilizer: Vec<MapEntry> = vec![];
        let mut fertilizer_to_water: Vec<MapEntry> = vec![];
        let mut water_to_light: Vec<MapEntry> = vec![];
        let mut light_to_temperature: Vec<MapEntry> = vec![];
        let mut temperature_to_humidity: Vec<MapEntry> = vec![];
        let mut humidity_to_location: Vec<MapEntry> = vec![];

        let seeds = 
        match fill_vecs(
            &mut seed_to_soil,
            &mut soil_to_fertilizer,
            &mut fertilizer_to_water,
            &mut water_to_light,
            &mut light_to_temperature,
            &mut temperature_to_humidity,
            &mut humidity_to_location) {
                Ok(val) => val,
                Err(_) => vec![]
        };

        // Seeds is now considered pairs of values describing ranges
        let mut seeds: Vec<SeedEntry> = seeds
            .chunks(2)
            .map(|x| SeedEntry::new(x))
            .collect();
        seeds.sort();

        // drill down from seed to location
        let mut min_location: u64 = MAX;
        for seed in &seeds {
            let seed_ranges = get_seed_range_for_soil(seed, &seed_to_soil);
            for seed_range in seed_ranges {
                for seed_value in seed_range.0..seed_range.1 {
                    let soil = get_mapped_dest_value(seed_value, &seed_to_soil);
                    let fertilizer = get_mapped_dest_value(soil, &soil_to_fertilizer);
                    let water = get_mapped_dest_value(fertilizer, &fertilizer_to_water);
                    let light = get_mapped_dest_value(water, &water_to_light);
                    let temperature = get_mapped_dest_value(light, &light_to_temperature);
                    let humidity = get_mapped_dest_value(temperature, &temperature_to_humidity);
                    let location = get_mapped_dest_value(humidity, &humidity_to_location);
                    if location < min_location { min_location = location; }
                }
            }
        }
        
        println!("{}", min_location);

        Ok(())
    }
    
    // Just returns every overlapping range where the seed entry matches to the given seed_to_soil map.
    fn get_seed_range_for_soil(seed: &SeedEntry, seed_to_soil: &Vec<MapEntry>) -> Vec<(u64, u64)> {
        let seed_low: u64 = seed.source_start;
        let seed_hi: u64 = seed.source_end;
        let mut result: Vec<(u64, u64)> = vec![];
        for soil in seed_to_soil {
            let seed_low_bounded: bool = seed_low >= soil.source_start && seed_low <= soil.source_end;
            let seed_hi_bounded: bool = seed_hi >= soil.source_start && seed_hi <= soil.source_end;
            if seed_low_bounded && seed_hi_bounded { 
                result.push((soil.source_start.max(seed_low),
                             soil.source_end.min(seed_hi)));
            } else if seed_low_bounded && !seed_hi_bounded {
                result.push((soil.source_start.max(seed_low),
                             soil.source_end));
            } else if !seed_low_bounded && seed_hi_bounded {
                result.push((soil.source_start,
                             soil.source_end.min(seed_hi)));
            } else if soil.source_start > seed_low && soil.source_end < seed_hi {
                if seed_hi < soil.source_start || seed_low > soil.source_end {
                    continue;
                }
                result.push((soil.source_start,
                             soil.source_end));
            }
        }

        result
    }
}