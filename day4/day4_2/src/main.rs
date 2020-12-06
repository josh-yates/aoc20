use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

fn main() {
    let nine_dig_regex: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
    let hex_regex: Regex = Regex::new(r"^#(?:[0-9a-fA-F]{3}){1,2}$").unwrap();

    let eye_cols: Vec<&str> = vec![
        "amb",
        "blu",
        "brn",
        "gry",
        "grn",
        "hzl",
        "oth"
    ];

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
                    let keys_values = kvp.split(':').collect::<Vec<&str>>();
                    let key = keys_values[0];
                    let value = keys_values[1];

                    if key == "cid" || !validate_kvp(key, value, &hex_regex, &nine_dig_regex, &eye_cols) {
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

fn validate_kvp(key: &str, value: &str, hex_regex: &Regex, nine_dig_regex: &Regex, eye_cols: &Vec<&str>) -> bool {
    match &key[..] {
        "byr" => {
            let val = value.parse::<i32>().unwrap_or(0);
            return val >= 1920 && val <= 2002;
        },
        "iyr" => {
            let val = value.parse::<i32>().unwrap_or(0);
            return val >= 2010 && val <= 2020;
        },
        "eyr" => {
            let val = value.parse::<i32>().unwrap_or(0);
            return val >= 2020 && val <= 2030;
        },
        "hgt" => {
            if value.ends_with("in") {
                let mut temp_str = value.to_string();
                temp_str.truncate(temp_str.len() - 2);

                let val = temp_str.parse::<i32>().unwrap();

                return val >= 59 && val <= 76;
            } else if value.ends_with("cm") {
                let mut temp_str = value.to_string();
                temp_str.truncate(temp_str.len() - 2);

                let val = temp_str.parse::<i32>().unwrap();

                return val >= 150 && val <= 193;
            } else {
                return false;
            }
        },
        "hcl" => hex_regex.is_match(&value),
        "ecl" => eye_cols.iter().any(|&i| &i == &value),
        "pid" => {
            return nine_dig_regex.is_match(&value);
        },
        _ => false
    }
}