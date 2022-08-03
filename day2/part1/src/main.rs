use core::fmt;
use std::io::{BufRead, BufReader, Read};
use std::{env, fs};

// we see a pattern emerge
fn main() {
    let args: Vec<String> = env::args().collect();

    assert!(args.len() > 1, "please provide a file path to read");

    let Buffer: BufReader<fs::File> = match fs::File::open(&args[1]) {
        Ok(file) => BufReader::new(file),
        Err(e) => panic!("err reading file {}", e),
    };
    // this time we construct a position
    let mut pos: Position = Position::new();
    for line in Buffer.lines() {
        let vector: (Command, u16) = match line {
            Ok(command) => parse(&command),
            Err(e) => panic!("error reading line from buffer {}", e),
        };
        pos.addVector(vector);
    }
    println!("the final values are {}", pos);
}

fn parse(line: &String) -> (Command, u16) {
    let bytes = line.as_bytes();
    #[rustfmt::skip]
    // applying some of that slice knowledge from the rust book here, not to brag but it compiled and ran the first time 🤠
        let ret : (Command, u16) = match bytes[0] {
            b'f' => (Command::Forward, line[7..].trim().parse().expect("Please provide integers as input!")),
            b'd' => (Command::Down, line[4..].trim().parse().expect("Please provide integers as input!")),
            b'u' => (Command::Up, line[2..].trim().parse().expect("Please provide integers as input!")),
            _ => panic!("unexpected input {}", bytes[0])
        };
    return ret;
}
// fUN with eNUM
// really liking rust's match syntax so far
enum Command {
    Forward,
    Down,
    Up,
}

struct Position {
    depth: u16,
    horizontal: u16,
}

impl Position {
    fn new() -> Position {
        Position {
            depth: 0,
            horizontal: 0,
        }
    }
    fn add_vector(&mut self, vector: (Command, u16)) {
        match vector.0 {
            Command::Forward => self.horizontal += vector.1,
            Command::Down => self.depth += vector.1,
            Command::Up => self.depth -= vector.1,
        }
    }
}
// testing writing a display implementation. reminds me of java's ToString
// I'm left wondering if I could define this in the scope of the position struct's impl? the syntax evades me, I just followed
// rust by example for the mean time
impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Depth: {} Horizontal: {} AoC answer of {}",
            self.depth,
            self.horizontal,
            // kinda hacky use of u32.. maybe should use usize for the same memory packing but better range of values from u16 * u16 w/o overflow?
            // how do systems programmers deal with theses sorts of memory problems? probably with implementing heap allocated structs for very large integers.
            u32::from(self.depth) * u32::from(self.horizontal)
            // usize::from(self.depth) * usize::from(self.horizontal)
        )
    }
}
