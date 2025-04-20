use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use chrono::{DateTime, Utc};

/// Represents a test session for rate limit testing
#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct TestSession {
    /// Unique identifier for the test session
    pub id: String,
    /// Name of the test session (optional)
    #[schema(example = "My Rate Limit Test")]
    pub name: Option<String>,
    /// When the test session was created
    pub created_at: DateTime<Utc>,
}

/// Metrics for a rate limit test session
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TestMetrics {
    /// ID of the test session
    pub session_id: String,
    /// Total number of requests received
    pub total_requests: usize,
    /// Timestamp of the first request
    pub first_request_time: Option<DateTime<Utc>>,
    /// Timestamp of the last request
    pub last_request_time: Option<DateTime<Utc>>,
    /// Calculated requests per second
    pub requests_per_second: f64,
    /// Time-bucketed request counts (each entry is a 1-second window)
    pub request_distribution: Vec<TimeBucket>,
}

/// Time-bucketed request counts
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TimeBucket {
    /// Start time of this 1-second bucket
    pub timestamp: DateTime<Utc>,
    /// Number of requests in this 1-second window
    pub count: usize,
}

/// Represents a request received during testing
#[derive(Debug, Clone)]
pub struct TestRequest {
    /// Timestamp when the request was received
    pub timestamp: DateTime<Utc>,
} 