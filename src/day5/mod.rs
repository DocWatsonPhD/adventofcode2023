pub mod day5 {
    use std::f64::INFINITY;
    use std::fs::File;
    use std::io::{BufReader, BufRead, Result};
    use std::u64::MAX;
    use std::vec;

    #[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
    struct MapEntryD {
        destination_start: u64,
        source_start: u64,
        destination_end: u64,
        source_end: u64,
        range_length: u64,
    }

    // Re-order fields to sourt by source_start
    #[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
    struct MapEntryS {
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

    // Could probably use a trait for the duplicate code but not really worth it for this
    impl MapEntryD {
        fn new(s: &str) -> MapEntryD {
            let mut spl = s.split(' ');
            let dstart = spl.next().unwrap().trim().parse().unwrap();
            let sstart = spl.next().unwrap().trim().parse().unwrap();
            let rlength = spl.next().unwrap().trim().parse().unwrap();
            MapEntryD {
                destination_start: dstart,
                source_start: sstart,
                range_length: rlength,
                destination_end: dstart + rlength - 1,
                source_end: sstart + rlength - 1
            }
        }
    }

    impl MapEntryS {
        fn new(s: &str) -> MapEntryS {
            let mut spl = s.split(' ');
            let dstart = spl.next().unwrap().trim().parse().unwrap();
            let sstart = spl.next().unwrap().trim().parse().unwrap();
            let rlength = spl.next().unwrap().trim().parse().unwrap();
            MapEntryS {
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
        d_vec: &mut Vec<MapEntryD>,
        s_vec: &mut Vec<MapEntryS>) {
            let mut line: String = String::new();
            reader.read_line(&mut line)
                .expect("Could not read line");
            while !line.trim().is_empty() {
                let entry_d = MapEntryD::new(&line);
                let entry_s: MapEntryS = MapEntryS::new(&line);
                match d_vec.binary_search(&entry_d) {
                    Ok(_) => {} // element already in vector @ `pos` 
                    Err(pos) => d_vec.insert(pos, entry_d),
                }
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
        seed_to_soil_d: &mut Vec<MapEntryD>, seed_to_soil_s: &mut Vec<MapEntryS>,
        soil_to_fertilizer_d: &mut Vec<MapEntryD>, soil_to_fertilizer_s: &mut Vec<MapEntryS>,
        fertilizer_to_water_d: &mut Vec<MapEntryD>, fertilizer_to_water_s: &mut Vec<MapEntryS>,
        water_to_light_d: &mut Vec<MapEntryD>, water_to_light_s: &mut Vec<MapEntryS>,
        light_to_temperature_d: &mut Vec<MapEntryD>, light_to_temperature_s: &mut Vec<MapEntryS>,
        temperature_to_humidity_d: &mut Vec<MapEntryD>, temperature_to_humidity_s: &mut Vec<MapEntryS>,
        humidity_to_location_d: &mut Vec<MapEntryD>, humidity_to_location_s: &mut Vec<MapEntryS> ) 
        -> Result<Vec<u64>> {
        let file = File::open("C:\\Users\\docwa\\projects\\adventofcode2023\\src\\day5\\test_input.txt")
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
        read_batch(&mut reader, seed_to_soil_d, seed_to_soil_s);

        reader.read_line(&mut line)?; // "soil-to-fertilizer map: "
        line.clear();
        read_batch(&mut reader, soil_to_fertilizer_d, soil_to_fertilizer_s);
        
        reader.read_line(&mut line)?; // "fertilizer-to-water map: "
        line.clear();
        read_batch(&mut reader, fertilizer_to_water_d, fertilizer_to_water_s);

        reader.read_line(&mut line)?; // "water-to-light map: "
        line.clear();
        read_batch(&mut reader, water_to_light_d, water_to_light_s);

        reader.read_line(&mut line)?; // "light-to-temperature map: "
        line.clear();
        read_batch(&mut reader, light_to_temperature_d, light_to_temperature_s);

        reader.read_line(&mut line)?; // "temperature-to-humidity map: "
        line.clear();
        read_batch(&mut reader, temperature_to_humidity_d, temperature_to_humidity_s);

        reader.read_line(&mut line)?; // "humidity-to-location map: "
        line.clear();
        read_batch(&mut reader, humidity_to_location_d, humidity_to_location_s);

        Ok(seeds)
    }

    pub fn part_one() -> Result<()> {
        let mut seed_to_soil_d: Vec<MapEntryD> = vec![];
        let mut seed_to_soil_s: Vec<MapEntryS> = vec![];
        let mut soil_to_fertilizer_d: Vec<MapEntryD> = vec![];
        let mut soil_to_fertilizer_s: Vec<MapEntryS> = vec![];
        let mut fertilizer_to_water_d: Vec<MapEntryD> = vec![];
        let mut fertilizer_to_water_s: Vec<MapEntryS> = vec![];
        let mut water_to_light_d: Vec<MapEntryD> = vec![];
        let mut water_to_light_s: Vec<MapEntryS> = vec![];
        let mut light_to_temperature_d: Vec<MapEntryD> = vec![];
        let mut light_to_temperature_s: Vec<MapEntryS> = vec![];
        let mut temperature_to_humidity_d: Vec<MapEntryD> = vec![];
        let mut temperature_to_humidity_s: Vec<MapEntryS> = vec![];
        let mut humidity_to_location_d: Vec<MapEntryD> = vec![];
        let mut humidity_to_location_s: Vec<MapEntryS> = vec![];

        let seeds = match fill_vecs(
            &mut seed_to_soil_d, &mut seed_to_soil_s,
            &mut soil_to_fertilizer_d, &mut soil_to_fertilizer_s,
            &mut fertilizer_to_water_d, &mut fertilizer_to_water_s,
            &mut water_to_light_d, &mut water_to_light_s,
            &mut light_to_temperature_d, &mut light_to_temperature_s,
            &mut temperature_to_humidity_d, &mut temperature_to_humidity_s,
            &mut humidity_to_location_d, &mut humidity_to_location_s) {
                Ok(val) => val,
                Err(_) => vec![]
            };

        // drill down from seed to location
        let mut min_location: u64 = MAX;
        for seed in seeds {
            // println!("seed {}", seed);
            let soil = get_mapped_dest_value(seed, &seed_to_soil_s);
            // println!("\tsoil {}", soil);
            let fertilizer = get_mapped_dest_value(soil, &soil_to_fertilizer_s);
            // println!("\tfertilizer {}", fertilizer);
            let water = get_mapped_dest_value(fertilizer, &fertilizer_to_water_s);
            // println!("\twater {}", water);
            let light = get_mapped_dest_value(water, &water_to_light_s);
            // println!("\tlight {}", light);
            let temperature = get_mapped_dest_value(light, &light_to_temperature_s);
            // println!("\ttemperature {}", temperature);
            let humidity = get_mapped_dest_value(temperature, &temperature_to_humidity_s);
            // println!("\thumidity {}", humidity);
            let location = get_mapped_dest_value(humidity, &humidity_to_location_s);
            // println!("\tlocation {}", location);
            if location < min_location { min_location = location; }
        }

        println!("{}", min_location);

        Ok(())
    }

    fn get_mapped_dest_value(target: u64, vec_s: &Vec<MapEntryS>) -> u64 {
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

    fn get_mapped_source_value(target: u64, vec_d: &Vec<MapEntryD>) -> u64 {
        let mut min_index: usize = 0;
        let mut max_index = vec_d.len() - 1;
        let mut i: usize = max_index / 2;
        let mut d_low: u64;
        let mut d_hi: u64;
        loop {
            d_low = vec_d[i].destination_start;
            d_hi = vec_d[i].destination_end;
            if target < d_low {
                if i == 0 { break; }
                max_index = i - 1;
                i = (max_index - min_index) / 2;
                continue;
            }
            if target > d_hi {
                min_index = i;
                i = (max_index + min_index).div_ceil(2);
                if max_index == min_index || i == vec_d.len() { break; }
                continue;
            }

            return vec_d[i].source_start + (target - d_low); 
        }

        target
    }

    pub fn part_two() -> Result<()>{
        let mut seed_to_soil_d: Vec<MapEntryD> = vec![];
        let mut seed_to_soil_s: Vec<MapEntryS> = vec![];
        let mut soil_to_fertilizer_d: Vec<MapEntryD> = vec![];
        let mut soil_to_fertilizer_s: Vec<MapEntryS> = vec![];
        let mut fertilizer_to_water_d: Vec<MapEntryD> = vec![];
        let mut fertilizer_to_water_s: Vec<MapEntryS> = vec![];
        let mut water_to_light_d: Vec<MapEntryD> = vec![];
        let mut water_to_light_s: Vec<MapEntryS> = vec![];
        let mut light_to_temperature_d: Vec<MapEntryD> = vec![];
        let mut light_to_temperature_s: Vec<MapEntryS> = vec![];
        let mut temperature_to_humidity_d: Vec<MapEntryD> = vec![];
        let mut temperature_to_humidity_s: Vec<MapEntryS> = vec![];
        let mut humidity_to_location_d: Vec<MapEntryD> = vec![];
        let mut humidity_to_location_s: Vec<MapEntryS> = vec![];

        let seeds = 
        match fill_vecs(
            &mut seed_to_soil_d, &mut seed_to_soil_s,
            &mut soil_to_fertilizer_d, &mut soil_to_fertilizer_s,
            &mut fertilizer_to_water_d, &mut fertilizer_to_water_s,
            &mut water_to_light_d, &mut water_to_light_s,
            &mut light_to_temperature_d, &mut light_to_temperature_s,
            &mut temperature_to_humidity_d, &mut temperature_to_humidity_s,
            &mut humidity_to_location_d, &mut humidity_to_location_s) {
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
        // Commented out because it doesn't work.
        // Initially planned to brute-force every number within ranges inside seeds,
        // but there are way too many to run through that the program takes a helluva
        // long time to run. Probably gotta re-approach the problem.
        // for seed in &seeds {
        //     let sr = get_seed_range_for_soil(seed, &seed_to_soil_s);
        //     for seed_value in sr {
        //     // for seed_value in seed.source_start..seed.source_end {
        //         // println!("seed {}", seed_value);
        //         let soil = get_mapped_dest_value(seed_value, &seed_to_soil_s);
        //         // println!("\tsoil {}", soil);
        //         let fertilizer = get_mapped_dest_value(soil, &soil_to_fertilizer_s);
        //         // println!("\tfertilizer {}", fertilizer);
        //         let water = get_mapped_dest_value(fertilizer, &fertilizer_to_water_s);
        //         // println!("\twater {}", water);
        //         let light = get_mapped_dest_value(water, &water_to_light_s);
        //         // println!("\tlight {}", light);
        //         let temperature = get_mapped_dest_value(light, &light_to_temperature_s);
        //         // println!("\ttemperature {}", temperature);
        //         let humidity = get_mapped_dest_value(temperature, &temperature_to_humidity_s);
        //         // println!("\thumidity {}", humidity);
        //         let location = get_mapped_dest_value(humidity, &humidity_to_location_s);
        //         // println!("\tlocation {}", location);
        //         if location < min_location { min_location = location; }
        //     }
        // }
        
        println!("{}", min_location);

        Ok(())
    }
    
    // Just returns every boundary where the seed entry matches to the given seed_to_soil map.
    // Unfortunately this doesn't work because even in test data the solution lied within the bounds
    fn get_seed_range_for_soil(seed: &SeedEntry, seed_to_soil_s: &Vec<MapEntryS>) -> Vec<u64> {
        let seed_low: u64 = seed.source_start;
        let seed_hi: u64 = seed.source_end;
        let mut result: Vec<u64> = vec![];
        for soil in seed_to_soil_s {
            let seed_low_bounded: bool = seed_low >= soil.source_start && seed_low <= soil.source_end;
            let seed_hi_bounded: bool = seed_hi >= soil.source_start && seed_hi <= soil.source_end;
            if seed_low_bounded && seed_hi_bounded { 
                result.push(soil.source_start.max(seed_low));
                result.push(soil.source_end.min(seed_hi));
            } else if seed_low_bounded && !seed_hi_bounded {
                result.push(soil.source_start.max(seed_low));
                result.push(soil.source_end);
            } else if !seed_low_bounded && seed_hi_bounded {
                result.push(soil.source_start);
                result.push(soil.source_end.min(seed_hi));
            } else if soil.source_start > seed_low && soil.source_end < seed_hi {
                if seed_hi < soil.source_start || seed_low > soil.source_end {
                    continue;
                }
                result.push(soil.source_start);
                result.push(soil.source_end);
            }
        }

        result
    }
}