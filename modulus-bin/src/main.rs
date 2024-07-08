use modulus::{lexer_plugins, Lexer, LexerError};
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
        for token in lexer {
            match token {
                Ok(token) => println!("{token:?}"),
                Err(LexerError::InvalidToken(c)) => eprintln!("Invalid Token: {c}"),
                Err(LexerError::IncompleteToken(err)) => eprintln!("Lexer Error: {err}"),
            }
        }
        buff.clear();
    }
}
