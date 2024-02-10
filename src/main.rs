// use actix_web::{get, web, App, HttpServer, Responder};
// use serde::{Deserialize, Serialize};
// use std::fs;

// #[derive(Debug, Serialize, Deserialize)]
// struct User {
//     id: u64,
//     name: String,
//     email: String,
//     phone: String,
// }

// #[get("/users")]
// async fn get_users() -> impl Responder {
//     let users = fs::read_to_string("./users.json")
//         .expect("Failed to read users.json file");
//     let users: Vec<User> = serde_json::from_str(&users)
//         .expect("Failed to parse users.json content");

//     web::Json(users)
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .service(get_users)
//     })
//     .bind("127.0.0.1:8080")?
//     .run()
//     .await
// }


use actix_web::{get, web, App, HttpServer, Responder};
use serde::{Serialize, Serializer};
use std::{fs, sync::Arc};
use lazy_static::lazy_static;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct User {
    id: u64,
    name: String,
    email: String,
    phone: String,
}

// Wrapper type around Arc<Vec<User>>
struct UsersWrapper(Arc<Vec<User>>);

// Implement Serialize for UsersWrapper
impl Serialize for UsersWrapper {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

lazy_static! {
    static ref USERS_JSON: Arc<Vec<User>> = {
        let users = fs::read_to_string("./users.json")
            .expect("Failed to read users.json file");
        Arc::new(serde_json::from_str(&users)
            .expect("Failed to parse users.json content"))
    };
}

#[get("/users")]
async fn get_users() -> impl Responder {
    web::Json(UsersWrapper(USERS_JSON.clone()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(get_users)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
