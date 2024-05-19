#![deny(missing_docs)]
//! swagger-ui-dist redistributes the swagger ui
//!
//! it repackages the JS/CSS code into axum routes
//! to allow for an easier implementation
//! 
//! ```rust
//! let api_def = ApiDefinition {
//!   uri_prefix: "/api",
//!   api_definition: OpenApiSource::Inline(include_str!("petstore.yaml")),
//!   title: Some("My Super Duper API"),
//! };
//! let app = Router::new().merge(swagger_ui_dist::generate_routes(api_def));
//! let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
//! println!("listening on http://localhost:3000/api");
//! axum::serve(listener, app).await.unwrap();
//! ```
use axum::{routing::get, Router};
use axum_core::{body::Body, extract::Request, response::Response};

async fn serve_index(api_def: String, title: String, req: Request) -> Response {
    let uri = req.uri();
    let response_str = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>{title}</title>
    <link rel="stylesheet" href="{uri}/swagger-ui.css" />
</head>
<body>
<div id="swagger-ui"></div>
<script src="{uri}/swagger-ui-bundle.js" crossorigin></script>
<script>
    window.onload = () => {{
    window.ui = SwaggerUIBundle({{
        url: '{api_def}',
        dom_id: '#swagger-ui',
    }});
    }};
</script>
</body>
</html>"#
    );
    Response::builder()
        .status(200)
        .header("Content-Type", "text/html")
        .body(Body::from(response_str))
        .unwrap()
}

async fn serve_js() -> Response {
    let js: &str = include_str!("../assets/swagger-ui-bundle.js");
    Response::builder()
        .status(200)
        .header("Content-Type", "text/javascript")
        .body(Body::from(js))
        .unwrap()
}

async fn serve_js_map() -> Response {
    let js: &str = include_str!("../assets/swagger-ui-bundle.js.map");
    Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(Body::from(js))
        .unwrap()
}

async fn serve_css() -> Response {
    let css: &str = include_str!("../assets/swagger-ui.css");
    Response::builder()
        .status(200)
        .header("Content-Type", "text/css")
        .body(Body::from(css))
        .unwrap()
}

async fn serve_css_map() -> Response {
    let js: &str = include_str!("../assets/swagger-ui.css.map");
    Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(Body::from(js))
        .unwrap()
}

/// Provide the OpenAPi Spec either Inline or as Url
#[derive(Debug, Clone)]
pub enum OpenApiSource<S: Into<String>> {
    /// generates the OpenAPI location at {uri_prefix}/openapi.yaml
    Inline(S),
    /// generates the OpenAPI location at the given URI
    InlineWithName {
        /// OpenAPI definition as String
        definition: S,
        /// OpenAPI URI that is used to expose the definition
        uri: S,
    },
    /// uses the given the OpenAPI location
    Uri(S),
}

/// Configuration for the API definition
#[derive(Debug, Clone)]
pub struct ApiDefinition<S: Into<String> + Clone> {
    /// URI prefix used for all Axum routes
    pub uri_prefix: S,
    /// OpenAPI definition given, either inline of as URL reference
    pub api_definition: OpenApiSource<S>,
    /// Optional title of the API, defaults to SwaggerUI
    pub title: Option<S>,
}

/// Generate the route for Axum depending on the given configuration
pub fn generate_routes<S: Into<String> + Clone>(def: ApiDefinition<S>) -> Router {
    let prefix = def.uri_prefix.into();
    let prefix2 = format!("{prefix}/");
    let def2 = def.api_definition.clone();
    let api_def = match def.api_definition {
        OpenApiSource::Uri(val) => val.into(),
        OpenApiSource::Inline(_val) => format!("{prefix}/openapi.yaml"),
        OpenApiSource::InlineWithName { definition: _, uri } => uri.into(),
    };
    let api_def2 = api_def.clone();
    let api_def3 = api_def.clone();
    let title = match def.title {
        Some(val) => val.into(),
        None => "SwaggerUI".to_string(),
    };
    let title2 = title.clone();
    let mut router = Router::new()
        .route(
            &prefix,
            get(|req: Request| async move { serve_index(api_def, title, req).await }),
        )
        .route(
            &prefix2,
            get(|req: Request| async move { serve_index(api_def2, title2, req).await }),
        )
        .route(&format!("{prefix}/swagger-ui.css"), get(serve_css))
        .route(&format!("{prefix}/swagger-ui-bundle.js"), get(serve_js))
        .route(&format!("{prefix}/swagger-ui.css.map"), get(serve_css_map))
        .route(
            &format!("{prefix}/swagger-ui-bundle.js.map"),
            get(serve_js_map),
        );
    if let OpenApiSource::Inline(source) = def2 {
        let yaml = source.into();
        router = router.route(&api_def3, get(|| async { yaml }));
    } else if let OpenApiSource::InlineWithName { definition, uri: _ } = def2 {
        let yaml = definition.into();
        router = router.route(&api_def3, get(|| async { yaml }));
    }
    router
}
