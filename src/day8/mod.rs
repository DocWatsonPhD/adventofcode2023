pub mod day8 {
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::{BufReader, BufRead, Result};
    
    const INPUT: &str = "C:\\Users\\docwa\\projects\\adventofcode2023\\src\\day8\\input.txt";

    type NodeMap = HashMap<String, Nodes>;

    pub fn part_one() -> Result<()> {
        let (directions, node_map, _) = get_data();

        let mut i: usize = 0;
        let mut current_key: String = "AAA".to_string();
        'outer: loop {
            for c in directions.chars() {
                i += 1;
                let next: String = get_next_key(c, &current_key, &node_map);
                if next == "ZZZ" { println!("{}", i); break 'outer; }
                else { current_key = next; }
            }
        }

        Ok(())
    }

    pub fn part_two() -> Result<()>{
        let (directions, node_map, targets) = get_data();
        
        let mut i: usize = 0;
        let mut current_keys: Vec<String> = targets.to_owned();
        let mut counts: Vec<usize> = vec![];
        'outer: loop {
            for c in directions.chars() {
                i += 1;
                let mut diff = current_keys.len();
                current_keys = get_next_keys(c, &current_keys, &node_map);
                // TODO: Do single removal(s) instead of what amounts to a copy?
                current_keys = current_keys.iter()
                    .filter(|k| k.chars().rev().nth(0).unwrap() != 'Z')
                    .map(|x| x.to_owned())
                    .collect();
                diff -= current_keys.len();
                for _j in 0..diff { counts.push(i); }
                if current_keys.len() == 0 { break 'outer; }
            }
        }

        let mut l = counts[0];
        for k in counts {
            l = num::integer::lcm(l, k);
        }

        println!("{}", l);

        Ok(())
    }

    fn get_next_key(dir: char, key: &str, node_map: &NodeMap) -> String {
        if dir == 'R' { node_map.get(key).unwrap().right.to_owned() }
        else { node_map.get(key).unwrap().left.to_owned() }
    }

    fn get_next_keys(dir: char, keys: &Vec<String>, node_map: &NodeMap) -> Vec<String> {
        keys.iter()
            .map(|k| {
                if dir == 'R' { node_map.get(k).unwrap().right.to_owned() }
                else { node_map.get(k).unwrap().left.to_owned() }
            })
            .collect()
    }
    
    #[derive(Debug)]
    struct Nodes {
        left: String,
        right: String
    }

    impl Nodes {
        pub fn new(s: String) -> Nodes {
            let mut spl = s[1..s.len()-1].split(", ");
            Nodes {
                left: spl.next().unwrap().to_owned(),
                right: spl.next().unwrap().to_owned()
            }
        }
    }

    fn get_data() -> (String, NodeMap, Vec<String>) {
        let file = File::open(INPUT)
            .expect("What a terrible thing to have happened");
        let reader = BufReader::new(file);
        let mut directions: String = String::new();
        let mut key: String;
        let mut nodes: Nodes;
        let mut node_map: NodeMap = HashMap::new();
        let mut targets: Vec<String> = vec![];

        for (i, line) in reader.lines().enumerate() {
            if i > 1 {
                let line: String = line.expect("YEEOOOOWCH!");
                let mut spl = line.split(" = ");
                key = spl.next().unwrap().to_owned();
                nodes = Nodes::new(spl.next().unwrap().to_owned());
                node_map.insert(key.to_owned(), nodes);
                if key.chars().rev().nth(0).unwrap() == 'A' { targets.push(key); }
            } else if i == 0 {
                let line: String = line.expect("Couldn't get first line");
                directions = line.trim().to_owned();
            } else {
                let _line: String = line.expect("Couldn't read the blank line");
            }
        }

        (directions, node_map, targets)
    }
}