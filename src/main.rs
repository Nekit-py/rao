use anyhow::Context;
use dotenvy::dotenv;
use sqlx::PgPool;
use std::env;
use std::net::SocketAddr;
use serde::Deserialize;
use axum::{
    {routing::{get, post}, Router},
    extract::{Query, State},
    response::{IntoResponse, Response},
    http::StatusCode,
    Json
};

mod orders {
    use sqlx::PgPool;
    use uuid::Uuid;

    pub mod schemas {
        use chrono::{NaiveDate, NaiveDateTime, Local, Duration};
        use uuid::Uuid;
        use serde::{Deserialize, Serialize};


        #[derive(Serialize, Deserialize, Debug)]
        pub enum OrderType{
            Order(String),
            Disposal(String)
        }

        #[derive(Serialize, Deserialize, Debug)]
        pub enum Status{
            Completed(String),
            InProgress(String)
        }

        #[derive(Serialize, Debug)]
        pub struct NewOrder {
            pub deleted: bool,
            pub order_type: OrderType,
            pub title: String,
            pub initiator: String,
            pub responsible_employee: String,
            pub deadline: NaiveDate,
            pub status: Status,
            pub close_date: NaiveDate,
            pub comment: String
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
                    close_date: now,
                    comment: "-".to_string()
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

    pub async fn get_order(pool: &PgPool, id: Uuid) -> String {
        "New order added".to_string()
    }

    pub async fn index() -> String {
        "Rusty accounting of orders index page".to_string()
    }
}


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use crate::orders;

    dotenv().ok();
    // let db_connection_str = env::var("DATABASE_URL").context("DATABASE_URL must be set")?;
	// let pool = sqlx::PgPool::connect(&db_connection_str)
    // 	.await
    // 	.context("can't connect to database")?;

	// let app = Router::new()
    // 	.route("/", get(orders::index))
    // 	.route("/order", get(orders::get_order))
	// 	.with_state(pool);

	// let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
	// axum::Server::bind(&addr)
    // 	.serve(app.into_make_service())
    // 	.await?;

    let new_order = orders::schemas::NewOrder::default();
    println!("{:#?}", new_order);
    Ok(())
}