use chrono::{Duration, Local, NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, sqlx::Type, Deserialize, Serialize)]
#[sqlx(rename_all = "lowercase", type_name = "order_type")]
pub enum OrderType {
    Order,
    Disposal,
}

impl OrderType {
    fn to_string(&self) -> String {
        match self {
            OrderType::Order => "Приказ".to_string(),
            OrderType::Disposal => "Распоряжение".to_string(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, sqlx::Type, Deserialize, Serialize)]
#[sqlx(rename_all = "lowercase", type_name = "status")]
pub enum Status {
    Completed,
    InProgress,
}

impl Status {
    fn to_string(&self) -> String {
        match self {
            Status::Completed => "Закрыт".to_string(),
            Status::InProgress => "На исполнении".to_string(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NewOrder {
    pub deleted: bool,
    pub number: String,
    pub order_type: OrderType,
    pub title: String,
    pub initiator: String,
    pub responsible_employee: String,
    pub deadline: NaiveDate,
    pub status: Status,
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
            initiator: "Федя Пупкин Амфибрахиевч".to_string(),
            responsible_employee: "Козлов Опущенцев".to_string(),
            deadline: now + Duration::days(3),
            status: Status::InProgress,
            closed: None,
            comment: Some("-".to_string()),
        }
    }
}
