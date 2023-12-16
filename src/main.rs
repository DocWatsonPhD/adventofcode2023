use std::time::Instant;

mod day9;

fn main() {
    use day9::day9;

    println!("Part 1");
    let mut now = Instant::now();
    let _ = day9::part_one();
    let mut elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    println!("Part 2");
    now = Instant::now();
    let _ = day9::part_two();
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