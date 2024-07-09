use crate::{
    lexer::{LexerError, LexerPlugin, LexerPos},
    ExperParser, ExprHandler, ExprPlugin, LanguageLevel, TokenGroup, TreeNode,
};
use core::panic;
use std::{any::Any, error, fmt, u32};

pub struct ArithmaticParser;
impl LexerPlugin for ArithmaticParser {
    fn is_handler(&self, c: char) -> bool {
        match c {
            '0'..='9' | '+' | '-' | '*' | '/' => true,
            _ => false,
        }
    }
    fn handel_lexum<'src>(
        &self,
        source: &'src str,
        (start, c): (usize, char),
        line: u32,
        pos: &mut LexerPos<'src>,
    ) -> Result<Box<dyn TokenGroup>, LexerError> {
        match c {
            '0'..='9' => {
                let mut last_pos = None;
                let mut seen_dot = false;
                while pos.peek().is_some_and(|(_, c)| match c {
                    '0'..='9' => true,
                    '.' if !seen_dot => {
                        seen_dot = true;
                        true
                    }
                    _ => false,
                }) {
                    last_pos = pos.next();
                }

                match last_pos {
                    Some((pos, '.')) => Err(LexerError::IncompleteToken(Box::new(
                        InvalidNumberError(line, source[start..=pos].to_string()),
                    ))),
                    Some((pos, _)) => Ok(Box::new(ArithmaticToken::Number {
                        line,
                        lexum: source[start..=pos].to_string(),
                    })),
                    None => Ok(Box::new(ArithmaticToken::Number {
                        line,
                        lexum: source[start..=start].to_string(),
                    })),
                }
            }
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
pub struct InvalidNumberError(u32, String);
impl fmt::Display for InvalidNumberError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[line: {}] Error at {}: Invalid Number", self.0, self.1)
    }
}
impl error::Error for InvalidNumberError {}

#[derive(Debug)]
pub enum ArithmaticToken {
    Number { line: u32, lexum: String },
    Plus { line: u32, lexum: String },
    Minus { line: u32, lexum: String },
    Star { line: u32, lexum: String },
    Slash { line: u32, lexum: String },
}

impl TokenGroup for ArithmaticToken {
    fn line(&self) -> u32 {
        match self {
            ArithmaticToken::Number { line, .. }
            | ArithmaticToken::Plus { line, .. }
            | ArithmaticToken::Minus { line, .. }
            | ArithmaticToken::Star { line, .. }
            | ArithmaticToken::Slash { line, .. } => *line,
        }
    }
    fn lexum(&self) -> &str {
        match self {
            ArithmaticToken::Number { lexum, .. }
            | ArithmaticToken::Plus { lexum, .. }
            | ArithmaticToken::Minus { lexum, .. }
            | ArithmaticToken::Star { lexum, .. }
            | ArithmaticToken::Slash { lexum, .. } => lexum,
        }
    }
    fn lang_level(&self) -> LanguageLevel {
        LanguageLevel::Expression
    }
    fn expr_handler<'a>(&'a self) -> Option<&'a dyn ExperParser> {
        Some(self)
    }
}

impl ExperParser for ArithmaticToken {
    fn parse_expr<'src>(
        &self,
        scanner: &mut crate::ParseScanner<'src>,
        lhs: Option<Box<(dyn ExprPlugin + 'static)>>,
    ) -> Box<(dyn TreeNode + 'static)> {
        match self {
            ArithmaticToken::Number { lexum, .. } => {
                let lhs = Box::new(ArithmaticExpr::Literal(
                    lexum.parse().expect("could not parse numer"),
                ));
                if let Some(next) = scanner.next() {
                    next.expect("scann err")
                        .expr_handler()
                        .expect("expected expression")
                        .parse_expr(scanner, Some(lhs))
                } else {
                    lhs
                }
            }
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
    fn evaluate(&self) -> Option<f32> {
        match self {
            ArithmaticExpr::Literal(v) => Some(*v),
            ArithmaticExpr::Add { a, b } => match (a.evaluate(), b.evaluate()) {
                (Some(a), Some(b)) => a + b,
                (Some(_), None) | (None, Some(_)) | (None, None) => unreachable!(),
            }
            .into(),
            ArithmaticExpr::Sub { a, b } => match (a.evaluate(), b.evaluate()) {
                (Some(a), Some(b)) => a - b,
                (Some(_), None) | (None, Some(_)) | (None, None) => unreachable!(),
            }
            .into(),
            ArithmaticExpr::Mul { a, b } => match (a.evaluate(), b.evaluate()) {
                (Some(a), Some(b)) => a * b,
                (Some(_), None) | (None, Some(_)) | (None, None) => unreachable!(),
            }
            .into(),
            ArithmaticExpr::Div { a, b } => match (a.evaluate(), b.evaluate()) {
                (Some(a), Some(b)) => a / b,
                (Some(_), None) | (None, Some(_)) | (None, None) => unreachable!(),
            }
            .into(),
        }
    }
}
impl TreeNode for ArithmaticExpr {
    fn as_expr<'a>(&'a self) -> Option<&'a dyn ExprPlugin> {
        Some(self)
    }
}
