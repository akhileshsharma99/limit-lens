use actix_web::{web, HttpResponse, Responder};
use crate::models::{CreateSessionRequest, TestMetrics, TestRequestData, TestSession};
use crate::storage::RateTestStorage;

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
    request_data: web::Json<TestRequestData>,
) -> impl Responder {
    let session_id = path.into_inner();
    
    if storage.record_request(&session_id, &request_data.worker_id) {
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

