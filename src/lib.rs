use actix_cors::Cors;
use actix_web::get;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use env_logger::Env;
use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

mod routes;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Limit Lens API",
        description = "A simple API for testing and visualizing rate limiters in real-time. Monitor request throughput and see how your rate limiting algorithms perform under load.",
        contact(name = "Limit Lens", url = "", email = "sharmaninenine@gmail.com"),
        version = "1.0.0"
    ),
    servers((url = "http://localhost:6969", description = "Local server")),
    paths(
        routes::health::health_check,
    ),
    components(
        schemas(
            
        )
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "Health", description = "Endpoint for checking the health of the service."),
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

pub fn main() -> std::io::Result<()> {
    actix_web::rt::System::new().block_on(async move {
        env_logger::init_from_env(Env::default().default_filter_or("info"));
        
        HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(Redoc::with_url("/redoc", ApiDoc::openapi()))
            .route("/", web::get().to(routes::health::health_check))
            .route("/health", web::get().to(routes::health::health_check))
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
