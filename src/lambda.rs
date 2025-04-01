use crate::*;

#[derive(Debug, Clone)]
pub enum Lambda {
    Variable(String),
    Abstract { bind: String, body: Box<Lambda> },
    Apply { func: Box<Lambda>, arg: Box<Lambda> },
}

impl Lambda {
    pub fn eval(&self) -> Option<Lambda> {
        match self {
            Lambda::Apply { func, arg } => {
                let Lambda::Abstract { bind, body } = func.eval()? else {
                    return None;
                };
                body.bind(&bind, &arg.eval()?).eval()
            }
            Lambda::Abstract { bind, body } => Some(Lambda::Abstract {
                bind: bind.clone(),
                body: Box::new(body.eval()?),
            }),
            Lambda::Variable(_) => Some(self.clone()),
        }
    }

    pub fn bind(&self, name: &String, value: &Lambda) -> Lambda {
        match self {
            Lambda::Variable(var) if var == name => value.clone(),
            Lambda::Abstract { bind, body } if bind != name => Lambda::Abstract {
                bind: bind.clone(),
                body: Box::new(body.bind(name, value)),
            },
            Lambda::Apply { func, arg } => Lambda::Apply {
                func: Box::new(func.bind(name, value)),
                arg: Box::new(arg.bind(name, value)),
            },
            _ => self.clone(),
        }
    }

    pub fn parse(source: &str) -> Option<Lambda> {
        if let Some(source) = source.strip_prefix("λ") {
            let (bind, body) = source.split_once(".")?;
            Some(Lambda::Abstract {
                bind: bind.trim().to_string(),
                body: Box::new(Lambda::parse(body)?),
            })
        } else {
            let tokens = tokenize(source)?;
            if tokens.len() == 1 {
                let token = tokens.last()?.to_string();
                if token.starts_with("(") && token.ends_with(")") {
                    let token = token.get(1..token.len() - 1)?;
                    Lambda::parse(token)
                } else if token.chars().count() == 1 {
                    Some(Lambda::Variable(token))
                } else {
                    None
                }
            } else if tokens.len() >= 2 {
                Some(Lambda::Apply {
                    func: Box::new(Lambda::parse(&tokens.get(..tokens.len() - 1)?.join(" "))?),
                    arg: Box::new(Lambda::parse(tokens.last()?)?),
                })
            } else {
                None
            }
        }
    }
}

impl Display for Lambda {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Lambda::Variable(var) => write!(f, "{var}"),
            Lambda::Abstract { bind, body } => write!(f, "(λ{bind}. {body})"),
            Lambda::Apply { func, arg } => write!(f, "({func} {arg})"),
        }
    }
}
