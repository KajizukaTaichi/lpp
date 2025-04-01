#[derive(Clone)]
pub enum Expr {
    Variable(String),
    LambdaAbstract { bind: String, body: Box<Expr> },
    Apply { func: Box<Expr>, arg: Box<Expr> },
}

impl Expr {
    pub fn eval(&self) -> Option<Expr> {
        match self {
            Expr::Apply { func, arg } => {
                let Expr::LambdaAbstract { bind, body } = func.eval()? else {
                    return None;
                };
                body.bind(&bind, arg).eval()
            }
            Expr::Variable(_) => None,
            _ => Some(self.clone()),
        }
    }

    pub fn bind(&self, name: &String, value: &Expr) -> Expr {
        match self {
            Expr::Variable(var) if var == name => value.clone(),
            Expr::LambdaAbstract { bind, body } if bind != name => Expr::LambdaAbstract {
                bind: bind.clone(),
                body: Box::new(body.bind(name, value)),
            },
            Expr::Apply { func, arg } => Expr::Apply {
                func: Box::new(func.bind(name, value)),
                arg: Box::new(arg.bind(name, value)),
            },
            _ => self.clone(),
        }
    }
}
