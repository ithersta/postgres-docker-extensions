use std::process::Command;

use actix_web::{App, get, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn dump() -> impl Responder {
    let dump = Command::new("pg_dump")
        .args(["-U", "postgres"])
        .output()
        .expect("pg_dump failed");
    HttpResponse::Ok().body(dump.stdout)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(dump)
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}