use modulus::{lexer::Lexer, lexer_plugins, Evaluator};
macro_rules! print_flush {
    ($($var:tt)*) => {
        use std::io::Write;
        print!($($var)*);
        std::io::stdout().flush().expect("could not flush stdout");
    };
}
fn main() {
    let mut buff = String::new();
    let stdin = std::io::stdin();
    loop {
        print_flush!("> ");
        if stdin.read_line(&mut buff).unwrap() == 0 {
            break;
        }
        let lexer = Lexer::builder(&buff)
            .add_handler(lexer_plugins::ArithmaticParser)
            .build();
        let mut evaluator = Evaluator::new(lexer.peekable());
        if let Err(err) = evaluator.parse() {
            eprintln!("{err}");
        }
        evaluator.eval();
        buff.clear();
    }
}
