# Tripos-Paper-Opener

Small CLI for opening tripos papers in the browser (on camcribs)

## Installation

```bash
cargo install --git https://github.com/Tommypop2/tripos-paper-opener
```

## Usage

```bash
Usage: tripos [-c] [-b] [-t] [--] <module> <year>

Open a given tripos paper

Positional Arguments:
  module
  year

Options:
  -c, --crib        open the crib
  -b, --both        open both question paper and crib
  -t, --together    open question paper and crib together (on same page)
  --help, help      display usage information
```

### Examples

- `tripos 1P1 2016`     - Open the 2016 1P1 question paper
- `tripos 1P2 2017 -c`  - Open the 2017 1P2 crib
- `tripos 1P2 2017 -b`  - Open the 2017 1P2 question paper and crib (in separate tabs)
- `tripos 1P3 2017 -t`  - Open the 2017 1P3 question paper and crib (side by side)
