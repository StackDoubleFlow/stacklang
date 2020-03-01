mod syntax;

use syntax::lexer::Lexer;
use syntax::parser::Parser;

fn main() {

    let source = "
fn println() {
    let x = 1;
    //bruh
    return 'this is a test on its intellegence'; '/* no way */
}
";

    // First pass
    let mut lexer = Lexer::new(source);
    match lexer.lex() {
        Err(e) => {
            eprintln!("ERROR: {}:{}: {}", lexer.line, lexer.col, e);
        }
        Ok(_) => {
            println!("Tokens: {:#?}", lexer.tokens);
        }
    };

    // Second pass
    let mut parser = Parser::new(lexer.tokens);
    parser.parse();
}
