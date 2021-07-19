# Kafka Practice

## Quick Start ⚡️
```bash
# Run Producer
cargo run --bin producer -- --topic test-async-processing

# Run Stream Consumer
cargo run --bin async_processing -- --input-topic test-async-processing --output-topic test-async-result

# Run Consumer
cargo run --bin consumer -- --topics test-async-result
```