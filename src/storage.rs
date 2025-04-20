use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use chrono::{TimeZone, Utc};

use crate::models::{TestMetrics, TestRequest, TestSession, TimeBucket, WorkerMetrics};

/// Storage for rate test data
pub struct RateTestStorage {
    /// Map of session_id to test session
    sessions: Arc<Mutex<HashMap<String, TestSession>>>,
    /// Map of session_id to list of test requests
    requests: Arc<Mutex<HashMap<String, Vec<TestRequest>>>>,
}

impl RateTestStorage {
    /// Create a new storage instance
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(Mutex::new(HashMap::new())),
            requests: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Create a new test session
    pub fn create_session(&self, name: Option<String>) -> TestSession {
        let session_id = Uuid::new_v4().to_string();
        let session = TestSession {
            id: session_id.clone(),
            name,
            created_at: Utc::now(),
        };

        // Store the session
        self.sessions
            .lock()
            .unwrap()
            .insert(session_id.clone(), session.clone());
        
        // Initialize request storage for this session
        self.requests
            .lock()
            .unwrap()
            .insert(session_id.clone(), Vec::new());
        
        session
    }

    /// Record a test request for a specific session
    pub fn record_request(&self, session_id: &str, worker_id: &str) -> bool {
        if !self.session_exists(session_id) {
            return false;
        }

        let request = TestRequest {
            timestamp: Utc::now(),
            worker_id: worker_id.to_string(),
        };

        // Store the request
        self.requests
            .lock()
            .unwrap()
            .get_mut(session_id)
            .map(|requests| requests.push(request));
        
        true
    }

    /// Check if a session exists
    pub fn session_exists(&self, session_id: &str) -> bool {
        self.sessions
            .lock()
            .unwrap()
            .contains_key(session_id)
    }

    /// Get metrics for a session
    pub fn get_metrics(&self, session_id: &str) -> Option<TestMetrics> {
        if !self.session_exists(session_id) {
            return None;
        }

        let requests = self.requests.lock().unwrap();
        let session_requests = requests.get(session_id)?;
        
        if session_requests.is_empty() {
            return Some(TestMetrics {
                session_id: session_id.to_string(),
                total_requests: 0,
                first_request_time: None,
                last_request_time: None,
                requests_per_second: 0.0,
                request_distribution: Vec::new(),
                per_worker_metrics: HashMap::new(),
            });
        }

        // Calculate metrics
        let first_time = session_requests.first().map(|r| r.timestamp);
        let last_time = session_requests.last().map(|r| r.timestamp);
        
        let mut rps = 0.0;
        if let (Some(first), Some(last)) = (first_time, last_time) {
            let duration = last.signed_duration_since(first);
            let seconds = duration.num_seconds().max(1) as f64;
            rps = session_requests.len() as f64 / seconds;
        }

        // Generate time-bucketed distribution (1-second windows)
        let mut distribution = HashMap::new();
        for request in session_requests.iter() {
            // Round timestamp to the nearest second
            let timestamp = request.timestamp.timestamp();
            let bucket_timestamp = Utc.timestamp_opt(timestamp, 0).single().unwrap();
            
            *distribution.entry(bucket_timestamp).or_insert(0) += 1;
        }

        // Convert to sorted vector
        let mut distribution_vec: Vec<TimeBucket> = distribution
            .into_iter()
            .map(|(timestamp, count)| TimeBucket { timestamp, count })
            .collect();

        distribution_vec.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

        // Calculate per-worker metrics
        let mut per_worker_metrics = HashMap::new();
        let mut worker_requests: HashMap<String, Vec<&TestRequest>> = HashMap::new();
        
        // Group requests by worker_id
        for request in session_requests.iter() {
            worker_requests
                .entry(request.worker_id.clone())
                .or_insert_with(Vec::new)
                .push(request);
        }
        
        // Calculate metrics for each worker
        for (worker_id, requests) in worker_requests {
            let worker_count = requests.len();
            
            // Calculate time distribution for this worker
            let mut worker_distribution = HashMap::new();
            for request in requests {
                let timestamp = request.timestamp.timestamp();
                let bucket_timestamp = Utc.timestamp_opt(timestamp, 0).single().unwrap();
                
                *worker_distribution.entry(bucket_timestamp).or_insert(0) += 1;
            }
            
            // Convert to sorted vector
            let mut worker_distribution_vec: Vec<TimeBucket> = worker_distribution
                .into_iter()
                .map(|(timestamp, count)| TimeBucket { timestamp, count })
                .collect();
                
            worker_distribution_vec.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
            
            per_worker_metrics.insert(worker_id.clone(), WorkerMetrics {
                worker_id,
                request_count: worker_count,
                request_distribution: worker_distribution_vec,
            });
        }

        Some(TestMetrics {
            session_id: session_id.to_string(),
            total_requests: session_requests.len(),
            first_request_time: first_time,
            last_request_time: last_time,
            requests_per_second: rps,
            request_distribution: distribution_vec,
            per_worker_metrics,
        })
    }
} 