
mod syntax;

fn main() {
    syntax::lexer::lex("fn main() {}");
}
