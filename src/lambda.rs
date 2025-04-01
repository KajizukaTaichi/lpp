use crate::*;

#[derive(Debug, Clone)]
pub enum Lambda {
    Variable(String),
    Abstract { bind: String, body: Box<Lambda> },
    Apply { func: Box<Lambda>, arg: Box<Lambda> },
}

impl Lambda {
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

    pub fn compile_to_python(&self) -> String {
        match self {
            Lambda::Variable(var) => format!("{var}"),
            Lambda::Abstract { bind, body } => {
                format!("(lambda {bind}: {})", body.compile_to_python())
            }
            Lambda::Apply { func, arg } => {
                format!("{}({})", func.compile_to_python(), arg.compile_to_python())
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
