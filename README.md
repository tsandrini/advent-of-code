# advent-of-code

## Description

These are my personal solutions to the [advent-of-code](https://adventofcode.com)
procrastination shenanigans. Instead of having multiple repositories I prefer to
have them all in one.

Every year I might switch and try out a different language, that's why each year
has its own root and tooling.

## Nix

For setting up reproducible tooling and developer environments I use
[Nix](https://nixos.org/) and my personal
[tsandrini/flake-parts-builder](https://github.com/tsandrini/flake-parts-builder/)
template builder which you can find in every year. This makes it fairly easy to
set up radically different tooling every year.

## Solutions

- **2025**: Using Rust and a modified and nixified version of
  the [fspoettel/advent-of-code-rust](https://github.com/fspoettel/advent-of-code-rust)
  template, yet again. I kinda wanted to use my
  [rust-tinynix](github.com/tsandrini/rust-tinynix) project, but I have very
  little time unfortunately TwT >.<
- **2024**: Using Rust and a modified and nixified version of
  the [fspoettel/advent-of-code-rust](https://github.com/fspoettel/advent-of-code-rust)
  template. I initially wanted to goof around with pure nix but I am unfortunately
  pretty busy so Rust it is. _Solutions_ located at `aoc/2024/src/bin/*.rs`
- **2023**: Used OCaml [OCaml](https://ocaml.org/) this year. Pretty good year,
  missed only the last problems. _Solutions_ located at `aoc/2023/aoc2023/lib/*.ml`
- **2022**: Using Rust and a modified and nixified version of
  the [fspoettel/advent-of-code-rust](https://github.com/fspoettel/advent-of-code-rust)
  template. _Solutions_ located at `aoc/2022/src/bin/*.rs`
