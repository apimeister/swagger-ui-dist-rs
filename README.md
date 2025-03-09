[![Latest Version](https://img.shields.io/crates/v/swagger-ui-dist.svg)](https://crates.io/crates/swagger-ui-dist)

The version number reflects the swagger-ui version embedded.

## Usage

This crate can either be used with `axum` or `axtix`. You can enbale/disable the implementations through feature flags.

**For Axum 0.8 (which is the default)**

Cargo.toml
```toml
[dependencies]
swagger-ui-dist = "*"
```

**For Axum 0.7**

Cargo.toml
```toml
[dependencies]
swagger-ui-dist = { version = "*", default-features = false, features = ["with-axum-07"] }
```

**For Actix**

Cargo.toml
```toml
[dependencies]
swagger-ui-dist = { version = "*", default-features = false, features = ["with-actix"] }
```

## Implementation

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

## Actix Sample use

```rust
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use swagger_ui_dist::{generate_scope, ApiDefinition, OpenApiSource};

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let api_def = ApiDefinition {
        uri_prefix: "/api".to_string(),
        api_definition: OpenApiSource::Inline(include_str!("petstore.yaml").to_string()),
        title: Some("My Super Duper API".to_string()),
    };

    println!("listening on http://localhost:8080/api/");

    HttpServer::new(move || {
        App::new()
            .service(web::scope("/").route("", web::get().to(hello)))
            .service(generate_scope(api_def.clone()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```

## Examples

More example are available through the `examples` in the repository.