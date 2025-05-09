/*
 * Limit Lens API
 *
 * A simple API for testing and visualizing rate limiters in real-time. Monitor request throughput and see how your rate limiting algorithms perform under load.
 *
 * The version of the OpenAPI document: 0.6.1
 * Contact: sharmaninenine@gmail.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// TestMetrics : Metrics for a rate limit test session
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestMetrics {
    /// Timestamp of the first request
    #[serde(rename = "first_request_time", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub first_request_time: Option<Option<String>>,
    /// Timestamp of the last request
    #[serde(rename = "last_request_time", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub last_request_time: Option<Option<String>>,
    /// Time-bucketed request counts (each entry is a 1-second window)
    #[serde(rename = "request_distribution")]
    pub request_distribution: Vec<models::TimeBucket>,
    /// Calculated requests per second
    #[serde(rename = "requests_per_second")]
    pub requests_per_second: f64,
    /// ID of the test session
    #[serde(rename = "session_id")]
    pub session_id: String,
    /// Total number of requests received
    #[serde(rename = "total_requests")]
    pub total_requests: i32,
}

impl TestMetrics {
    /// Metrics for a rate limit test session
    pub fn new(request_distribution: Vec<models::TimeBucket>, requests_per_second: f64, session_id: String, total_requests: i32) -> TestMetrics {
        TestMetrics {
            first_request_time: None,
            last_request_time: None,
            request_distribution,
            requests_per_second,
            session_id,
            total_requests,
        }
    }
}

