use anyhow::Context;
use dotenvy::dotenv;
use sqlx::PgPool;
use std::env;
use std::net::SocketAddr;
use serde::Deserialize;
use axum::{
    {routing::get, Router},
    extract::{Query, State},
    response::{IntoResponse, Response},
    http::StatusCode,
};

mod orders {

    pub async fn get_order() -> String {
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
    let db_connection_str = env::var("DATABASE_URL").context("DATABASE_URL must be set")?;
	let pool = sqlx::PgPool::connect(&db_connection_str)
    	.await
    	.context("can't connect to database")?;

	let app = Router::new()
    	.route("/", get(orders::index))
    	.route("/order", get(orders::get_order))
		.with_state(pool);

	let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
	axum::Server::bind(&addr)
    	.serve(app.into_make_service())
    	.await?;
    Ok(())
}