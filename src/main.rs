use actix_web::{get, web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: u64,
    name: String,
    email: String,
    phone: String,
}

#[get("/users")]
async fn get_users() -> impl Responder {
    let users = fs::read_to_string("./users.json")
        .expect("Failed to read users.json file");
    let users: Vec<User> = serde_json::from_str(&users)
        .expect("Failed to parse users.json content");

    web::Json(users)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_users)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}