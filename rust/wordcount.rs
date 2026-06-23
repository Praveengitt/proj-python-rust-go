use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("words.txt")
        .expect("Failed to open file");

    let reader = BufReader::new(file);

    let mut word_count: HashMap<String, u32> = HashMap::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");

        for word in line.split_whitespace() {
            let word = word.to_lowercase();

            *word_count.entry(word).or_insert(0) += 1;
        }
    }

    println!("{:?}", word_count);
}