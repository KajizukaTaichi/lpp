use lambda::Lambda;
use lexer::tokenize;
use std::fmt::{self, Display, Formatter};

mod lambda;
mod lexer;
mod lpp;

fn main() {
    println!("Hello, world!");
    let a = lpp::Expr::parse("1 + 2");
    println!(
        "{}",

            .unwrap()
            .compile()
            .unwrap()
            .eval()
            .unwrap()
            .eval()
            .unwrap()
    );
}
