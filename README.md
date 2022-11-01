# joxide

joxide is a CLI tool to validate and formate json files written in rust. The parser and the validator is completely written from scratch and uses [argh](https://github.com/google/argh) to parse CLI arguments.

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

## Usage

**Formatting files**

```
joxide format <file> [--indent-length <indent-length>] [--write]
```

**Validating files**

```
joxide validate <file>
```
