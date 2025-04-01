use lambda::Lambda;
use lexer::tokenize;
use std::env::args;
use std::fmt::{self, Display, Formatter};
use std::process::Command;

mod lambda;
mod lexer;
mod lpp;

const CMD: &str = "python3";

fn main() {
    let expr = args().collect::<Vec<_>>()[1..].join(" ");
    let expr = lpp::Expr::parse(&expr).unwrap().compile().unwrap();
    println!("Lambda Calculus Formula: {expr}");

    let pycode = format!("print({}(lambda x: x + 1)(0))", expr.compile());
    let output = Command::new(CMD).args(["-c", &pycode]).output().unwrap();
    let output = String::from_utf8_lossy(&output.stdout).trim().to_string();
    println!("Evaluated Result: {output}");
}
