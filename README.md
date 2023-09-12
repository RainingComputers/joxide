# joxide

joxide is a CLI tool to validate and format json files written in rust. The parser, validator and the formatter are completely written from scratch and uses [argh](https://github.com/google/argh) to parse CLI arguments.

_test.json_

```json
{
    "hello": "world",
    "numbers": [
        2
        3
    ]
}
```

_Validate_

```
> joxide validate test.json
At test.json:5:9
        3
        ^
Did not expect '3', expected ']'. Forgot a comma maybe?
```

## Install

```
cargo install joxide
```

## Usage

_Formatting files_

```
joxide format <path-or-glob-or-dir> [--indent-length <indent-length>] [--write]
```

_Validating files_

```
joxide validate <path-or-glob-or-dir>
```
