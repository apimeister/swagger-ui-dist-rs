use axum::{routing::get, Router};
#[cfg(feature = "axum-07")]
use axum_07 as axum;
#[cfg(feature = "axum-08")]
use axum_08 as axum;
use swagger_ui_dist::{ApiDefinition, OpenApiSource};

#[tokio::main]
async fn main() {
    let api_def = ApiDefinition {
        uri_prefix: "/api",
        api_definition: OpenApiSource::Uri("/openapi.yml"),
        title: Some("My Super Duper API"),
    };
    let app = Router::new()
        .route(
            "/openapi.yml",
            get(|| async move { include_str!("petstore.yaml") }),
        )
        .merge(swagger_ui_dist::generate_routes(api_def));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on http://localhost:3000/api");
    axum::serve(listener, app).await.unwrap();
}
