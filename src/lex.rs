use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    LBracket, // [
    RBracket, // ]
    LBrace,   // {
    RBrace,   // }
    Colon,    // :
    Comma,    // ,

    False,
    True,
    Null,
    Eof,
    Number(f64),
    String(String),

    Invalid,
}

pub struct Lexer<'a> {
    source: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Lexer {
            source: source.chars().peekable(),
        }
    }

    pub fn next_token(&mut self) -> TokenKind {
        self.skip_whitespace();

        match self.source.next() {
            Some('[') => TokenKind::LBracket,
            Some(']') => TokenKind::RBracket,
            Some('{') => TokenKind::LBrace,
            Some('}') => TokenKind::RBrace,
            Some(':') => TokenKind::Colon,
            Some(',') => TokenKind::Comma,

            Some('f') => self.handle_literal_values("alse", TokenKind::False),
            Some('t') => self.handle_literal_values("rue", TokenKind::True),
            Some('n') => self.handle_literal_values("ull", TokenKind::Null),

            Some('"') => todo!("handle string"),
            Some(ch) if ch.is_ascii_digit() => todo!("handle numbers"),

            Some(_) => TokenKind::Invalid,

            None => TokenKind::Eof,
        }
    }

    fn skip_whitespace(&mut self) {
        // Todo: Json spec only defines four whitespaces characters
        // https://www.ietf.org/ietf-ftp/rfc/std/std90.txt
        //
        //   ws = *(
        //      %x20 /              ; Space
        //      %x09 /              ; Horizontal tab
        //     %x0A /               ; Line feed or New line
        //      %x0D )              ; Carriage return
        //
        //
        while let Some(&ch) = self.source.peek() {
            if ch.is_whitespace() {
                self.source.next();
            } else {
                break;
            }
        }
    }

    fn handle_literal_values(&mut self, rest: &str, token: TokenKind) -> TokenKind {
        for expected in rest.chars() {
            if self.source.next() != Some(expected) {
                return TokenKind::Invalid;
            }
        }

        if let Some(&next_ch) = self.source.peek() {
            if next_ch.is_ascii_alphanumeric() {
                return TokenKind::Invalid;
            }
        }

        token
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = TokenKind;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_token() {
            TokenKind::Eof => None,
            token => Some(token),
        }
    }
}
