use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow, Debug )]
pub struct Client {
    #[serde(rename = "limite")]
    pub currency_limit: i32,
    #[serde(rename = "saldo")]
    pub balance: Option<i64>,
    #[serde(rename = "valor")]
    pub value: Option<i32>,
    #[serde(rename = "tipo")]
    pub role: Option<String>,
    #[serde(rename = "descricao")]
    pub description: Option<String>,
    #[serde(rename = "realizada_em")]
    pub realized_at: Option<DateTime<Utc>>
}
