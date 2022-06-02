#[macro_use]
extern crate lalrpop_util;
extern crate colored;
extern crate exitcode;
extern crate rustyline;

use colored::*;
use core::f64::consts::E;
use core::f64::consts::PI;
use std::collections::HashMap;
use std::process;
use rustyline::Editor;
use rustyline::error::ReadlineError;
use std::env;
use std::fs;

lalrpop_mod!(pub parser);

fn main() {
    let mut symtab = HashMap::new();

    // Insert some default variables
    symtab.insert(String::from("pi"), PI);
    symtab.insert(String::from("e"), E);
    symtab.insert(String::from("ans"), 0.0_f64); // ans variable. updated after very succseful operation

    println!("{}", "[type HELP for more info]".blue().underline());
    
    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new();
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline(">-> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                
                (|| { //Match input
                    match line.trim(){
                        "quit" | "exit" => process::exit(exitcode::OK),
                        
                        "history" => {
                            
                            // for (key, value) in &hist {
                            //     println!("{}: {}", key, value.to_string().bold());
                            // }
                            
                        },
                        
                        "" => return,
                        
                        _ => {
                            match parser::StatementParser::new().parse(&mut symtab, line.trim()) {
                                Ok(v) => {
                                    let v_10 = (v * 10000000000.0).round() / 10000000000.0; // 10 digits of decimal precision
                                    println!("{}", v_10);
                                    *symtab.get_mut("ans").unwrap() = v_10;
                                    // hist.insert(String::from(line.trim()),v_10);
                                }
                                Err(e) => println!("Error : {}", e),
                            };
                        }
                    }
                })();
                
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C: Quitting");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D: Quitting");
                break
            },
            Err(err) => {
                println!("Error : {:?}", err);
                break
            }
        }
    }
    rl.save_history("history.txt").unwrap();
}