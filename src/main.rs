use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

mod d9;
use d9::{p1, p2};

fn main() {
    let file = File::open("inputs/d9.txt").expect("no such file");

    let buf = BufReader::new(file);

    let lines: Vec<String> = buf
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();

    println!("{:?}", p1(&lines));
    println!("{:?}", p2(&lines));
}
