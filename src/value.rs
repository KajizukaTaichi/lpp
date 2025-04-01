use crate::lambda::Expr;

pub struct Value(pub usize);

impl Value {
    pub fn compile(&self) -> Expr {
        let mut body = Expr::Variable(String::from("x"));
        for _ in 0..self.0 {
            body = Expr::Apply {
                func: Box::new(Expr::Variable(String::from("f"))),
                arg: Box::new(body),
            }
        }
        Expr::LambdaAbstract {
            bind: String::from("f"),
            body: Box::new(Expr::LambdaAbstract {
                bind: String::from("x"),
                body: Box::new(body),
            }),
        }
    }
}
