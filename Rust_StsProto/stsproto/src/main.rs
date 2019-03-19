use std::{
    fs,
    io
};


struct Token {
    token_type : TokenType,
    token_value : String
}

impl Token {
    fn new(ttype : TokenType, tval : String) -> Token {
        Token { token_type: ttype, token_value: tval }
    }
}


#[derive(Copy, Clone, PartialEq)]
enum TokenType {
    Unknown,
    Str,
    ParOpen,
    ParClose,
    Semicolon,
    Whitespace,
}

impl TokenType {
    fn determine(ch : char) -> TokenType {
        match ch {
            'a'...'z' | 'A'...'Z' | '0'...'9' => TokenType::Str,
            '{' => TokenType::ParOpen,
            '}' => TokenType::ParClose,
            ';' => TokenType::Semicolon,
            ' ' | '\t' | '\r' | '\n' => TokenType::Whitespace,
            _ => panic!("cannot determine")
        }
    }
}



struct Tokenizer {
    tokens : Vec<Token>,
    current_token : String,
    state : TokenType
}


impl Tokenizer {
    fn new() -> Tokenizer {
        Tokenizer {
            tokens : vec![],
            current_token : String::new(),
            state : TokenType::Unknown
        }
    }

    fn parse(&mut self, protocol : String) -> Result<(), io::Error> {
        for ch in protocol.chars() {
            self.parse_token(self.state, ch)
        }

        Ok(())
    }


    fn parse_token(&mut self, prev_state : TokenType, ch : char) {
        let current_state = TokenType::determine(ch);

        if prev_state != TokenType::Unknown && prev_state != current_state {
            if prev_state != TokenType::Unknown {
                self.tokens.push(Token::new(prev_state.clone(), self.current_token.clone()));
                self.current_token.clear();
            }
        }

        self.current_token.push(ch);
        self.state = current_state.clone();
    }
}


fn main() {
    let mut tokenizer = Tokenizer::new();

    match fs::read_to_string("input.spr") {
        Err(e) => panic!("{:?}", e),
        Ok(s) => tokenizer.parse(s).unwrap()
    }
}
