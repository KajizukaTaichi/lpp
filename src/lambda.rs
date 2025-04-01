use std::fmt::{self, Display, Formatter};

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

    pub fn parse(source: &str) {}
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Expr::Variable(var) => write!(f, "{var}"),
            Expr::LambdaAbstract { bind, body } => write!(f, "(λ{bind}. {body})"),
            Expr::Apply { func, arg } => write!(f, "({func} {arg})"),
        }
    }
}

fn tokenize(input: &str) -> Option<Vec<String>> {
    let mut tokens: Vec<String> = Vec::new();
    let mut current = String::new();
    let mut nest: usize = 0;

    for c in input.chars() {
        match c {
            '(' => {
                current.push(c);
                nest += 1;
            }
            ')' => {
                current.push(c);
                if nest != 0 {
                    nest -= 1;
                } else {
                    return None;
                }
            }
            ' ' => {
                if nest == 0 {
                    tokens.push(current.clone());
                    current.clear();
                } else {
                    current.push(c)
                }
            }
            _ => current.push(c),
        }
    }

    // Syntax error check
    if nest != 0 {
        return None;
    }
    if !current.is_empty() {
        tokens.push(current.clone());
    }
    Some(tokens)
}
