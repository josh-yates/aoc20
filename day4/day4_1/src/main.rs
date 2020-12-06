use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {

    let mut expected_keys = vec![
        "byr".to_string(),
        "iyr".to_string(),
        "eyr".to_string(),
        "hgt".to_string(),
        "hcl".to_string(),
        "ecl".to_string(),
        "pid".to_string()
    ];

    expected_keys.sort();

    if let Ok(lines) = read_lines("/home/josh/Downloads/day4") {

        let mut count = 0;
        let mut cur_buf = Vec::<String>::new();

        for line in lines {
            if let Ok(content) = line {
                if content == "" {
                    cur_buf.sort();

                    if vecs_match(&expected_keys, &cur_buf) {
                        count += 1;
                    }

                    cur_buf.clear();
                    continue;
                }

                for kvp in content.split_whitespace() {
                    let key = kvp.split(':').collect::<Vec<&str>>()[0];

                    if key == "cid" {
                        continue;
                    }

                    cur_buf.push(key.to_string())
                }
            }
        }

        if vecs_match(&expected_keys, &cur_buf) {
            count += 1;
        }

        println!("Answer: {0}", count);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn vecs_match<P>(vec1: &Vec<P>, vec2: &Vec<P>) -> bool
where P: Eq {
    let mut matching = vec1.len() == vec2.len();
    let mut index = 0;

    while matching && index < vec1.len() {
        matching = vec1[index] == vec2[index];
        index += 1;
    }

    return matching;
}