use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow, Debug )]
pub struct Client {
    pub id: i32,
    #[serde(rename = "limite")]
    pub currency_limit: i32,
    #[serde(rename = "saldo")]
    pub balance: i32,
    transactions: Vec<i32>
}
