use std::{error, fmt, iter::Peekable};

use lexer::{Lexer, LexerError};

pub mod lexer;
pub mod lexer_plugins;


#[derive(Debug)]
pub enum LanguageLevel {
    Declaration,
    Statement,
    Expression,
}
pub trait TokenGroup: fmt::Debug {
    fn line(&self) -> u32;
    fn lexum(&self) -> &str;
    fn lang_level(&self) -> LanguageLevel;
    fn expr_handler<'a>(&'a self) -> Option<&'a dyn ExperParser> {
        None
    }
}
pub type ParseScanner<'src> = Peekable<Lexer<'src>>;
pub type ExprHandler = Box<dyn ExprPlugin>;

pub trait ExperParser: TokenGroup {
    fn parse_expr<'src>(&self,  scanner: &mut ParseScanner<'src>) -> Box<(dyn TreeNode + 'static)>;
}

pub trait ExprPlugin: 'static + fmt::Debug {
    fn evaluate(&self) -> Option<f32> {
        None
    }
}


pub trait TreeNode: ExprPlugin + fmt::Debug + 'static { }

#[repr(transparent)]
pub struct AstNode(Box<dyn TreeNode>);
impl AstNode {
    pub fn evaluate(&self) -> Option<f32> {
        self.0.evaluate()
    }
}
impl From<Box<(dyn TreeNode + 'static)>> for AstNode {
    fn from(value: Box<dyn TreeNode>) -> Self {
        Self(value)
    }
}

pub struct Evaluator<'src> {
    scanner: ParseScanner<'src>,
    exprs: Vec<AstNode>,
}
impl<'src> Evaluator<'src> {
    pub fn new(scanner: ParseScanner<'src>) -> Self {
        Evaluator { scanner, exprs: vec![] }
    }
    pub fn parse(&mut self) -> Result<(), LexerError> {
        loop {
            let Some(token) = self.scanner.next() else {
                break;
            };
            let token = token?;
            let expr = token.expr_handler().expect("token must be a expression").parse_expr(&mut self.scanner);
            self.exprs.push(AstNode(expr));
        }
        Ok(())
    }
    pub fn eval(&mut self) {
        for expr in self.exprs.drain(..) {
            if let Some(expr) = expr.evaluate() {
                println!("{expr}");
            }
        }
    }
}
