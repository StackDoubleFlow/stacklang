mod syntax;

use syntax::lexer::Lexer;

fn main() {
    let mut lexer = Lexer::new("'fish' + 'fish' + foo + 101868762148313247621");
    match lexer.lex() {
        Err(e) => {
            eprintln!("{}:{} :: {}", lexer.line, lexer.col, e);
        }
        Ok(_) => {
            println!("Tokens: {:#?}", lexer.tokens);
        }
    };
}
