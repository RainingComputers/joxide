#[derive(Debug, PartialEq)]
pub enum TokenType<'a> {
    Null,
    Bool(bool),
    Number(f64),
    String(&'a str),
    KeyOrVal(&'a str),
    OpenCurly,
    CloseCurly,
    OpenSquare,
    CloseSquare,
    Colon,
    Comma,
}

#[derive(Debug, PartialEq)]
pub struct Token<'a> {
    token_type: TokenType<'a>,
    line: usize,
    col: usize,
}

impl Token<'_> {
    fn from_punctuator(c: char, line: usize, col: usize) -> Token<'static> {
        let token_type = match c {
            '{' => TokenType::OpenCurly,
            '}' => TokenType::CloseCurly,
            '[' => TokenType::OpenSquare,
            ']' => TokenType::CloseSquare,
            ':' => TokenType::Colon,
            ',' => TokenType::Comma,
            _ => panic!("Not a valid punctuator character"),
        };

        Token {
            token_type,
            line,
            col,
        }
    }

    fn from_quoted_str<'a>(string: &'a str, line: usize, col: usize) -> Token {
        let token_string = &string[1..string.len() - 1];

        Token {
            token_type: TokenType::<'a>::String(token_string),
            line,
            col,
        }
    }

    fn from_key_or_val(symbol: &str, line: usize, col: usize) -> Token {
        fn get_token_type(symbol: &str) -> TokenType {
            match symbol.parse::<f64>() {
                Ok(number) => TokenType::Number(number),
                _ => match symbol {
                    "null" => TokenType::Null,
                    "true" => TokenType::Bool(true),
                    "false" => TokenType::Bool(false),
                    _ => TokenType::KeyOrVal(symbol),
                },
            }
        }

        let token_type = get_token_type(symbol);

        Token {
            token_type,
            line,
            col,
        }
    }
}

struct LexerState {
    start: usize,
    building: bool,
    inside_quotes: bool,
    prev_char_escape: bool,
}

impl LexerState {
    fn new() -> LexerState {
        LexerState {
            start: 0,
            building: false,
            inside_quotes: false,
            prev_char_escape: false,
        }
    }
}

fn is_punctuator(c: char) -> bool {
    return c == '{' || c == '}' || c == '[' || c == ']' || c == ':' || c == ',';
}

fn is_quote(c: char) -> bool {
    return c == '"';
}

pub fn lex(s: &str) -> Vec<Token> {
    // Lexers are always long functions (at least for me every time I write one :D) 

    let mut state = LexerState::new(); 
    let mut tokens = vec![];

    for (line_no, line_str) in s.split_terminator('\n').enumerate() {
        for (col_no, c) in line_str.chars().enumerate() {
            if !state.building && c.is_whitespace() {
                continue;
            }

            if !state.building {
                state.start = col_no;
                state.building = true;
            }

            if is_punctuator(c) && !state.inside_quotes {
                if state.start != col_no {
                    let token = Token::from_key_or_val(
                        &line_str[state.start..col_no],
                        line_no,
                        state.start,
                    );

                    tokens.push(token);
                }

                tokens.push(Token::from_punctuator(c, line_no, col_no));
                state.building = false;
            }

            if c.is_whitespace() && !state.inside_quotes {
                let token =
                    Token::from_key_or_val(&line_str[state.start..col_no + 1], line_no, col_no);

                tokens.push(token);
            }

            if is_quote(c) {
                if state.prev_char_escape {
                    continue;
                }

                if !state.inside_quotes {
                    state.inside_quotes = true;
                    continue;
                }

                tokens.push(Token::from_quoted_str(
                    &line_str[state.start..col_no + 1],
                    line_no,
                    state.start,
                ));

                state.building = false;
                state.inside_quotes = false;
                continue;
            }

            if c == '\\' {
                state.prev_char_escape = true
            }
        }
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_1() {
        let tokens = lex("\"foo\" : \n [ \"bar\" }");

        let expected = vec![
            Token {
                token_type: TokenType::String("foo"),
                line: 0,
                col: 0,
            },
            Token {
                token_type: TokenType::Colon,
                line: 0,
                col: 6,
            },
            Token {
                token_type: TokenType::OpenSquare,
                line: 1,
                col: 1,
            },
            Token {
                token_type: TokenType::String("bar"),
                line: 1,
                col: 3,
            },
            Token {
                token_type: TokenType::CloseCurly,
                line: 1,
                col: 9,
            },
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_lexer_2() {
        let tokens = lex("\n { \"bar\" ]");

        let expected = vec![
            Token {
                token_type: TokenType::OpenCurly,
                line: 1,
                col: 1,
            },
            Token {
                token_type: TokenType::String("bar"),
                line: 1,
                col: 3,
            },
            Token {
                token_type: TokenType::CloseSquare,
                line: 1,
                col: 9,
            },
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_lexer_3() {
        let tokens = lex("\n {\"bar\" ] ,");

        let expected = vec![
            Token {
                token_type: TokenType::OpenCurly,
                line: 1,
                col: 1,
            },
            Token {
                token_type: TokenType::String("bar"),
                line: 1,
                col: 2,
            },
            Token {
                token_type: TokenType::CloseSquare,
                line: 1,
                col: 8,
            },
            Token {
                token_type: TokenType::Comma,
                line: 1,
                col: 10,
            }
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_lexer_4() {
        let tokens = lex("\n {\"bar\"]:\n\"foo\"");

        let expected = vec![
            Token {
                token_type: TokenType::OpenCurly,
                line: 1,
                col: 1,
            },
            Token {
                token_type: TokenType::String("bar"),
                line: 1,
                col: 2,
            },
            Token {
                token_type: TokenType::CloseSquare,
                line: 1,
                col: 7,
            },
            Token {
                token_type: TokenType::Colon,
                line: 1,
                col: 8,
            },
            Token {
                token_type: TokenType::String("foo"),
                line: 2,
                col: 0,
            },
        ];

        assert_eq!(tokens, expected);
    }
}
