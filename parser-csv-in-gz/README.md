# Parser CSV in GZ

## Quick Start ⚡️
```bash
# Run with Cargo
cargo run -- datasets/2019-Oct.test.csv.gz

# If Run single-perf-csv
cargo run -- < datasets/2019-Oct.test.csv
```

## Build 🚀
```bash
# build a single
cargo build --release --bin single

# build a single-perf-csv
cargo build --release --bin single-perf-csv

# build a parallel
cargo build --release --bin parallel
```