// use dotenvy::dotenv;
// use sqlx::PgPool;
// use tokio::time::error;
// use std::env;
// use std::net::SocketAddr;
// use serde::Deserialize;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use serde::{Deserialize, Serialize};
// use std::error::Error;

mod orders {
    // use sqlx::PgPool;
    // use uuid::Uuid;

    // use self::schemas::NewOrder;

    pub mod schemas {
        use chrono::{Duration, Local, NaiveDate};
        // use uuid::Uuid;
        use serde::{Deserialize, Serialize};

        #[derive(Serialize, Deserialize, Debug)]
        pub enum OrderType {
            Order(String),
            Disposal(String),
        }

        #[derive(Serialize, Deserialize, Debug)]
        pub enum Status {
            Completed(String),
            InProgress(String),
        }

        #[derive(Deserialize, Serialize, Debug)]
        pub struct NewOrder {
            pub deleted: bool,
            pub order_type: OrderType,
            pub title: String,
            pub initiator: String,
            pub responsible_employee: String,
            pub deadline: NaiveDate,
            pub status: Status,
            pub close_date: Option<NaiveDate>,
            pub comment: Option<String>,
        }

        impl Default for NewOrder {
            fn default() -> Self {
                let now = Local::now().with_timezone(&Local).date_naive();
                NewOrder {
                    deleted: false,
                    order_type: OrderType::Order("Приказ".to_string()),
                    title: "Тестовый приказ".to_string(),
                    initiator: "Федя Пупкин Амфибрахиевч".to_string(),
                    responsible_employee: "Козлов Опущенцев".to_string(),
                    deadline: now + Duration::days(3),
                    status: Status::InProgress("В работе".to_string()),
                    close_date: None,
                    comment: Some("-".to_string()),
                }
            }
        }

        // #[derive(Deserialize)]
        // pub struct Order {
        //     id: Option<Uuid>,
        //     number: u32,
        //     deleted: bool,
        //     created: NaiveDateTime,
        //     updated: NaiveDateTime,
        //     order_type: OrderType,
        //     title: String,
        //     initiator: String,
        //     responsible_employee: String,
        //     deadline: NaiveDate,
        //     status: Status,
        //     close_date: NaiveDate,
        //     comment: String
        // }
    }
}

// #[derive(Deserialize, Serialize, Debug, Clone)]
// struct User {
//     age: u8,
//     name: String,
// }

#[post("/add_order")]
async fn add_order(order: web::Json<orders::schemas::NewOrder>) -> HttpResponse {
    HttpResponse::Ok().json(order)
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(add_order)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

