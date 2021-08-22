use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse};
use serde_derive::{ Deserialize, Serialize};
use serde_json::{Value, Map};
use struct2vec::ToVec;
use struct2vec_derive::ToVec;
use std::collections::HashMap;
use from_value_derive::From;

#[derive(ToVec, Debug, Clone, Deserialize, Serialize)]
pub struct User {
    #[to_vec(comment = "ID")]
    id: usize,
    #[to_vec(comment = "用户名")]
    name: String,
    active: bool,
    array: Vec<String>,
    st: Vec<Custom>,
    ts: Custom1,
}

#[derive(ToVec, From, Debug, Clone, Deserialize, Serialize)]
pub struct Custom {
    name: String,
}

#[derive(ToVec, From, Debug, Clone, Deserialize, Serialize)]
pub struct Custom1 {
    name: String,
}

async fn greet(_req: HttpRequest) -> impl Responder {
    let st = Custom{name:String::from("123")};
    let name = User{id: 1, name: String::from("123"), active: true, array: vec![String::from("321")], st: vec![st], ts: Custom1{name: String::from("rrrrrrr")}};
    HttpResponse::Ok().json(name.to_vec())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}