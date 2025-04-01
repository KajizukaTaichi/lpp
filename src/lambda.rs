#[derive(Clone)]
pub enum Expr {
    Variable(String),
    LambdaAbstract(Function),
    Apply(Box<Expr>, Box<Expr>),
}

#[derive(Clone)]
pub struct Function {
    bind: String,
    expr: Box<Expr>,
}

impl Expr {
    pub fn eval(&self) -> Option<Expr> {
        match self {
            Expr::Apply(lambda, expr) => {
                let Expr::LambdaAbstract(Function { bind, expr: body }) = lambda.eval()? else {
                    return None;
                };
                body.bind(&bind, expr).eval()
            }
            Expr::Variable(_) => None,
            _ => Some(self.clone()),
        }
    }

    pub fn bind(&self, name: &String, value: &Expr) -> Expr {
        match self {
            Expr::Variable(var) if var == name => value.clone(),
            Expr::LambdaAbstract(Function { bind, expr }) if bind != name => {
                Expr::LambdaAbstract(Function {
                    bind: bind.clone(),
                    expr: Box::new(expr.bind(name, value)),
                })
            }
            Expr::Apply(lambda, expr) => Expr::Apply(
                Box::new(lambda.bind(name, value)),
                Box::new(expr.bind(name, value)),
            ),
            _ => self.clone(),
        }
    }
}
