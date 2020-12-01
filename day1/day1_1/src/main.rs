use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;

fn main() {
    let items = read_file("/home/josh/Downloads/day1_1");

    for i in 0..items.len() - 1 {
        for j in i..items.len() - 1 {
            if items[i] + items[j] == 2020 {
                println!("Answer: {0}", items[i] * items[j]);
                return;
            }
        }
    }
}

fn read_file(filename: impl AsRef<Path>) -> Vec<i32> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    return buf.lines()
        .map(|l| l.unwrap().parse::<i32>().unwrap())
        .collect();
}