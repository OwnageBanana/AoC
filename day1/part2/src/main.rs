// in this example I'm learning about structs and implementing functionality with the `impl` keyword
// I am doing this without much regard to a good solution to the problem of a moving window into the values of the file.

use core::panic;
use std::env;
use std::fs::{self};
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() > 1, "please provide a path to the signal file");

    let buffer: BufReader<fs::File> = match fs::File::open(&args[1]) {
        Ok(file) => BufReader::new(file),
        Err(e) => panic!("err reading file {}", e),
    };

    let mut range = Window::new();
    let mut prev_sum: u16 = 0;
    let mut count: u16 = 0;
    for res in buffer.lines() {
        let val: u16 = match res {
            #[rustfmt::skip]
            Ok(line) => line.trim().parse().expect("Please provide integers as input!"),
            Err(e) => panic!("failed to parse the line {}", e),
        };
        // this line caused a lot of issues for me because I don't yet know how to deal with the ownership model yet
        // what I think fixed it, was making range mutable so it can pass ownership and be changed without copy and making insert use a &mut self to permit mutation of the struct
        let sum = range.insert(val);
        if sum > prev_sum {
            count += 1;
        }
        prev_sum = sum;
    }
    // -2 for loop iter. 0,1 and 2 where increase in count is always true. simplifies code above but I don't think this is very pretty anymore
    // preventing any integer underflow, maybe wrapping add is better? not asserting any condition here because it really is an edge case where this window solution simply doesn't work on inputs < 4
    if count > 3 {
        count -= 3
    }
    println!("the total number of increases is: {}", count);
}

struct Window {
    values: [u16; 3],
    // I did not know you had to use usize to index an array, I need to learn the implications of this on memory management
    index: usize, //corresponds to the next write location
}

// window hold a rotating set of 3 values in it's array.
impl Window {
    fn new() -> Window {
        Window {
            values: [0, 0, 0],
            index: 0,
        }
    }
    // inserts a value and increment index
    fn insert(&mut self, val: u16) -> u16 {
        self.values[self.index] = val;
        match self.index {
            2 => self.index = 0,
            _ => self.index += 1,
        }
        return self.sum();
    }

    fn sum(&self) -> u16 {
        return self.values[0] + self.values[1] + self.values[2];
    }
}
