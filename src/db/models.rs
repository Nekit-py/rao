use crate::db::{OrderStatus, OrderType};
use actix_web::{body::BoxBody, http::header::ContentType, HttpRequest, HttpResponse, Responder};
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct OrderModel {
    pub id: Uuid,
    pub number: String,
    pub deleted: bool,
    pub created: NaiveDateTime,
    pub updated: Option<NaiveDateTime>,
    pub order_type: OrderType,
    pub title: String,
    pub discription: String,
    pub initiator: String,
    pub responsible_employee: String,
    pub deadline: NaiveDate,
    pub status: OrderStatus,
    pub closed: Option<NaiveDateTime>,
    pub comment: Option<String>,
}

// Responder
impl Responder for OrderModel {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}
