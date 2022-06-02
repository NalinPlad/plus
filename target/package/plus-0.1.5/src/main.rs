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

lalrpop_mod!(pub parser);

fn main() {
    let history_path = "rc.hist.txt";
    let mut symtab = HashMap::new();
    let mut s_hist = HashMap::new();

    // Insert some default variables
    symtab.insert(String::from("pi"), PI);
    symtab.insert(String::from("e"), E);
    symtab.insert(String::from("ans"), 0.0_f64); // ans variable. updated after very succseful operation

    println!("{}", "[type HELP for more info]".blue().underline());
    
    let mut rl = Editor::<()>::new();
    if rl.load_history(history_path).is_err() {
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
                            if s_hist.keys().len() < 1{
                                println!("no session history [ctrl-r => search  all history]")
                            } else {
                                for (key, value) in &s_hist {
                                    println!{"session history [ctrl-r => search  all history]"}
                                    println!("{}: {}", key, value);
                                }
                            }
                        },
                        
                        "" => return,
                        
                        _ => {
                            match parser::StatementParser::new().parse(&mut symtab, line.trim()) {
                                Ok(v) => {
                                    let v_10 = (v * 10000000000.0).round() / 10000000000.0; // 10 digits of decimal precision
                                    println!("{}", v_10);
                                    *symtab.get_mut("ans").unwrap() = v_10;
                                    rl.add_history_entry(line.as_str()); // perm hist
                                    s_hist.insert(String::from(line.trim()),v_10); // session hist
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
    rl.save_history(history_path).unwrap();
}