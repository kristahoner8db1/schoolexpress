use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut scores = HashMap::new();
    let mut file = File::open("scores.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    for line in contents.lines() {
        let score: i32 = line.parse().unwrap();
        scores.insert(score, ());
    }
}
