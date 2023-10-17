use crate::db::{OrderType, OrderStatus};
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
    pub initiator: String,
    pub responsible_employee: String,
    pub deadline: NaiveDate,
    pub order_status: OrderStatus,
    pub closed: Option<NaiveDateTime>,
    pub comment: Option<String>,
}
