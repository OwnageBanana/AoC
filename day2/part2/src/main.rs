//! part 2 commentary: I got to copy basically all my code and simply adjust my struct to add aim and adjust the addVector function
//! any further new commentary will be in ! comments and inline in those areas
use core::fmt;
use std::io::{BufRead, BufReader};
use std::{env, fs};

// we see a pattern emerge
fn main() {
    let args: Vec<String> = env::args().collect();

    assert!(args.len() > 1, "please provide a file path to read");

    let buffer: BufReader<fs::File> = match fs::File::open(&args[1]) {
        Ok(file) => BufReader::new(file),
        Err(e) => panic!("err reading file {}", e),
    };
    // this time we construct a position
    let mut pos: Position = Position::new();
    for line in buffer.lines() {
        let vector: (Command, u64) = match line {
            Ok(command) => parse(&command),
            Err(e) => panic!("error reading line from buffer {}", e),
        };
        pos.add_vector(vector);
    }
    println!("the final values are {}", pos);
}

fn parse(line: &String) -> (Command, u64) {
    let bytes = line.as_bytes();
    #[rustfmt::skip]
    // applying some of that slice knowledge from the rust book here, not to brag but it compiled and ran the first time 🤠
        let ret : (Command, u64) = match bytes[0] {
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
// ! too much integer overflow we're pulling out the big boy integers now
struct Position {
    aim: u64,
    depth: u64,
    horizontal: u64,
}

impl Position {
    fn new() -> Position {
        Position {
            aim: 0,
            depth: 0,
            horizontal: 0,
        }
    }
    fn add_vector(&mut self, vector: (Command, u64)) {
        match vector.0 {
            // ! intuited a little more match syntax here: needing braces for multiple LoC. Funny the formatter doesn't use the comma at the closing brace and even the last semicolon isn't required.
            // ! I'd like it to just stay consistent in syntax but I can see why they aren't required so technically its better to remove them.. both are valid syntax so it should probably leave the code I write unless there's some more advanced syntax that could break my assumption about why it may do this
            // ! I find myself telling the rust formatter to screw off a lot, I'm getting the familiar taste of the golang experience in my mouth, but at least this is logical and clean
            // ! still going to use my preferred syntax in my project though. Fight me rustfmt, may the better program win.
            #[rustfmt::skip]
            Command::Forward => {
                self.horizontal += vector.1;
                self.depth += vector.1 * self.aim;
            }
            // ! now we adjust aim
            Command::Down => self.aim += vector.1,
            Command::Up => self.aim -= vector.1,
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
            // ! addition of aim
            "Aim: {}
            Depth: {} Horizontal: {}
            AoC answer of {}",
            self.aim,
            self.depth,
            self.horizontal,
            // kinda hacky use of u32.. maybe should use usize for the same memory packing but better range of values from u16 * u16 w/o overflow?
            // how do systems programmers deal with theses sorts of memory problems? probably with implementing heap allocated structs for very large integers.
            self.depth * self.horizontal,
            // usize::from(self.depth) * usize::from(self.horizontal)
        )
    }
}
