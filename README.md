# Limit Lens

A simple API for testing and visualizing rate limiters in real-time. Monitor request throughput and see how your rate limiting algorithms perform under load.

## Features

- Create test sessions to benchmark rate limiters
- Generate configurable load patterns
- Visualize request distribution in real-time
- Analyze rate limiter performance metrics

## Installation

### Prerequisites

- Rust (latest stable)

### Building from source

```bash
# Clone the repository
git clone https://github.com/yourusername/limit-lens.git
cd limit-lens

# Build the project
cargo build --release

# Run the server
cargo run --release
```

The server will start on `http://localhost:6969`.

## Usage

### Creating a test session

```bash
curl -X POST http://localhost:6969/api/test/session \
  -H "Content-Type: application/json" \
  -d '{"name": "My Test Session"}'
```

This will return a session ID that you can use for testing.

### Generating load

You can use any HTTP load testing tool. For example, with curl in a loop:

```bash
for i in {1..100}; do
  curl http://localhost:6969/api/test/request/{session_id} &
done
```

### Viewing metrics

```bash
curl http://localhost:6969/api/test/metrics/{session_id}
```

## Testing

Limit Lens includes integration tests that verify the rate limiting functionality under different load scenarios.

### Running the tests

The integration tests require [Vegeta](https://github.com/tsenart/vegeta) - an HTTP load testing tool to generate controlled HTTP load.

To install Vegeta:
- On macOS: `brew install vegeta`
- On Linux: Download from [GitHub releases](https://github.com/tsenart/vegeta/releases)
- On Windows: Use [Chocolatey](https://chocolatey.org/): `choco install vegeta`

To run the rate limit scenario tests with output:

```bash
cargo test test_rate_limit_scenarios -- --nocapture
```

This will run tests with various load patterns (low, medium, and high rates) and verify that the system correctly tracks and reports metrics.

## API Reference

Complete API documentation is available at:

- ReDoc UI: [http://localhost:6969/redoc](http://localhost:6969/redoc)

## License

MIT

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
