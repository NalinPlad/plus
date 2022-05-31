#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(pub parser); //defines parser mod
fn main(){
     // Output 4
     match parser::ExprParser::new().parse("2+2") {
            Ok(v) => println!("{}", v),
            Err(e) => println!("Error : {}", e),
        }
}