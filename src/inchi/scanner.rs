#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token<'a> {
    Alpha(&'a str),
    Numeric(usize),
    Hyphen,
    Comma,
    LParen,
    RParen,
}

pub struct Scanner<'a> {
    input: &'a str,
}

impl<'a> Scanner<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input }
    }

    pub fn lookahead_numeric(&self) -> bool {
        self.input.starts_with(char::is_numeric)
    }

    pub fn lookahead_hyphen(&self) -> bool {
        self.input.starts_with('-')
    }

    pub fn lookahead_comma(&self) -> bool {
        self.input.starts_with(',')
    }

    pub fn expect(&mut self, expected: Token) {
        match self.next() {
            Some(token) if token == expected => {}
            Some(token) => panic!("Expected token: {expected:?}, got: {token:?}"),
            None => panic!("Expected token: {expected:?}, got nothing"),
        }
    }

    pub fn expect_numeric(&mut self) -> usize {
        match self.next() {
            Some(Token::Numeric(n)) => n,
            _ => panic!("Expected numeric token"),
        }
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.input.is_empty() {
            return None;
        }

        let c = self.input.chars().next().unwrap();
        let token;
        match c {
            _ if c.is_alphabetic() => {
                let len = self
                    .input
                    .find(|c: char| !c.is_alphabetic())
                    .unwrap_or(self.input.len());
                token = Token::Alpha(&self.input[..len]);
                self.input = &self.input[len..];
            }
            _ if c.is_numeric() => {
                let len = self
                    .input
                    .find(|c: char| !c.is_numeric())
                    .unwrap_or(self.input.len());
                token = Token::Numeric(self.input[..len].parse().unwrap());
                self.input = &self.input[len..];
            }
            '-' => {
                token = Token::Hyphen;
                self.input = &self.input[1..];
            }
            ',' => {
                token = Token::Comma;
                self.input = &self.input[1..];
            }
            '(' => {
                token = Token::LParen;
                self.input = &self.input[1..];
            }
            ')' => {
                token = Token::RParen;
                self.input = &self.input[1..];
            }
            _ => panic!("Unexpected character in InChI: {c:?}"),
        }

        Some(token)
    }
}
