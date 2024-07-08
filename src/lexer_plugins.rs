use std::{fmt,error};

use crate::{LexerError, LexerPlugin, TokenGroup};
pub struct ArithmaticParser;
impl<'src> LexerPlugin<'src> for ArithmaticParser {
    fn is_handler(&self, c: char) -> bool {
        match c {
            '0'..='9'
            | '+' 
            | '-' 
            | '*' 
            | '/'=> true,
            _ => false,
        }
    }
    fn handel_lexum(&self, source: &'src str, (start, c): (usize, char),line: u32, pos: &mut crate::LexerPos<'src>) -> Result<Box<dyn TokenGroup<'src>>, crate::LexerError> {
        match c {
            '0'..='9' => {
                let mut last_pos = None;
                let mut seen_dot = false;
                while pos.peek().is_some_and(|(_,c)| match c {
                    '0'..='9' => true,
                    '.' if !seen_dot => {
                        seen_dot = true;
                        true
                    },
                    _ => false,
                }) {
                    last_pos = pos.next();
                }

                match last_pos {
                    Some((pos, '.')) => Err(LexerError::IncompleteToken(Box::new(InvalidNumberError(line, source[start..=pos].to_string())))),
                    Some((pos, _)) => Ok(Box::new(ArithmaticToken::Number { line, lexum: &source[start..=pos] })),
                    None => Ok(Box::new(ArithmaticToken::Number { line, lexum: &source[start..=start] }))
                }

            },
            '+' => Ok(Box::new(ArithmaticToken::Plus { line, lexum: &source[start..=start] })),
            '-' => Ok(Box::new(ArithmaticToken::Minus  { line, lexum: &source[start..=start] })),
            '*' => Ok(Box::new(ArithmaticToken::Star { line, lexum: &source[start..=start] })),
            '/' => Ok(Box::new(ArithmaticToken::Slash { line, lexum: &source[start..=start] })),
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
pub enum ArithmaticToken<'src> {
    Number{
        line: u32,
        lexum: &'src str,
    },
    Plus{
        line: u32,
        lexum: &'src str,
    },
    Minus {
        line: u32,
        lexum: &'src str,
    },
    Star {
        line: u32,
        lexum: &'src str,
    },
    Slash {
        line: u32,
        lexum: &'src str,
    },
}

impl<'src> TokenGroup<'src> for ArithmaticToken<'src> {
    fn line(&self) -> u32 {
        match self {
            ArithmaticToken::Number { line, .. }
           | ArithmaticToken::Plus { line, .. } 
           | ArithmaticToken::Minus { line, .. } 
           | ArithmaticToken::Star { line, .. } 
           | ArithmaticToken::Slash { line, .. } => *line,

        }
    }
    fn lexum(&self) -> &'src str {
        match self {
            ArithmaticToken::Number{ lexum, .. }
           | ArithmaticToken::Plus { lexum, .. } 
           | ArithmaticToken::Minus { lexum, .. }
           | ArithmaticToken::Star { lexum, .. } 
           | ArithmaticToken::Slash { lexum, .. } => *lexum,
        }
    }
}
