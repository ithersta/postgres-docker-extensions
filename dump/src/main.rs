use std::process::Command;

use actix_web::{App, get, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn dump() -> impl Responder {
    let dump = Command::new("pg_dump")
        .output()
        .expect("pg_dump failed");
    HttpResponse::Ok().body(dump.stdout)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(dump)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}