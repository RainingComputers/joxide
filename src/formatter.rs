use crate::lexer::TokenType;
use parser::Json;
use std::{collections::BTreeMap, usize};

fn comma_builder<I, T, F>(
    length: usize,
    iterator: I,
    open: TokenType,
    formatter: F,
    ilvl: usize,
    ilen: usize,
) -> String
where
    F: Fn(T, usize, usize) -> String,
    I: std::iter::Iterator<Item = T>,
{
    let mut result = match open {
        TokenType::OpenCurly => "{".to_string(),
        _ => "[".to_string(),
    };

    for (index, item) in iterator.enumerate() {
        if ilen != 0 {
            result += format!(
                "\n{}{}",
                " ".repeat((ilvl + 1) * ilen),
                formatter(item, ilvl + 1, ilen)
            )
            .as_str();
        } else {
            result += formatter(item, ilvl, ilen).as_str();
        }

        if index != length - 1 {
            result += ",";
        }
    }

    if ilen != 0 {
        result += "\n";
        result += &" ".repeat(ilvl * ilen);
    }

    result += match open {
        TokenType::OpenCurly => "}",
        _ => "]",
    };

    result
}

fn array(arr: Box<Vec<Json>>, ilvl: usize, ilen: usize) -> String {
    comma_builder(
        arr.len(),
        arr.into_iter(),
        TokenType::OpenSquare,
        value,
        ilvl,
        ilen,
    )
}

fn object(obj: Box<BTreeMap<&str, Json>>, ilvl: usize, ilen: usize) -> String {
    let formatter = |item: (&str, Json), ilvl: usize, ilen: usize| -> String {
        let (key, val) = item;

        if ilen != 0 {
            format!("\"{}\": {}", key, value(val, ilvl, ilen))
        } else {
            format!("\"{}\":{}", key, value(val, ilvl, ilen))
        }
    };

    comma_builder(
        obj.len(),
        obj.into_iter(),
        TokenType::OpenCurly,
        formatter,
        ilvl,
        ilen,
    )
}

fn value(val: Json, ilvl: usize, ilen: usize) -> String {
    match val {
        Json::Null => "null".to_string(),
        Json::Bool(b) => format!("{}", b),
        Json::Number(n) => format!("{}", n),
        Json::String(s) => format!("\"{}\"", s),
        Json::Object(obj) => object(obj, ilvl, ilen),
        Json::Array(arr) => array(arr, ilvl, ilen),
    }
}

pub fn format_json(val: Json, indent_length: usize) -> String {
    value(val, 0, indent_length)
}

#[cfg(test)]
mod tests {
    use crate::{lexer, parser};

    use super::format_json;

    #[test]
    fn test_formatter() {
        let raw = "{\"foo\":[1,{\"bar\":{\"foo\":\"bar\"},\"foo\":[{\"foo\":\"bar\"},{\"foo\":\"bar\"}]},3,4],\"hello\":\"world\",\"qaz\":\"{\\\"bar\\\":0}\"}";

        for i in 0..10 {
            let tokens = lexer::lex(raw);
            let value = parser::parse(&tokens).unwrap();
            let formatted = format_json(value, i);

            let tokens_rev = lexer::lex(&formatted);
            let value_rev = parser::parse(&tokens_rev).unwrap();
            let formatted_rev = format_json(value_rev, 0);

            assert_eq!(formatted_rev, raw);
        }
    }

    #[test]
    fn test_indent() {
        let expected = [
            "{\"a\":\"b\",\"f\":[{\"a\":\"b\"},1,2,\"three\"]}",
            "{\n \"a\": \"b\",\n \"f\": [\n  {\n   \"a\": \"b\"\n  },\n  1,\n  2,\n  \"three\"\n ]\n}",
            "{\n  \"a\": \"b\",\n  \"f\": [\n    {\n      \"a\": \"b\"\n    },\n    1,\n    2,\n    \"three\"\n  ]\n}",
            "{\n   \"a\": \"b\",\n   \"f\": [\n      {\n         \"a\": \"b\"\n      },\n      1,\n      2,\n      \"three\"\n   ]\n}",
            "{\n    \"a\": \"b\",\n    \"f\": [\n        {\n            \"a\": \"b\"\n        },\n        1,\n        2,\n        \"three\"\n    ]\n}"
        ];

        for i in 0..5 {
            let raw = expected[0];

            let tokens = lexer::lex(raw);
            let value = parser::parse(&tokens).unwrap();
            let formatted = format_json(value, i);

            assert_eq!(formatted, expected[i]);
        }
    }
}
