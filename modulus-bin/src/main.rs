use modulus::{lexer_plugins, Evaluator};
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
        let mut evaluator = Evaluator::builder()
            .plugin(lexer_plugins::ArithmaticParser)
            .source(&buff)
            .build();
        if let Err(err) = evaluator.parse() {
            eprintln!("{err}");
        }
        evaluator.eval();
        buff.clear();
    }
}
