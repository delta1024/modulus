use crate::{
    EvalError,
    lexer::{LexerError, LexerPlugin, LexerPos}, value::Value, ExperParser, ExprHandler, ExprPlugin, LanguageLevel, TokenGroup, TreeNode
};
use std::{error, fmt};

pub struct ArithmaticParser;
impl LexerPlugin for ArithmaticParser {
    fn is_handler(&self, c: char) -> bool {
        matches!(c,  '+' | '-' | '*' | '/')
    }
    fn handel_lexum<'src>(
        &self,
        source: &'src str,
        (start, c): (usize, char),
        line: u32,
        pos: &mut LexerPos<'src>,
    ) -> Result<Box<dyn TokenGroup>, LexerError> {
        match c {

            '+' => Ok(Box::new(ArithmaticToken::Plus {
                line,
                lexum: source[start..=start].to_string(),
            })),
            '-' => Ok(Box::new(ArithmaticToken::Minus {
                line,
                lexum: source[start..=start].to_string(),
            })),
            '*' => Ok(Box::new(ArithmaticToken::Star {
                line,
                lexum: source[start..=start].to_string(),
            })),
            '/' => Ok(Box::new(ArithmaticToken::Slash {
                line,
                lexum: source[start..=start].to_string(),
            })),
            _ => unreachable!(),
        }
    }
}


#[derive(Debug)]
pub enum ArithmaticToken {
    Plus { line: u32, lexum: String },
    Minus { line: u32, lexum: String },
    Star { line: u32, lexum: String },
    Slash { line: u32, lexum: String },
}

impl TokenGroup for ArithmaticToken {
    fn line(&self) -> u32 {
        match self {
             ArithmaticToken::Plus { line, .. }
            | ArithmaticToken::Minus { line, .. }
            | ArithmaticToken::Star { line, .. }
            | ArithmaticToken::Slash { line, .. } => *line,
        }
    }
    fn lexum(&self) -> &str {
        match self {
             ArithmaticToken::Plus { lexum, .. }
            | ArithmaticToken::Minus { lexum, .. }
            | ArithmaticToken::Star { lexum, .. }
            | ArithmaticToken::Slash { lexum, .. } => lexum,
        }
    }
    fn lang_level(&self) -> LanguageLevel {
        LanguageLevel::Expression
    }
    fn expr_handler(&self) -> Option<&dyn ExperParser> {
        Some(self)
    }
}

impl ExperParser for ArithmaticToken {
    fn parse_expr<'src>(
        &self,
        scanner: &mut crate::ParseScanner<'src>,
        lhs: Option<Box<(dyn ExprPlugin + 'static)>>,
    ) -> Box<(dyn ExprPlugin + 'static)> {
        match self {
            ArithmaticToken::Plus { .. } => {
                let next = scanner
                    .next()
                    .expect("need number after '+'")
                    .expect("scann error");
                let b = next.lexum().parse::<f32>().expect("num parse err");
                Box::new(ArithmaticExpr::Add {
                    a: lhs.expect("need a lhs assignment"),
                    b: Box::new(ArithmaticExpr::Literal(b)),
                })
            }
            ArithmaticToken::Minus { .. } => {
                let next = scanner
                    .next()
                    .expect("need number after '+'")
                    .expect("scann error");
                let b = next.lexum().parse::<f32>().expect("num parse err");
                Box::new(ArithmaticExpr::Sub {
                    a: lhs.expect("need a lhs assignment"),
                    b: Box::new(ArithmaticExpr::Literal(b)),
                })
            }
            ArithmaticToken::Star { .. } => {
                let next = scanner
                    .next()
                    .expect("need number after '+'")
                    .expect("scann error");
                let b = next.lexum().parse::<f32>().expect("num parse err");
                Box::new(ArithmaticExpr::Mul {
                    a: lhs.expect("need a lhs assignment"),
                    b: Box::new(ArithmaticExpr::Literal(b)),
                })
            }
            ArithmaticToken::Slash { .. } => {
                let next = scanner
                    .next()
                    .expect("need number after '+'")
                    .expect("scann error");
                let b = next.lexum().parse::<f32>().expect("num parse err");
                Box::new(ArithmaticExpr::Div {
                    a: lhs.expect("need a lhs assignment"),
                    b: Box::new(ArithmaticExpr::Literal(b)),
                })
            }
        }
    }
}

#[derive(Debug)]
pub enum ArithmaticExpr {
    Literal(f32),
    Add { a: ExprHandler, b: ExprHandler },
    Sub { a: ExprHandler, b: ExprHandler },
    Mul { a: ExprHandler, b: ExprHandler },
    Div { a: ExprHandler, b: ExprHandler },
}

impl ExprPlugin for ArithmaticExpr {
    fn evaluate(&self) -> Result<Value, EvalError> {
        match self {
            ArithmaticExpr::Literal(v) => Ok(Value::from(*v)),
            ArithmaticExpr::Add { a, b } => match (a.evaluate(), b.evaluate()) {
                (Ok(Value::Number(a)), Ok(Value::Number(b))) => Ok(a + b).map(Value::Number),
                _ => panic!("wrong type"),
            },
            ArithmaticExpr::Sub { a, b } => match (a.evaluate(), b.evaluate()) {
                (Ok(Value::Number(a)), Ok(Value::Number(b))) => Ok(a - b).map(Value::Number),
                _ => panic!("wrong type"),
            },
            ArithmaticExpr::Mul { a, b } => match (a.evaluate(), b.evaluate()) {
                (Ok(Value::Number(a)), Ok(Value::Number(b))) => Ok(a * b).map(Value::Number),
                _ => panic!("wrong type"),
            },
            ArithmaticExpr::Div { a, b } => match (a.evaluate(), b.evaluate()) {
                (Ok(Value::Number(a)), Ok(Value::Number(b))) => Ok(a / b).map(Value::Number),
                _ => panic!("wrong type"),
            },
        }
    }
}
impl TreeNode for ArithmaticExpr {
    fn as_expr(&self) -> Option<&dyn ExprPlugin> {
        Some(self)
    }
}
