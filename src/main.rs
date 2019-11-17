
mod syntax;

use syntax::lexer::Lexer;

fn main() {
    let lexer = Lexer::new("fn main() {}");
    lexer.lex();
}
