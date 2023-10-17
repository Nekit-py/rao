mod db;
// use crate::db::OrderModel;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use db::schema::*;
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

#[post("/add_order")]
async fn add_order(body: web::Json<NewOrder>, data: web::Data<AppState>) -> HttpResponse {
    let _query_result = sqlx::query(
        "INSERT INTO _menin.orders (
            deleted, number, order_type, title, initiator, responsible_employee,
            deadline, closed, comment
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)")
        .bind(body.deleted)
        .bind(body.number.clone())
        .bind(body.order_type as OrderType)
        .bind(body.title.clone())
        .bind(body.initiator.clone())
        .bind(body.responsible_employee.clone())
        .bind(body.deadline)
        .bind(body.closed)
        .bind(body.comment.clone())
    .execute(&data.db)
    .await;

    match _query_result {
        Ok(_) => println!("Row inserted"),
        Err(e) => println!("Error inserting row: {}", e),
    }

    HttpResponse::Ok().body("Ok, cool!")
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

pub struct AppState {
    db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    println!("🚀 Server started successfully");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .service(hello)
            .service(echo)
            .service(add_order)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

