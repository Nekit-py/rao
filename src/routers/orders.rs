use crate::db::schema::*;
use crate::db::NewOrder;
use crate::db::OrderModel;
use crate::AppState;
use actix_web::{get, patch, post, web, HttpResponse};
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
    //query_file
    let query_result = sqlx::query_as!(
        OrderModel,
        r#"SELECT
            id,
            number,
            deleted,
            created,
            updated,
            order_type as "order_type: OrderType",
            title,
            discription,
            initiator,
            responsible_employee,
            deadline,
            status as "status: OrderStatus",
            closed,
            comment
        FROM _menin.orders"#
    )
    .fetch_all(&data.db)
    .await
    .unwrap();
    web::Json(query_result)
}

#[patch("/delete_order")]
pub async fn delete_order(id: String) -> HttpResponse {
    let uuid_id: Uuid = Uuid::parse_str(id.as_str()).unwrap();
    // let query_res = sqlx::query_as("update _menin.orders set deleted = true where id = $1")
    println!("{uuid_id}");
    HttpResponse::Ok().body("Order deleted successfully")
}
