use crate::{
    lexer::{Lexer, LexerError, LexerHandler, LexerPlugin},
    AstNode, ParseScanner,
};

pub struct Evaluator<'src> {
    scanner: ParseScanner<'src>,
    exprs: Vec<AstNode>,
}
impl<'src> Evaluator<'src> {
    pub fn builder() -> EvalBuilder<'src> {
        EvalBuilder {
            source: None,
            plugins: vec![],
        }
    }
    pub fn parse(&mut self) -> Result<(), LexerError> {
        loop {
            let Some(token) = self.scanner.next() else {
                break;
            };
            let token = token?;
            if token.lit_handler().is_some() {
                let expr = token.lit_handler().unwrap().parse_lit();
            self.exprs.push(AstNode(expr));
            } else {
                let lhs = self.exprs.pop().expect("expected binary expression");
            let expr = token
                .expr_handler()
                .expect("token must be a expression")
                .parse_expr(&mut self.scanner, Some(lhs.0));
                self.exprs.push(AstNode(expr));

            }
        }
        Ok(())
    }
    pub fn eval(&mut self) {
        for expr in self.exprs.drain(..) {
            let _ = Box::new(5);
            if let Some(expr) = expr.as_expr().map(|e| e.evaluate().ok()).flatten() {
                println!("{expr}");
            }
        }
    }
}
pub struct EvalBuilder<'src> {
    source: Option<&'src str>,
    plugins: Vec<LexerHandler>,
}
impl<'src> EvalBuilder<'src> {
    pub fn source(&mut self, src: &'src str) -> &mut Self {
        self.source = Some(src);
        self
    }
    pub fn plugin(&mut self, plugin: impl LexerPlugin) -> &mut Self {
        self.plugins.push(Box::new(plugin));
        self
    }
    pub fn build(&mut self) -> Evaluator<'src> {
        let mut scanner = Lexer::builder(self.source.expect("source field must be set"));
        for plugin in self.plugins.drain(..) {
            scanner.add_handler(plugin);
        }
        Evaluator {
            scanner: scanner.build().peekable(),
            exprs: vec![],
        }
    }
}
