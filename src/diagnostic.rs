use crate::{
    lexer::TokenType,
    parser::{ParseError, ParseErrorType},
};

fn get_message_unexpected_token<'a>(parse_error: &'a ParseError<'a>) -> String {
    match parse_error.token {
        None => "Did not expect this word or character".to_owned(),
        Some(token) => match parse_error.expected {
            None => format!("Did not expect '{}'", token.token_type),
            Some(expected) => match expected {
                TokenType::CloseCurly | TokenType::CloseSquare => format!(
                    "Did not expect '{}', expected '{}'. Forgot a comma maybe?",
                    token.token_type, expected
                ),
                _ => format!(
                    "Did not expect '{}', expected '{}'",
                    token.token_type, expected
                ),
            },
        },
    }
}

pub fn get_message<'a>(parse_error: &'a ParseError<'a>) -> String {
    match parse_error.error_type {
        ParseErrorType::UnexpectedEnd => "File ended unexpectedly".to_owned(),
        ParseErrorType::UnexpectedToken => get_message_unexpected_token(parse_error),
        ParseErrorType::TrailingComma => "Trailing commas are not valid".to_owned(),
        ParseErrorType::DuplicateKey => "Duplicate keys are not valid".to_owned(),
        ParseErrorType::KeyNotInQuotes => "Key should be in quotes".to_owned(),
        ParseErrorType::MissingColon => "Missing a ':' separator".to_owned(),
    }
}
