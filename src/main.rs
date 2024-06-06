use std::{
    fs::File,
    io::{BufReader, Seek},
};

use p3::{p1, p2};

fn main() {
    let f = File::open("input.txt").expect("can't open file");
    let mut f = BufReader::new(f);
    let result = p1(&mut f);
    println!("{}", result);
    f.rewind().expect("rewind failed");
    let result = p2(&mut f);
    println!("{}", result);
}

