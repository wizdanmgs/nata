# nata

A CLI tool that sorts files by extension or date.

## How to install

Make sure you have `rustc` and `cargo` in your system.

Clone this project, go to the project directory, and then install it with this command:

```bash
cargo install --path .
```

Now you can use `nata` as a command line tool.

## Usage

Organize files by extension

```bash
nata ~/Downloads --by extension
```

Organize files by date recursively

```bash
nata ~/Downloads --by date --recursive
```

Preview recursive organization by date

```bash
nata ~/Downloads --by date --recursive --dry-run
```

Undo last organization

```bash
nata ~/Downloads --undo
```
