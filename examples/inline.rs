use axum::Router;
use swagger_ui_dist::{ApiDefinition, OpenApiSource};

#[tokio::main]
async fn main() {
    let api_def = ApiDefinition {
        uri_prefix: "/api",
        api_definition: OpenApiSource::Inline(include_str!("petstore.yaml")),
        title: Some("My Super Duper API"),
    };
    let app = Router::new().merge(swagger_ui_dist::generate_routes(api_def));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on http://localhost:3000/api");
    axum::serve(listener, app).await.unwrap();
}
