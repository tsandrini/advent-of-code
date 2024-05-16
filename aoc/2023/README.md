# aoc2023

## Description

Hey there! :sunglasses: These are my solution to the 2023 version of AoC. I
decided to write
these in :camel: . Packaging in :camel: is a bit complicated, so I'm delegating it
to `opam` and `dune` which are installed via `nix` ❄️ (because I said so).

## Solutions

Individual solutions are implemented in `aoc2023/lib/*.ml`, they usually contain
some inline tests, which can be run via `dune runtests`.

For executing an individual AoC solution, use the `bin/main.ml` entrypoint

```bash
dune exec bin/main.exe -- 03part1 data/day03
```

Feel free to ~take~ ... uh - be Inspired^TM. :sunglasses:
