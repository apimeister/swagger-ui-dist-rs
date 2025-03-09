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

    println!("listening on http://localhost:8080/api");

    HttpServer::new(move || {
        App::new()
            .service(web::scope("/").route("", web::get().to(hello)))
            .service(generate_scope(api_def.clone()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
