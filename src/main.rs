mod syntax;

use syntax::lexer::Lexer;

fn main() {
    let mut lexer = Lexer::new("fn main() {''}");
    lexer.lex().unwrap();
    println!("Tokens: {:#?}", lexer.tokens);
}
