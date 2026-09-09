mod btree;
mod error;
mod executor;
mod heap;
mod page;
mod query;
mod row;
mod table;
mod wal;

use std::io::{self, BufRead, Write};
use crate::executor::execute;
use crate::query::{lex, parse};
use crate::table::Table;

fn main() {
    let mut table = Table::create("tinydb.dat").unwrap();
    let stdin = io::stdin();

    println!("tinydb> SQL REPL — type 'exit' to quit");
    println!("Commands: INSERT INTO t (id, name) VALUES (1, \"alice\")");
    println!("          SELECT * FROM t WHERE id = 1");
    loop {
        print!("tinydb> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        match stdin.lock().read_line(&mut line) {
            Ok(0) => break, // EOF
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                break;
            }
        }

        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if line == "exit" || line == "quit" {
            break;
        }

        let tokens = lex(line);
        match parse(&tokens) {
            Ok(stmt) => match execute(stmt, &mut table) {
                Ok(msg) => println!("{}", msg),
                Err(e) => println!("Error: {}", e),
            },
            Err(e) => println!("Parse error: {}", e),
        }
    }
}
