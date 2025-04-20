#[cfg(test)]
mod rate_test {
    use crate::models::{TestSession, TestMetrics};
    use actix_web::{test, web, App};
    use serde_json::json;
    use std::time::Duration;
    use tokio::time;
    use std::process::Command;
    use std::io::Write;
    use tempfile::NamedTempFile;

    use crate::routes;
    use crate::storage::RateTestStorage;

    // Helper struct to define test attack scenarios
    struct AttackScenario {
        name: &'static str,
        rate: u32,        // requests per second
        duration: u32,    // seconds
    }

    // Helper function to set up and run a rate limit test
    async fn run_rate_test(scenario: &AttackScenario) -> TestMetrics {
        // Setup test app
        let rate_test_data = web::Data::new(RateTestStorage::new());
        let app = test::init_service(
            App::new()
                .app_data(rate_test_data.clone())
                .route("/api/test/session", web::post().to(routes::rate_test::create_test_session))
                .route("/api/test/request/{session_id}", web::get().to(routes::rate_test::receive_test_request))
                .route("/api/test/metrics/{session_id}", web::get().to(routes::rate_test::get_test_metrics)),
        )
        .await;

        // Create a new test session
        let create_req = test::TestRequest::post()
            .uri("/api/test/session")
            .set_json(&json!({"name": scenario.name}))
            .to_request();
        
        let session: TestSession = test::call_and_read_body_json(&app, create_req).await;
        assert!(!session.id.is_empty());
        assert_eq!(session.name, Some(scenario.name.to_string()));

        // Server needs to be running for Vegeta to hit it
        // For testing purposes, we'll start a test server on a specific port
        let server = actix_web::rt::spawn(
            actix_web::HttpServer::new(move || {
                App::new()
                    .app_data(rate_test_data.clone())
                    .route("/api/test/session", web::post().to(routes::rate_test::create_test_session))
                    .route("/api/test/request/{session_id}", web::get().to(routes::rate_test::receive_test_request))
                    .route("/api/test/metrics/{session_id}", web::get().to(routes::rate_test::get_test_metrics))
            })
            .bind("0.0.0.0:6969")
            .unwrap()
            .run()
        );

        // Give the server time to start
        time::sleep(Duration::from_millis(500)).await;

        // Create a temporary file for vegeta targets
        let mut targets_file = NamedTempFile::new().expect("Failed to create temporary file");
        writeln!(
            targets_file,
            "GET http://localhost:6969/api/test/request/{}",
            session.id
        ).expect("Failed to write to targets file");
        
        // Run vegeta attack command
        let output = Command::new("vegeta")
            .args(&[
                "attack", 
                "-targets", targets_file.path().to_str().unwrap(),
                "-rate", &format!("{}/s", scenario.rate),
                "-duration", &format!("{}s", scenario.duration)
            ])
            .output()
            .expect("Failed to execute vegeta attack command");
        
        assert!(output.status.success(), "Vegeta attack command failed");
        
        // Wait a bit to make sure all requests are processed
        time::sleep(Duration::from_secs(2)).await;

        // Get metrics 
        let metrics_req = test::TestRequest::get()
            .uri(&format!("/api/test/metrics/{}", session.id))
            .to_request();
        
        let metrics: TestMetrics = test::call_and_read_body_json(&app, metrics_req).await;
        
        // Stop the server
        server.abort();
        
        metrics
    }

    #[actix_web::test]
    async fn test_rate_limit_scenarios() {
        // Define multiple test scenarios
        let scenarios = vec![
            AttackScenario {
                name: "Low Rate Test",
                rate: 10,
                duration: 5,
            },
            AttackScenario {
                name: "Medium Rate Test",
                rate: 50,
                duration: 5,
            },
            AttackScenario {
                name: "High Rate Test",
                rate: 200,
                duration: 5,
            },
        ];

        // Run each scenario and validate results
        for scenario in scenarios {
            println!("Running scenario: {}", scenario.name);
            
            let metrics = run_rate_test(&scenario).await;
            let expected_requests = scenario.rate * scenario.duration;
            
            // Verify metrics
            assert_eq!(metrics.total_requests, expected_requests as usize, 
                       "Failed on scenario '{}': expected {} requests, got {}", 
                       scenario.name, expected_requests, metrics.total_requests);
            
            // The actual rate should be close to our target rate (within 20% tolerance)
            let target_rate_f64 = scenario.rate as f64;
            let rate_tolerance = 0.2; 
            let min_rate = target_rate_f64 * (1.0 - rate_tolerance);
            let max_rate = target_rate_f64 * (1.0 + rate_tolerance);

            println!("  Target rate: {} requests/second", target_rate_f64);
            println!("  Min acceptable rate: {} requests/second", min_rate);
            println!("  Max acceptable rate: {} requests/second", max_rate);
            println!("  Measured rate: {} requests/second", metrics.requests_per_second);
            
            assert!(metrics.requests_per_second >= min_rate,
                   "Failed on scenario '{}': rate too low ({} < {})", 
                   scenario.name, metrics.requests_per_second, min_rate);
            assert!(metrics.requests_per_second <= max_rate,
                   "Failed on scenario '{}': rate too high ({} > {})", 
                   scenario.name, metrics.requests_per_second, max_rate);
            
            // Check distribution buckets
            assert!(!metrics.request_distribution.is_empty(),
                   "Failed on scenario '{}': request distribution is empty", 
                   scenario.name);
        }
    }
} 