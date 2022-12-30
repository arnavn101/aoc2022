use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

mod d2;

fn main() {
    let file = File::open("inputs/d2.txt").expect("no such file");

    let buf = BufReader::new(file);

    let lines: Vec<String> = buf
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();

    println!("{:?}", d2::p1(&lines));
    println!("{:?}", d2::p2(&lines));
}
