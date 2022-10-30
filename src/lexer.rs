#[derive(Debug, PartialEq)]
pub enum TokenType<'a> {
    Null,
    Bool(bool),
    Number(f64),
    String(&'a str),
    Invalid(&'a str),
    OpenCurly,
    CloseCurly,
    OpenSquare,
    CloseSquare,
    Colon,
    Comma,
}

#[derive(Debug, PartialEq)]
pub struct Token<'a> {
    pub token_type: TokenType<'a>,
    pub line: usize,
    pub col: usize,
}

impl<'a> Token<'a> {
    fn from_punctuator(c: char, line: usize, col: usize) -> Token<'a> {
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

    fn from_quoted_str(string: &'a str, line: usize, col: usize) -> Token {
        let token_string = &string[1..string.len() - 1];

        Token {
            token_type: TokenType::String(token_string),
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
                    _ => TokenType::Invalid(symbol),
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

fn is_punctuator(c: char) -> bool {
    return c == '{' || c == '}' || c == '[' || c == ']' || c == ':' || c == ',';
}

fn is_quote(c: char) -> bool {
    return c == '"';
}

pub fn lex(s: &str) -> Vec<Token> {
    let mut tokens = vec![];

    let mut start: usize = 0;
    let mut building = false;
    let mut inside_quotes = false;
    let mut prev_char_escape = false;

    for (line_no, line_str) in s.split_terminator('\n').enumerate() {
        for (col_no, c) in line_str.chars().enumerate() {
            if !building {
                if c.is_whitespace() {
                    continue;
                }

                start = col_no;
                building = true;
            }

            if !inside_quotes {
                if is_punctuator(c) {
                    if start != col_no {
                        let token =
                            Token::from_key_or_val(&line_str[start..col_no], line_no, start);

                        tokens.push(token);
                    }

                    tokens.push(Token::from_punctuator(c, line_no, col_no));
                    building = false;
                } else if c.is_whitespace() {
                    let token = Token::from_key_or_val(&line_str[start..col_no], line_no, start);

                    tokens.push(token);
                    building = false;
                } else if col_no == line_str.len() - 1 {
                    let token =
                        Token::from_key_or_val(&line_str[start..col_no + 1], line_no, start);

                    tokens.push(token);
                    building = false;
                }
            }

            if is_quote(c) {
                if prev_char_escape {
                    continue;
                }

                if !inside_quotes {
                    inside_quotes = true;
                    continue;
                }

                tokens.push(Token::from_quoted_str(
                    &line_str[start..col_no + 1],
                    line_no,
                    start,
                ));

                building = false;
                inside_quotes = false;
            }

            if c == '\\' {
                prev_char_escape = true
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
            },
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_lexer_4() {
        let tokens = lex("\n {\"bar\"]");

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
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_lexer_5() {
        let tokens = lex("\n {bar]:\n\"foo\"");

        let expected = vec![
            Token {
                token_type: TokenType::OpenCurly,
                line: 1,
                col: 1,
            },
            Token {
                token_type: TokenType::Invalid("bar"),
                line: 1,
                col: 2,
            },
            Token {
                token_type: TokenType::CloseSquare,
                line: 1,
                col: 5,
            },
            Token {
                token_type: TokenType::Colon,
                line: 1,
                col: 6,
            },
            Token {
                token_type: TokenType::String("foo"),
                line: 2,
                col: 0,
            },
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_lexer_6() {
        let tokens = lex("bar");

        let expected = vec![Token {
            token_type: TokenType::Invalid("bar"),
            line: 0,
            col: 0,
        }];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_lexer_7() {
        let tokens = lex("\"bar\"");

        let expected = vec![Token {
            token_type: TokenType::String("bar"),
            line: 0,
            col: 0,
        }];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_lexer_8() {
        let tokens = lex("2345");

        let expected = vec![Token {
            token_type: TokenType::Number(2345.0),
            line: 0,
            col: 0,
        }];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_lexer_9() {
        let tokens = lex("2345}");

        let expected = vec![
            Token {
                token_type: TokenType::Number(2345.0),
                line: 0,
                col: 0,
            },
            Token {
                token_type: TokenType::CloseCurly,
                line: 0,
                col: 4,
            },
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_lexer_10() {
        let tokens = lex("2345      } 456 ");

        let expected = vec![
            Token {
                token_type: TokenType::Number(2345.0),
                line: 0,
                col: 0,
            },
            Token {
                token_type: TokenType::CloseCurly,
                line: 0,
                col: 10,
            },
            Token {
                token_type: TokenType::Number(456.0),
                line: 0,
                col: 12,
            },
        ];

        assert_eq!(tokens, expected);
    }
}
