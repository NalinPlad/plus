#[macro_use]
extern crate lalrpop_util;
extern crate colored;
extern crate exitcode;

use colored::*;
use core::f64::consts::E;
use core::f64::consts::PI;
use std::collections::HashMap;
use std::io::BufRead;
use std::io::Write;
use std::process;

lalrpop_mod!(pub parser);

fn main() {
    let mut symtab = HashMap::new();
    let mut hist: HashMap<String, f64> = HashMap::new();

    // Insert some default variables
    symtab.insert(String::from("pi"), PI);
    symtab.insert(String::from("e"), E);
    symtab.insert(String::from("ans"), 0.0_f64); // ans variable. updated after very succseful operation

    println!("{}", "[type HELP for more info]".blue().underline());
    print!("{} ", ">>>>>".blue().bold());
    std::io::stdout().flush().unwrap();
    for line in std::io::stdin().lock().lines() {
        let line = line.expect("Input Error");
        
        (|| { //Match input
            match line.trim(){
                "quit" | "exit" => process::exit(exitcode::OK),
                
                "history" => {
                    println!("----------");
                    for (key, value) in &hist {
                        println!("{}: {}", key, value.to_string().bold());
                    }
                    println!("----------");
                },
                
                "" => return,
                
                _ => {
                    match parser::StatementParser::new().parse(&mut symtab, line.trim()) {
                        Ok(v) => {
                            println!("{}", (v * 10000000000.0).round() / 10000000000.0); // 10 digits of decimal precision
                            *symtab.get_mut("ans").unwrap() = v;
                            hist.insert(String::from(line.trim()),v);
                        }
                        Err(e) => println!("Error : {}", e),
                    };
                }
            }
        })();

        print!("{} ", ">>>>>".blue().bold());
        std::io::stdout().flush().unwrap();
    }
}
