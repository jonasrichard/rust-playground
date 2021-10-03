use std::collections::HashMap;
use std::fs::File;
//use std::io::prelude::*;
use std::io::{BufRead, BufReader};

fn main() {
    let f = File::open("/Users/richardjonas/.zsh_history").unwrap();
    let mut reader = BufReader::new(f);
    let mut buffer = String::new();
    let mut previous = String::new();
    let mut lines = HashMap::new();

    while let Ok(size) = reader.read_line(&mut buffer) {
        if size == 0 {
            break;
        }

        match buffer.chars().next() {
            None => (),
            Some(':') => {
                // 1st line
                if let Some(separator) = previous.find(';') {
                    let cmd = previous.split_off(separator);

                    lines.insert(cmd, previous.clone());
                }

                previous.clear();
                previous = buffer.clone()
            }
            _ => {
                // continuation
                previous.push_str(&buffer)
            }
        }

        buffer.clear();
    }

    let mut result = vec![];

    for (cmd, ts) in lines {
        result.push(format!("{}{}", ts, cmd));
    }

    result.sort();

    for r in result {
        print!("{}", r);
    }
}
