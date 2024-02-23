use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::Type, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum TransactionRoles {
    C,
    D
}

impl TransactionRoles {
    pub fn as_str(&self) -> String {
        match self {
            TransactionRoles::C => String::from("c"),
            TransactionRoles::D => String::from("d")
        }
    }
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Transaction {
    #[serde(rename = "valor")]
    pub value: i32,
    #[serde(rename = "tipo")]
    pub role: String,
    #[serde(rename = "descricao")]
    pub description: String,
    #[serde(rename = "realizada_em")]
    pub realized_at: DateTime<Utc>,
    // client_id: i32
}

#[derive(Serialize, Deserialize, sqlx::FromRow, Debug, Clone)]
pub struct NewTransaction {
    #[serde(rename = "valor")]
    pub value: i32,
    #[serde(rename = "tipo")]
    pub role: TransactionRoles,
    #[serde(rename = "descricao")]
    pub description: String
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct ClientLimitAndBalance {
    #[serde(rename = "limite")]
    pub limit: i32,
    #[serde(rename = "saldo")]
    pub balance: i64
}