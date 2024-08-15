use actix_web::{dev::Server, get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde;
use serde_json;
use std::net::TcpListener;

pub async fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            // .service(json)
            // .route("/{name}", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}

async fn health_check(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish()
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[get("/json")]
async fn json(_req: HttpRequest) -> impl Responder {
    let json = serde_json::json!({
        "status": "ok"
    });
    //    返回json格式数据
    HttpResponse::Ok()
        .append_header(("Content-Type", "application/json"))
        .json(json)
}

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String,
}

async fn subscribe(_form: web::Form<FormData>) -> impl Responder {
    HttpResponse::Ok().finish()
}
