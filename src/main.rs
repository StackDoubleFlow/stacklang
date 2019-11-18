mod syntax;

use syntax::lexer::Lexer;

fn main() {
    let mut lexer = Lexer::new("'fish' + 'fish' + foo + 10121");
    lexer.lex().unwrap();
    println!("Tokens: {:#?}", lexer.tokens);
}
