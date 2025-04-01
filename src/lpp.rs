use crate::*;

#[derive(Clone)]
pub struct Value(pub usize);

impl Value {
    pub fn compile(&self) -> Lambda {
        let mut body = Lambda::Variable(String::from("x"));
        for _ in 0..self.0 {
            body = Lambda::Apply {
                func: Box::new(Lambda::Variable(String::from("f"))),
                arg: Box::new(body),
            }
        }
        Lambda::Abstract {
            bind: String::from("f"),
            body: Box::new(Lambda::Abstract {
                bind: String::from("x"),
                body: Box::new(body),
            }),
        }
    }
}

#[derive(Clone)]
pub enum Expr {
    Literal(Value),
    Add(Box<Expr>, Box<Expr>),
}

impl Expr {
    pub fn parse(source: &str) -> Option<Self> {
        let tokens: Vec<String> = tokenize(source)?;
        if tokens.len() == 1 {
            if let Ok(n) = tokens.last()?.parse::<usize>() {
                Some(Expr::Literal(Value(n)))
            } else {
                None
            }
        } else {
            let token = Expr::parse(tokens.last()?)?;
            let operator = tokens.get(tokens.len().checked_sub(2)?)?;
            let has_lhs = || Expr::parse(&tokens.get(..tokens.len() - 2)?.join(" "));
            Some(match operator.as_str() {
                "+" => Expr::Add(Box::new(has_lhs()?), Box::new(token)),
                _ => return None,
            })
        }
    }

    pub fn compile(&self) -> Option<Lambda> {
        match self {
            Expr::Literal(value) => Some(value.compile()),
            Expr::Add(lhs, rhs) => Some(Lambda::Apply {
                func: Box::new(Lambda::Apply {
                    func: Box::new(Lambda::parse("λm. λn. λf. λx. m f (n f x)")?),
                    arg: Box::new(lhs.compile()?),
                }),
                arg: Box::new(rhs.compile()?),
            }),
        }
    }
}
