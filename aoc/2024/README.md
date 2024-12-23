# 2024

## Description

My solutions to AoC in 2024. Using rust and

-   the [fspoettel/advent-of-code-rust](https://github.com/fspoettel/advent-of-code-rust)
    template
-   the [tsandrini/flake-parts-builder](https://github.com/tsandrini/flake-parts-builder)
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

| Day          | Part 1         |  Part 2       |
| :---         |     :---      |          :--- |
| 01 | 2970687 (126.5µs @ 6579 samples) | 23963899 (141.1µs @ 6394 samples) |
| 02 | 287 (258.7µs @ 3319 samples) | 354 (576.8µs @ 1690 samples) |
| 03 |  164730528 (322.6µs @ 1523 samples) | 70478672 (445.6µs @ 2075 samples) |
| 04 |  2646 (962.4µs @ 838 samples) |  2000 (484.3µs @ 1876 samples) |
| 05 | 5268 (602.3µs @ 1118 samples) | 5799 (1.2ms @ 896 samples) |
| 06 |  5131 (337.4µs @ 1720 samples) | 1784 (260.8ms @ 10 samples) |
| 07 |  12553187650171 (12.1ms @ 63 samples) |  96779702119491 (595.8ms @ 10 samples) |
| 08 |  244 (578.3µs @ 707 samples) | 912 (1.4ms @ 791 samples) |
| 09 | 6283170117911 (1.3ms @ 381 samples) | 6307653242596 (307.6ms @ 10 samples) |
| 10 | 552 (389.5µs @ 1260 samples) |  1225 (306.0µs @ 2618 samples) |
| 11 | 189092 (383.7µs @ 2005 samples) | 224869647102559 (22.9ms @ 40 samples) |
| 12 | 1319878 (3.7ms @ 276 samples) |  784982 (15.9ms @ 59 samples) |
| 13 |  29388 (379.0µs @ 1757 samples) |  99548032866004 (373.4µs @ 3299 samples) |
| 14 |  79600140 (138.8µs @ 6560 samples) | ✖ (solved by input inspection) |
| 15 | 55764 (469.1µs @ 2111 samples) |  9824 (796.2µs @ 1268 samples) |
| 16 |  102504 (11.2ms @ 57 samples) | 535 (20.3ms @ 39 samples) |
| 17 | 146164303 (1.9µs @ 10000 samples) | 265061364597659 (6.6µs @ 10000 samples)  |
| 18 |  436 (1.4ms @ 791 samples) | 61,50 (862.3ms @ 10 samples) |
| 19 | 355 (331.5µs @ 215 samples) | 732978410442050 (335.0µs @ 24 samples) |
| 20 | 1441 (1.6ms @ 587 samples) | 1021490 (10.8s @ 10 samples) | 
| 21 | 212488 (973.8µs @ 893 samples) | 258263972600402 (984.8µs @ 645 samples) |
| 22 | 14392541715 (6.6ms @ 149 samples) | 1628 (148.7ms @ 10 samples) |
