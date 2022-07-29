use std::env;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Read};

fn main() {
    let mut args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    if args.len() < 2 {
        panic!("expected argument: please provide a file path to read from");
    }
    let buffer: BufReader<fs::File> = match fs::File::open(&args[1]) {
        Ok(file) => BufReader::new(file),
        Err(e) => panic!("err reading file {}", e),
    };
    let mut count: u16 = 0;
    let mut prev: u16 = 0;
    for res in buffer.lines() {
        let cur = match res {
            Ok(line) => line.trim().parse::<u16>().expect("need an integer please!"),
            Err(e) => panic!("err reading file {}", e),
        };
        if cur > prev {
            println!("{} increased", cur);
            count += 1;
        } else {
            println!("{} decreased", cur);
        }
        prev = cur;
    }
    // -1 for loop iter. 0 where increase is always true. simplifies code, and prettier :)
    if count != 0 {
        count -= 1;
    }
    println!("the total number of increases is: {}", count);
}
