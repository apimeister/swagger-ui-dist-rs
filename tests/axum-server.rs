use axum::Router;
use swagger_ui_dist::{ApiDefinition, OpenApiSource};

#[tokio::test]
async fn run_server() {
    let api_def = ApiDefinition {
        uri_prefix: "/api",
        api_definition: OpenApiSource::Uri("/openapi.yml"),
        title: Some("My Super Duper API"),
    };
    let app = Router::new().merge(swagger_ui_dist::generate_routes(api_def));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    // tokio::spawn(async move{
    axum::serve(listener, app).await.unwrap();
    // });
}
