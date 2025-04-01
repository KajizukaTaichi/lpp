use lambda::Lambda;
use lexer::tokenize;
use std::fmt::{self, Display, Formatter};

mod lambda;
mod lexer;
mod lpp;

fn main() {
    println!("Hello, world!");
    let expr = lpp::Expr::parse("1 + 2")
        .unwrap()
        .compile()
        .unwrap()
        .eval()
        .unwrap()
        .expand()
        .unwrap();
    println!("{expr}");
}
