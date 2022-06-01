#[macro_use]
extern crate lalrpop_util;
extern crate colored;

use colored::*;
use std::collections::HashMap;
use std::io::BufRead;
use std::io::Write;
use core::f64::consts::PI;
use core::f64::consts::E;

lalrpop_mod!(pub parser);

fn main() {
    let mut symtab = HashMap::new();

    // Insert some default variables. Change these later
    symtab.insert(String::from("pi"),PI);
    symtab.insert(String::from("e"),E);
    symtab.insert(String::from("ans"),0.0_f64); // ans variable. updated after very succseful operation

    println!("{}","rcalc FOSS [HELP for more info]".blue().underline());
    print!("{} ",">>>>>".blue().bold());
    std::io::stdout().flush().unwrap();
    for line in std::io::stdin().lock().lines() {
        let line = line.expect("Input Error");
        match parser::StatementParser::new().parse(&mut symtab, line.trim()) {
            Ok(v) => {
                println!("{}", (v * 10000000000.0).round() / 10000000000.0);
                *symtab.get_mut("ans").unwrap() = v;
            },
            Err(e) => println!("Error : {}", e),
        }
        print!("{} ",">>>>>".blue().bold());
        std::io::stdout().flush().unwrap();
    }
}