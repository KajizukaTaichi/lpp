use lambda::Expr;
use value::Value;

mod lambda;
mod value;

fn main() {
    println!("Hello, world!");
    println!("{}", Value(3).compile());
}
