mod syntax;

use syntax::lexer::Lexer;

fn main() {
    let mut lexer = Lexer::new("
    fn println() {
        let x = 1;
        return 'this is a test on its intellegence';
    }
");
    match lexer.lex() {
        Err(e) => {
            eprintln!("ERROR: {}:{}: {}", lexer.line, lexer.col, e);
        }
        Ok(_) => {
            println!("Tokens: {:#?}", lexer.tokens);
        }
    };
}
