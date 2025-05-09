use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use utoipa::ToSchema;

use crate::storage::RateTestStorage;
use crate::models::{TestSession, TestMetrics};

/// Request for creating a new test session
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateSessionRequest {
    /// Optional name for the test session
    pub name: Option<String>,
}

/// Create a new test session
///
/// Returns a unique session ID that can be used for rate limit testing
#[utoipa::path(
    post,
    path = "/api/test/session",
    request_body = CreateSessionRequest,
    responses(
        (status = 200, description = "Session created successfully", body = TestSession),
        (status = 500, description = "Internal server error")
    ),
    tag = "Rate Test"
)]
pub async fn create_test_session(
    storage: web::Data<RateTestStorage>,
    session_req: web::Json<CreateSessionRequest>,
) -> impl Responder {
    let session = storage.create_session(session_req.name.clone());
    HttpResponse::Ok().json(session)
}

/// Receive a test request
///
/// Records a request for the specified session ID
#[utoipa::path(
    get,
    path = "/api/test/request/{session_id}",
    params(
        ("session_id" = String, Path, description = "The test session ID")
    ),
    responses(
        (status = 200, description = "Request recorded successfully"),
        (status = 404, description = "Session not found")
    ),
    tag = "Rate Test"
)]
pub async fn receive_test_request(
    storage: web::Data<RateTestStorage>,
    path: web::Path<String>,
) -> impl Responder {
    let session_id = path.into_inner();
    
    if storage.record_request(&session_id) {
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::NotFound().body("Session not found")
    }
}

/// Get metrics for a test session
///
/// Returns metrics and analysis of requests received for a session
#[utoipa::path(
    get,
    path = "/api/test/metrics/{session_id}",
    params(
        ("session_id" = String, Path, description = "The test session ID")
    ),
    responses(
        (status = 200, description = "Metrics retrieved successfully", body = TestMetrics),
        (status = 404, description = "Session not found")
    ),
    tag = "Rate Test"
)]
pub async fn get_test_metrics(
    storage: web::Data<RateTestStorage>,
    path: web::Path<String>,
) -> impl Responder {
    let session_id = path.into_inner();
    
    match storage.get_metrics(&session_id) {
        Some(metrics) => HttpResponse::Ok().json(metrics),
        None => HttpResponse::NotFound().body("Session not found"),
    }
}

/// Get all active test sessions
///
/// Returns a list of all active test sessions
#[utoipa::path(
    get,
    path = "/api/test/sessions",
    responses(
        (status = 200, description = "Active sessions retrieved successfully", body = Vec<TestSession>),
    ),
    tag = "Rate Test"
)]
pub async fn get_all_sessions(
    storage: web::Data<RateTestStorage>,
) -> impl Responder {
    let sessions = storage.get_all_sessions();
    HttpResponse::Ok().json(sessions)
}

/// Web interface for viewing live metrics
///
/// Serves an HTML page that displays live metrics for all active sessions
#[utoipa::path(
    get,
    path = "/dashboard",
    responses(
        (status = 200, description = "Dashboard HTML page")
    ),
    tag = "Rate Test"
)]
pub async fn metrics_dashboard() -> impl Responder {
    let html = include_str!("../../static/dashboard.html");
    HttpResponse::Ok().content_type("text/html").body(html)
} 