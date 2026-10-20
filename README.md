# wfcr

Implementation of heuristic and metaheuristic algorithms for the WFCR problem
(cable routing in wind farms).

## Requirement: Rust nightly

This project **does not build on Rust stable**. It uses the `portable_simd`
feature (`src/lib.rs`), which is still nightly-only in Rust.

The correct channel is already pinned in `rust-toolchain.toml`, so you just
need [rustup](https://rustup.rs/) installed — it downloads and uses the right
nightly automatically whenever you run any `cargo` command inside this
repository. There is no need to run `rustup default nightly` or anything
manual.

## How to run

```sh
# Build
cargo build

# Run the tests
cargo test

# Run the binary (see `cargo run -- --help` for the available options)
cargo run -- --help
```

## Lint

```sh
cargo clippy --all-targets
```

## How to use 

You can choose to run one algorithm with one test instance, or run all the algorithms with all the test instances `N` times. 

- `run one`: You can choose which test instance (`.turb`), and which algorithm you will run. The trial (solution, cost and execution time) is saved to `sqlite.db`.
- `run all`: You define in `wfcr.toml` which algorithms you will run, how many times it will be repeated. For "official" comparison, the ideal is to execute each algorithm 30 times and then take the median (cost/time of execution) of each 30 executions. All the statistics of the executions are saved to `sqlite.db`.

Visualization is done in Python, in the Jupyter notebooks under `analyze/`:
`visualize.ipynb` draws a solution stored in `sqlite.db` and `analysis.ipynb` produces
the aggregate charts and tables.

```
❯ wfcr 
CLI to run WFCR algorithms and save the results.

Usage: wfcr <COMMAND>

Commands:
  run      Runs a specific algorithm to one instance of WFCR
  run-all  Run all the algorithms with all instances defined in `wfcr.toml`
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

