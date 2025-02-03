[![Latest Version](https://img.shields.io/crates/v/swagger-ui-dist.svg)](https://crates.io/crates/swagger-ui-dist)

The version number reflects the swagger-ui version embedded.

## Usage

### With Inline OpenAPI

```rust
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
```

### With external Route

```rust
use axum::{routing::get, Router};
use swagger_ui_dist::{ApiDefinition, OpenApiSource};

#[tokio::main]
async fn main() {
    let api_def = ApiDefinition {
        uri_prefix: "/api",
        api_definition: OpenApiSource::Uri("/openapi.yml"),
        title: Some("My Super Duper API"),
    };
    let app = Router::new()
        .route("/openapi.yml", get(|| async move { include_str!("petstore.yaml") }))
        .merge(swagger_ui_dist::generate_routes(api_def));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on http://localhost:3000/api");
    axum::serve(listener, app).await.unwrap();
}
```

### Supporting axum 0.7 and 0.8

Since axum 0.8 has breaking changes, this crate supports both axum 0.7 and 0.8. By default, the crate uses the latest axum.

To use axum 0.7, add the following to your `Cargo.toml`:

```toml
[dependencies]
swagger-ui-dist = { version = "5.18.2", default-features = false, features = ["with-axum-07"] }
```

To use axum 0.8, add the following to your `Cargo.toml`:

```toml
[dependencies]
swagger-ui-dist = { version = "5.18.2" }
```

