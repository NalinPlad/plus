// plus [+]

#[macro_use]
extern crate lalrpop_util;
extern crate rustyline;


use core::f64::consts::E;
use core::f64::consts::PI;
use std::collections::HashMap;
use std::process;
use rustyline::Editor;
use rustyline::error::ReadlineError;

lalrpop_mod!(pub parser);

pub fn root(x: f64, n: u32) -> f64 {
    let exp = 1.0 / n as f64;
    if (n & 1) == 0 {
        x.powf(exp)
    } else {
        let absroot = x.abs().powf(exp);
        if x < 0.0 {
            -absroot
        } else {
            absroot
        }
    }
}

// Main loop

fn main() {
    let history_path = "rc.hist.txt";
    let mut symtab = HashMap::new();
    let mut s_hist = HashMap::new();

    // Insert some default variables
    symtab.insert(String::from("pi"), PI);
    symtab.insert(String::from("e"), E);
    symtab.insert(String::from("ans"), 0.0_f64); // ans variable. updated after very succseful operation

    // println!("{}", "[type HELP for more info]".blue().underline());
    
    let mut rl = Editor::<()>::new();
    if rl.load_history(history_path).is_err() {
        println!("No previous history.");
    }

    loop {
        let readline = rl.readline("plus> ");
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
                                println!{"session history [ctrl-r => search  all history]"}
                                for (key, value) in &s_hist {
                                    println!("{}: {}", key, value);
                                }
                            }
                        },
                        
                        "" => return,
                        
                        _ => {
                            match parser::StatementParser::new().parse(&mut symtab, line.trim()) {
                                Ok(v) => {
                                    let mut v_10 = (v * 10000000000.0).round() / 10000000000.0; // 10 digits of decimal precision
                                    if v_10 == -0.0{
                                        v_10 = 0.0
                                    }
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


// Tests

#[cfg(test)]
mod tests {
    
    extern crate lalrpop_util;
    extern crate rustyline;


    use core::f64::consts::E;
    use core::f64::consts::PI;
    use std::collections::HashMap;

    lalrpop_mod!(pub parser);

    //Modified parse fn for tests
    pub fn parse_eq(line: String) -> f64 {
        let mut symtab_test = HashMap::new();

        symtab_test.insert(String::from("pi"), PI);
        symtab_test.insert(String::from("e"), E);
        symtab_test.insert(String::from("ans"), 0.0_f64);

        match parser::StatementParser::new().parse(&mut symtab_test, &line) {
            Ok(v) => {
                let mut v_10 = (v * 10000000000.0).round() / 10000000000.0; // 10 digits of decimal precision
                if v_10 == -0.0{
                    v_10 = 0.0
                }
                return v_10;
            }
            Err(e) => panic!("{}", e),
        };
    }
    #[test]
    fn integer_addition(){
        let result: f64 = parse_eq(String::from("5+3"));
        assert_eq!(result, 8.0);
    }

    #[test]
    fn float_addition(){
        let result: f64 = parse_eq(String::from("0.1+0.2"));
        assert_eq!(result, 0.3);
    }

    #[test]
    fn grouping(){
        let result: f64 = parse_eq(String::from("3(5+3)"));
        assert_eq!(result, 24.0);
    }

    #[test]
    fn builtin_addition(){
        let result: f64 = parse_eq(String::from("5+sin(50)"));
        assert_eq!(result, 4.7376251463);
    }
}