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

#[cfg(feature = "actix-web")]
use actix_web::{dev::HttpServiceFactory, web, HttpRequest, HttpResponse, Responder};
#[cfg(any(feature = "axum-07", feature = "axum-08"))]
use axum::{http::header, routing::get, Router};
#[cfg(feature = "axum-07")]
use axum_07 as axum;
#[cfg(feature = "axum-08")]
use axum_08 as axum;
#[cfg(any(feature = "axum-07", feature = "axum-08"))]
use axum_core::{body::Body, extract::Request, response::Response};
#[cfg(feature = "axum-07")]
use axum_core_04 as axum_core;
#[cfg(feature = "axum-08")]
use axum_core_05 as axum_core;

#[cfg(any(feature = "axum-07", feature = "axum-08"))]
async fn serve_index_axum(api_def: String, title: String, req: Request) -> Response {
    let uri = req.uri().to_string();

    let response_str = serve_index(api_def, title, uri);

    Response::builder()
        .status(200)
        .header(header::CONTENT_TYPE, "text/html")
        .body(Body::from(response_str))
        .unwrap()
}

#[cfg(feature = "actix-web")]
async fn serve_index_actix(api_def: String, title: String, req: HttpRequest) -> impl Responder {
    let uri = req.uri().to_string();
    let uri = if uri.ends_with("/") {
        uri.trim_end_matches("/").to_string()
    } else {
        uri
    };

    let response_str = serve_index(api_def, title, uri);

    HttpResponse::Ok()
        .content_type("text/html")
        .body(response_str)
}

fn serve_index(api_def: String, title: String, uri: String) -> String {
    format!(
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
    )
}

#[cfg(any(feature = "axum-07", feature = "axum-08"))]
async fn serve_js_axum() -> Response {
    let js: &str = include_str!("../assets/swagger-ui-bundle.js");
    Response::builder()
        .status(200)
        .header("Content-Type", "text/javascript")
        .body(Body::from(js))
        .unwrap()
}

#[cfg(feature = "actix-web")]
async fn serve_js_actix() -> impl Responder {
    let js: &str = include_str!("../assets/swagger-ui-bundle.js");
    HttpResponse::Ok().content_type("text/javascript").body(js)
}

#[cfg(any(feature = "axum-07", feature = "axum-08"))]
async fn serve_js_map_axum() -> Response {
    let js: &str = include_str!("../assets/swagger-ui-bundle.js.map");
    Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(Body::from(js))
        .unwrap()
}

#[cfg(feature = "actix-web")]
async fn serve_js_map_actix() -> impl Responder {
    let js: &str = include_str!("../assets/swagger-ui-bundle.js.map");
    HttpResponse::Ok().content_type("application/json").body(js)
}

#[cfg(any(feature = "axum-07", feature = "axum-08"))]
async fn serve_css_axum() -> Response {
    let css: &str = include_str!("../assets/swagger-ui.css");
    Response::builder()
        .status(200)
        .header("Content-Type", "text/css")
        .body(Body::from(css))
        .unwrap()
}

#[cfg(feature = "actix-web")]
async fn serve_css_actix() -> impl Responder {
    let css: &str = include_str!("../assets/swagger-ui.css");
    HttpResponse::Ok().content_type("text/css").body(css)
}

#[cfg(any(feature = "axum-07", feature = "axum-08"))]
async fn serve_css_map_axum() -> Response {
    let js: &str = include_str!("../assets/swagger-ui.css.map");
    Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(Body::from(js))
        .unwrap()
}

#[cfg(feature = "actix-web")]
async fn serve_css_map_actix() -> impl Responder {
    let js: &str = include_str!("../assets/swagger-ui.css.map");
    HttpResponse::Ok().content_type("application/json").body(js)
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
#[cfg(any(feature = "axum-07", feature = "axum-08"))]
pub fn generate_routes<S: Into<String> + Clone>(def: ApiDefinition<S>) -> Router {
    let prefix = def.uri_prefix.into();
    let prefix2 = format!("{prefix}/");
    let def2 = def.api_definition.clone();
    let api_def_uri = match def.api_definition {
        OpenApiSource::Uri(val) => val.into(),
        OpenApiSource::Inline(_val) => format!("{prefix}/openapi.yaml"),
        OpenApiSource::InlineWithName { definition: _, uri } => uri.into(),
    };
    let api_def2 = api_def_uri.clone();
    let api_def3 = api_def_uri.clone();
    let title = match def.title {
        Some(val) => val.into(),
        None => "SwaggerUI".to_string(),
    };
    let title2 = title.clone();
    let mut router = Router::new()
        .route(
            &prefix,
            get(|req: Request| async move { serve_index_axum(api_def_uri, title, req).await }),
        )
        .route(
            &prefix2,
            get(|req: Request| async move { serve_index_axum(api_def2, title2, req).await }),
        )
        .route(&format!("{prefix}/swagger-ui.css"), get(serve_css_axum))
        .route(
            &format!("{prefix}/swagger-ui-bundle.js"),
            get(serve_js_axum),
        )
        .route(
            &format!("{prefix}/swagger-ui.css.map"),
            get(serve_css_map_axum),
        )
        .route(
            &format!("{prefix}/swagger-ui-bundle.js.map"),
            get(serve_js_map_axum),
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

/// Generate a scope for the route for Actix depending on the given configuration
#[cfg(feature = "actix-web")]
pub fn generate_scope<S: Into<String> + Clone>(def: ApiDefinition<S>) -> impl HttpServiceFactory {
    let prefix = def.uri_prefix.into();
    let (uri, yaml) = match def.api_definition {
        OpenApiSource::Uri(val) => (val.into(), "".to_string()),
        OpenApiSource::Inline(val) => (format!("{prefix}/openapi.yaml"), val.into()),
        OpenApiSource::InlineWithName { definition, uri } => (uri.into(), definition.into()),
    };
    let title = match def.title {
        Some(val) => val.into(),
        None => "SwaggerUI".to_string(),
    };
    let source = ApiDefinition::<String> {
        uri_prefix: prefix.clone(),
        api_definition: OpenApiSource::InlineWithName {
            definition: yaml,
            uri: uri.clone(),
        },
        title: Some(title),
    };
    web::scope(&prefix)
        .app_data(web::Data::new(source))
        .route(
            "/",
            web::get().to(
                |req: HttpRequest, data: web::Data<ApiDefinition<String>>| async move {
                    let uri = match data.api_definition.clone() {
                        OpenApiSource::InlineWithName { definition: _, uri } => uri,
                        _ => "".to_string(),
                    };
                    serve_index_actix(uri, data.title.clone().unwrap(), req).await
                },
            ),
        )
        .route("/swagger-ui.css", web::get().to(serve_css_actix))
        .route("/swagger-ui-bundle.js", web::get().to(serve_js_actix))
        .route("/swagger-ui.css.map", web::get().to(serve_css_map_actix))
        .route(
            "/swagger-ui-bundle.js.map",
            web::get().to(serve_js_map_actix),
        )
        .route(
            uri.trim_start_matches(prefix.as_str()),
            web::get().to(|data: web::Data<ApiDefinition<String>>| async move {
                let yaml = match data.api_definition.clone() {
                    OpenApiSource::InlineWithName { definition, uri: _ } => definition,
                    _ => "".to_string(),
                };
                HttpResponse::Ok().body(yaml)
            }),
        )
}
