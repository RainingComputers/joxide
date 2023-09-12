extern crate argh;
extern crate glob;

use crate::args::JoxideSubcommand;
use glob::{glob, GlobError, Paths, PatternError};
use pretty::eprint_parse_error;
use std::{path::PathBuf, process::ExitCode};

mod args;
mod diagnostic;
mod formatter;
mod lexer;
mod parser;
mod pretty;

fn main() -> ExitCode {
    let args: args::JoxideArgs = argh::from_env();

    let path_matchers = match args.sub_command {
        JoxideSubcommand::Validate(ref validate_args) => &validate_args.paths,
        JoxideSubcommand::Format(ref format_args) => &format_args.paths,
    };

    let failures: Vec<Result<(), ()>> = path_matchers
        .iter()
        .map(get_glob)
        .map(|glob_result| process_glob(glob_result, &args.sub_command))
        .collect();

    match failures.len() {
        0 => ExitCode::SUCCESS,
        _ => ExitCode::FAILURE,
    }
}

fn get_glob(path: &String) -> Result<Paths, PatternError> {
    match std::fs::metadata(path) {
        Ok(metadata) => match metadata.is_dir() {
            true => glob(&format!("{}/**/*.json", path)),
            false => glob(path),
        },
        Err(_) => glob(path),
    }
}

fn process_glob(
    glob_result: Result<Paths, PatternError>,
    sub_command: &JoxideSubcommand,
) -> Result<(), ()> {
    match glob_result {
        Ok(paths) => paths
            .into_iter()
            .map(|entry| process_glob_entry(entry, sub_command))
            .filter(|result| result.is_err())
            .collect(),
        Err(err) => {
            eprint!("Invalid glob pattern, reason: {}", err);
            Err(())
        }
    }
}

fn process_glob_entry(
    entry: Result<PathBuf, GlobError>,
    sub_command: &JoxideSubcommand,
) -> Result<(), ()> {
    match entry {
        Ok(path) => process_file(&path, sub_command),
        Err(err) => {
            eprint!("Unable to do a glob pattern match, reason: {}", err);
            Err(())
        }
    }
}

fn process_file(file_path: &PathBuf, sub_command: &JoxideSubcommand) -> Result<(), ()> {
    let raw = match std::fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Unable to open file, reason: {}", err);
            return Err(());
        }
    };

    let tokens = lexer::lex(&raw);

    let parsed_value = match parser::parse(&tokens) {
        Ok(value) => value,
        Err(parse_error) => {
            eprint_parse_error(parse_error, &raw, file_path);
            return Err(());
        }
    };

    match sub_command {
        JoxideSubcommand::Format(format_args) => format_file(parsed_value, format_args, file_path),
        JoxideSubcommand::Validate(_) => Ok(()),
    }
}

fn format_file(
    value: parser::Json<'_>,
    format_args: &args::FormatArgs,
    file_path: &PathBuf,
) -> Result<(), ()> {
    let formatted = formatter::format_json(value, format_args.indent_length);

    if format_args.write {
        if let Err(err) = std::fs::write(file_path, formatted) {
            eprintln!("Unable to write to file, reason: {}", err);
            return Err(());
        }
    } else {
        println!("{}", formatted);
    }

    Ok(())
}
