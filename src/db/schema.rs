use chrono::{Duration, Local, NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, sqlx::Type, Deserialize, Serialize)]
#[sqlx(rename_all = "lowercase", type_name = "order_type")]
pub enum OrderType {
    Order,
    Disposal,
}

impl Display for OrderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Order => "order",
            Self::Disposal => "disposal",
        })
    }
}

// impl OrderType {
//     pub fn to_string(&self) -> String {
//         match self {
//             OrderType::Order => "Приказ".to_string(),
//             OrderType::Disposal => "Распоряжение".to_string(),
//         }
//     }
// }
//
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, sqlx::Type, Deserialize, Serialize)]
#[sqlx(rename_all = "lowercase", type_name = "order_status")]
pub enum OrderStatus {
    Completed,
    InProgress,
}

impl Display for OrderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Completed => "completed",
            Self::InProgress => "in_progress",
        })
    }
}

// impl OrderStatus {
//     pub fn to_string(&self) -> String {
//         match self {
//             OrderStatus::Completed => "Закрыт".to_string(),
//             OrderStatus::InProgress => "На исполнении".to_string(),
//         }
//     }
// }
//
#[derive(Deserialize, Serialize, Debug)]
pub struct NewOrder {
    pub deleted: bool,
    pub number: String,
    pub order_type: OrderType,
    pub title: String,
    pub discription: String,
    pub initiator: String,
    pub responsible_employee: String,
    pub deadline: NaiveDate,
    pub closed: Option<NaiveDateTime>,
    pub comment: Option<String>,
}

impl Default for NewOrder {
    fn default() -> Self {
        let now = Local::now().with_timezone(&Local).date_naive();
        NewOrder {
            deleted: false,
            number: "1tst".to_string(),
            order_type: OrderType::Order,
            title: "Тестовый приказ".to_string(),
            discription: "Описание приказа".to_string(),
            initiator: "Федя Пупкин Амфибрахиевч".to_string(),
            responsible_employee: "Козлов Опущенцев".to_string(),
            deadline: now + Duration::days(3),
            closed: None,
            comment: Some("-".to_string()),
        }
    }
}
