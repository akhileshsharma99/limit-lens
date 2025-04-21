# Limit Lens Rust Client

A Rust client library for interacting with the limit-lens API - a service for testing and visualizing rate limiters in real-time.

This client is for [limit-lens](https://github.com/akhileshsharma99/limit-lens).

## Installation

```bash
cargo add limit-lens
```

## Usage

This client provides access to the following API endpoints:

- **Health API**: Check service health status
- **Rate Test API**: Create test sessions, get metrics, and simulate requests

### Example

```rust
use limit_lens::apis::{configuration::Configuration, rate_test_api, health_api};
use limit_lens::models::CreateSessionRequest;

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up client configuration
    let config = Configuration::default();
    
    // Check API health
    let health = health_api::health_check(&config).await?;
    println!("API Health: {}", health);
    
    // Create a new test session
    let session_request = CreateSessionRequest::new();
    let session = rate_test_api::create_test_session(&config, session_request).await?;
    println!("Created session with ID: {}", session.id);
    
    // Send test requests
    for _ in 0..10 {
        let response = rate_test_api::receive_test_request(&config, &session.id).await?;
        println!("Request status: {}", response.status());
    }
    
    // Get metrics for the session
    let metrics = rate_test_api::get_test_metrics(&config, &session.id).await?;
    println!("Total requests: {}", metrics.total_requests);
    println!("Requests per second: {}", metrics.requests_per_second);
    
    // Print request distribution
    for bucket in metrics.request_distribution {
        println!("Time: {}, Count: {}", bucket.timestamp, bucket.count);
    }
    
    Ok(())
}
```

## Documentation

The library consists of two main modules:

- `apis`: Contains client methods for calling API endpoints
- `models`: Contains data structures used by the API

For detailed documentation:

```
cargo doc --open
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.

