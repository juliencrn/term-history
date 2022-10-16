# term-history

That little Rust powered CLI apps shows you which terminal programs you use most often.

## Usage

1. Get your history file path

```bash
echo $HISTFILE
```

2. Call the program with that file path as argument

```bash
cargo run <path/to/history/file>
```