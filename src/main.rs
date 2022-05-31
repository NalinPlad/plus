#[macro_use]
extern crate lalrpop_util;
extern crate colored;

use colored::*;
use std::collections::HashMap;
use std::io::BufRead;
use std::io::Write;

lalrpop_mod!(pub parser);

fn main() {

    let mut symtab = HashMap::new();
    print!("{} ",">>>>>".blue().bold());
    std::io::stdout().flush().unwrap();
    for line in std::io::stdin().lock().lines() {
        let line = line.expect("Input Error");
        match parser::StatementParser::new().parse(&mut symtab, line.trim()) {
            Ok(v) => println!("{}", v),
            Err(e) => println!("Error : {}", e),
        }
        print!("{} ",">>>>>".blue().bold());
        std::io::stdout().flush().unwrap();
    }
}