use argh::FromArgs;

#[derive(FromArgs, PartialEq, Debug)]
/// joxide, CLI tool for formatting and validating JSON files
pub struct JoxideArgs {
    #[argh(subcommand)]
    pub sub_command: JoxideSubcommand,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
pub enum JoxideSubcommand {
    Format(FormatArgs),
    Validate(ValidateArgs),
}

#[derive(FromArgs, PartialEq, Debug)]
/// format json file
#[argh(subcommand, name = "format")]
pub struct FormatArgs {
    #[argh(option, default = "4")]
    /// indent length, default is 4
    pub indent_length: usize,

    #[argh(switch)]
    /// modify the file instead of printing to console
    pub write: bool,

    #[argh(positional)]
    /// path to the file you want to format
    pub file: String,
}

#[derive(FromArgs, PartialEq, Debug)]
/// validate json file for syntax errors
#[argh(subcommand, name = "validate")]
pub struct ValidateArgs {
    #[argh(positional)]
    /// path to the file you want to validate
    pub file: String,
}
