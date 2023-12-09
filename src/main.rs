use std::time::Instant;

mod day1;
mod day2;
mod day3;

fn main() {
    use day1::day1;
    use day2::day2;
    use day3::day3;

    println!("Part 1");
    let mut now = Instant::now();

    day3::part_one();

    let mut elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    println!("Part 2");
    now = Instant::now();
    day3::part_two();
    elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

mod my_reader {
    use std::{
        fs::File,
        io::{self, prelude::*},
    };

    pub struct BufReader {
        reader: io::BufReader<File>,
    }

    impl BufReader {
        pub fn open(path: impl AsRef<std::path::Path>) -> io::Result<Self> {
            let file = File::open(path)?;
            let reader = io::BufReader::new(file);

            Ok(Self { reader })
        }

        pub fn read_line<'buf>(
            &mut self,
            buffer: &'buf mut String,
        ) -> Option<io::Result<&'buf mut String>> {
            buffer.clear();

            self.reader
                .read_line(buffer)
                .map(|u| if u == 0 { None } else { Some(buffer) })
                .transpose()
        }
    }
}