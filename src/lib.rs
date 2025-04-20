use actix_cors::Cors;
use actix_web::get;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use env_logger::Env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

mod routes;
mod models;
mod storage;
#[cfg(test)]
mod tests;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Limit Lens API",
        description = "A simple API for testing and visualizing rate limiters in real-time. Monitor request throughput and see how your rate limiting algorithms perform under load.",
        contact(name = "Limit Lens", url = "", email = "sharmaninenine@gmail.com"),
        version = env!("CARGO_PKG_VERSION")
    ),
    servers((url = "http://localhost:6969", description = "Local server")),
    paths(
        routes::health::health_check,
        routes::rate_test::create_test_session,
        routes::rate_test::receive_test_request,
        routes::rate_test::get_test_metrics,
        routes::rate_test::get_all_sessions,
        routes::rate_test::metrics_dashboard,
    ),
    components(
        schemas(
            models::TestSession,
            models::TestMetrics,
        )
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "Health", description = "Endpoint for checking the health of the service."),
        (name = "Rate Test", description = "Endpoints for testing rate limiting algorithms."),
    )
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi
            .components
            .as_mut()
            .expect("Safe to expect since the component was already registered");
        components.add_security_scheme(
            "api_key",
            SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("Authorization"))),
        );
    }
}

#[get("/openapi.json")]
pub async fn get_openapi_spec_handler() -> impl actix_web::Responder {
    web::Json(ApiDoc::openapi())
}

pub fn generate_and_save_openapi_spec(output_path: &str) -> std::io::Result<()> {
    let spec = serde_json::to_string_pretty(&ApiDoc::openapi())?;
    let mut file = File::create(Path::new(output_path))?;
    file.write_all(spec.as_bytes())?;
    Ok(())
}

pub fn main() -> std::io::Result<()> {
    actix_web::rt::System::new().block_on(async move {
        env_logger::init_from_env(Env::default().default_filter_or("info"));
        
        let rate_test_data = web::Data::new(storage::RateTestStorage::new());
        
        HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(rate_test_data.clone())
            .service(Redoc::with_url("/redoc", ApiDoc::openapi()))
            .route("/", web::get().to(routes::health::health_check))
            .route("/health", web::get().to(routes::health::health_check))
            .route("/dashboard", web::get().to(routes::rate_test::metrics_dashboard))
            .service(
                web::scope("/api/test")
                    .route("/session", web::post().to(routes::rate_test::create_test_session))
                    .route("/sessions", web::get().to(routes::rate_test::get_all_sessions))
                    .route("/request/{session_id}", web::get().to(routes::rate_test::receive_test_request))
                    .route("/metrics/{session_id}", web::get().to(routes::rate_test::get_test_metrics))
            )
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/docs/openapi.json", ApiDoc::openapi()),
            )
        })
        .bind("0.0.0.0:6969")?
        .run()
        .await
    })
}
