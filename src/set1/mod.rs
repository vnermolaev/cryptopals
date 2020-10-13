use std::fs::File;
use std::io::{BufRead, BufReader};

mod ch1;
mod ch2;
mod ch3;
mod ch4;
mod ch5;
mod ch6;
mod ch7;
mod ch8;

fn read_base64_file_content(path: &str) -> Vec<u8> {
    let file = File::open(path).expect("Must read");
    let reader = BufReader::new(file);

    let content = reader
        .lines()
        .into_iter()
        .map(|line| line.expect("Line must be defined"))
        .collect::<String>();

    base64::decode(content).expect("Must decode")
}
