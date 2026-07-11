mod error;
mod page;
mod row;

use page::Page;
use std::io::{self, Write};
fn main() {
    let mut p = Page::new();
    loop {
        print!("tinydb> ");
        io::stdout().flush().unwrap();
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        let parts: Vec<&str> = buf.trim().split_whitespace().collect();
        match parts.first() {
            Some(&".insert") => {
                let input = buf.trim();
                if let Some(pos) = input.find(' ') {
                    let data = input[pos+1..].trim();
                    p.insert_tuple(data.as_bytes());
                    println!("Query Succes")
                }
            }

            Some(&".get") => {
                if let Some(slot_str) = parts.get(1) {
                    if let Ok(slot) = slot_str.parse::<u16>() {
                        match p.get_tuple(slot) {
                            Some(data) => println!("{}", String::from_utf8_lossy(data)),
                            None => println!("not found or deleted"),
                        }
                    }
                }
            }
            Some(&".free") => {
                println!("free space: {}", p.free_space());
            }
            Some(&".exit") => break,
            _ => println!(".insert <text> | .get <slot> | .free | .exit"),
        }
    }
}
