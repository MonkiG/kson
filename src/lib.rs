mod lex;

pub struct Parser {}

impl Parser {
    pub fn new() -> Self {
        Parser {}
    }
}

pub struct Kson {}

impl Kson {
    pub fn parse(source: &str) {
        let lex = lex::Lexer::new(source);
        let parser = Parser::new();
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(result, 4);
    }
}
