use crate::db::schema::*;
use crate::db::NewOrder;
use crate::db::OrderModel;
use crate::AppState;
use actix_web::{get, patch, post, web, HttpResponse};
use serde_json::json;
use uuid::Uuid;

#[post("/add_order")]
pub async fn add_order(body: web::Json<NewOrder>, data: web::Data<AppState>) -> HttpResponse {
    let _query_result = sqlx::query(
        "INSERT INTO _menin.orders (
            deleted, number, order_type, title, discription, initiator, responsible_employee,
            deadline, closed, comment
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
    )
    .bind(body.deleted)
    .bind(body.number.clone())
    .bind(body.order_type as OrderType)
    .bind(body.title.clone())
    .bind(body.discription.clone())
    .bind(body.initiator.clone())
    .bind(body.responsible_employee.clone())
    .bind(body.deadline)
    .bind(body.closed)
    .bind(body.comment.clone())
    .execute(&data.db)
    .await;

    match _query_result {
        Ok(_) => println!("{} № {} добавлено.", body.order_type, body.number),
        Err(e) => println!(
            "Ошибка. Не удалось добавить {} № {}: {}",
            body.order_type, body.number, e
        ),
    }

    HttpResponse::Ok().body("Order added successfully")
}

#[get("/get_orders")]
pub async fn get_orders(data: web::Data<AppState>) -> web::Json<Vec<OrderModel>> {
    //https://stackoverflow.com/questions/76465657/how-do-i-create-custom-postgres-enum-types-in-rust-sqlx
    let orders = sqlx::query_file_as!(OrderModel, "src/db/sql/get_orders.sql")
        .fetch_all(&data.db)
        .await
        .unwrap();
    web::Json(orders)
}

#[patch("/delete_order")]
pub async fn delete_order(id: String, data: web::Data<AppState>) -> HttpResponse {
    let uuid_id: Uuid = Uuid::parse_str(id.as_str()).unwrap();
    let query_res = sqlx::query!(
        "update _menin.orders set deleted = true where id = $1",
        uuid_id
    )
    .execute(&data.db)
    .await
    .unwrap()
    .rows_affected();

    if query_res == 0 {
        let message = format!("Note with ID: {} not found", uuid_id);
        return HttpResponse::NotFound().json(json!({"status": "fail","message": message}));
    }

    HttpResponse::Ok().body("Order deleted successfully")
}
