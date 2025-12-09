# 2025

## Description

My solutions to AoC in 2025. Using rust and

- the [fspoettel/advent-of-code-rust](https://github.com/fspoettel/advent-of-code-rust)
  template
- the [tsandrini/flake-parts-builder](https://github.com/tsandrini/flake-parts-builder)
  nix flakes template builder

## Running

Enter the development shell (or acquire rust, rustc, cargo in your preferred way)

```bash
direnv allow
```

or

```bash
nix develop
```

And then

```bash
cargo build
cargo solve XY
cargo test
cargo time XY
```

And more.

## Stats

Note that these were ran on my poor laptop and measuring performance is hard, and
yodayodayoda, you know the deal, it's just for fun.

| Day          | Part 1                      |  Part 2                       |
| :---         |     :---                    | :---                          |
| 1            | 962 (94.9µs @ 7490 samples) | 5782 (102.2µs @ 9938 samples) |
| 2            | 18952700150 (2.2µs @ 10000 samples) | 28858486244 (20.8µs @ 10000 samples) |
| 3            | 17452 (49.7µs @ 1824 samples) | 173300819005913 (55.2µs @ 8675 samples)|
| 4            | 1502 (103.2µs @ 2055 samples) | 9083 (1.9ms @ 344 samples) |
| 5            | 848 (59.7µs @ 10000 samples)  | 334714395325710 (22.5µs @ 10000 samples) |
| 6            | 5595593539811 (94.7µs @ 9505 samples) | 10153315705125 (274.8µs @ 2557 samples) |
| 7            | 1613 (199.4µs @ 4242 samples) | 48021610271997 (202.3µs @ 3961 samples) |
| 8            | 24360 (22.8ms @ 36 samples) | 2185817796 (21.5ms @ 41 samples) |
