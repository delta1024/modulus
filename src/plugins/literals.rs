use std::{error, fmt};

use crate::{lexer::{LexerError, LexerPlugin}, value::Value, LanguageLevel, LitParser, LitPlugin, TokenGroup, TreeNode};

pub struct LiteralParser;
impl LexerPlugin for LiteralParser {
    fn is_handler(&self, c: char) -> bool {
        matches!(c, '0'..='9')
    }
    fn handel_lexum<'src>(
            &self,
            source: &'src str,
        (start, c): (usize, char),
            line: u32,
            pos: &mut crate::lexer::LexerPos<'src>,
        ) -> Result<Box<dyn crate::TokenGroup>, crate::lexer::LexerError> {
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
                    Some((pos, _)) => Ok(Box::new(LiteralToken::Number {
                        line,
                        lexum: source[start..=pos].to_string(),
                    })),
                    None => Ok(Box::new(LiteralToken::Number {
                        line,
                        lexum: source[start..=start].to_string(),
                    })),
                }
            }
            _ => Err(LexerError::IncompleteToken(Box::new(
                        InvalidNumberError(line, source[start..=start].to_string()),
                    ))),

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
pub enum LiteralToken{
    Number{
        line: u32,
        lexum: String,
    }
}
impl TokenGroup for LiteralToken {
    fn lang_level(&self) -> crate::LanguageLevel {
        LanguageLevel::Literal
    }
    fn line(&self) -> u32 {
        match self {
            Self::Number { line, ..} => *line,
        }
    }
    fn lexum(&self) -> &str {
        match self {
            Self::Number { lexum, ..} => lexum,
        }
    }
    fn lit_handler(&self) -> Option<&dyn crate::LitParser> {
        Some(self)
    }
}

impl LitParser for LiteralToken {
    fn parse_lit(&self) -> Box<(dyn crate::ExprPlugin + 'static)> {
        match self {
            LiteralToken::Number { lexum, .. } => {
                Box::new(LiteralNode(Value::Number( lexum.parse().expect("could not parse numer"))))
            }
        }
    }
}

#[derive(Debug)]
pub struct LiteralNode(Value);
impl TreeNode for LiteralNode {
    fn as_lit(&self) -> Option<&dyn crate::LitPlugin> {
        Some(self)
    }
    fn as_expr(&self) -> Option<&dyn crate::ExprPlugin> {
        Some(self)
    }
}
impl LitPlugin for LiteralNode {
    fn value(&self) -> Value {
        self.0
    }
}
