# Advent of code solutions

Solutions for [Advent of Code](https://adventofcode.com/) in [Rust](https://www.rust-lang.org/).

## Setup

1.  Install the [Rust toolchain](https://www.rust-lang.org/tools/install).
2.  (recommended) Install the [rust-analyzer](https://rust-analyzer.github.io/manual.html) extension for your code editor.
3.  (optional) Install a native debugger. If you are using VS Code, [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb) is a good option.

## Usage

### Scaffold a day

```shell
# example: `cargo scaffold 1 -y 2021`
cargo scaffold <year>-<day>

# output
# [INFO  aoc] 🎄 aoc-cli - Advent of Code command-line tool
# [INFO  aoc_client] 🎅 Saved puzzle to 'src/bin/2021-02/puzzle.md'
# [INFO  aoc_client] 🎅 Saved input to 'src/bin/2021-02/input.txt'
# Fetched puzzle and input from aoc website
# ---
# 🎄 Type `cargo solve 2021-02` to run your solution.
```

### Solve a day

```shell
# example: cargo solve 2021-01
cargo solve <year>-<day>

# output
# Part 1 response: 1288
# Part 2 response: 1311
```

### Submit a day


```shell
# example: cargo solve 2021-01 -- --submit
cargo solve <year>-<day> -- --submit
```

# Acknowledgements

This repository is inspired by [this one from fspoettel](https://github.com/fspoettel/advent-of-code-rust)
