use actix_web::{web::{Data, Path}, Error, HttpResponse};
use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::{repository::client::ClientRepository, AppData};
use crate::schemas::transaction::Transaction;

pub struct ClientController;

impl ClientController {
    pub async fn find(repository: Data<AppData>, path: Path<i32>) -> Result<HttpResponse, Error> {
        let id = path.into_inner();
        match ClientRepository::find(repository.pool.clone(), id).await {
            Ok(clients) => {

                if clients.len() == 0 {
                    return Ok(HttpResponse::NotFound().json("Cliente Não Encontrado"))
                }

                let utc = Utc::now();
                let mut transactions : Vec<Transaction> = Vec::new();

                for client in &clients {
                    if client.value.is_some() && client.role.is_some() && client.description.is_some() && client.realized_at.is_some()  {
                        transactions.push(Transaction {
                            value: client.value.clone().unwrap(),
                            role: client.role.clone().unwrap(),
                            description: client.description.clone().unwrap(),
                            realized_at: client.realized_at.clone().unwrap()
                        })
                    } else {
                        transactions = vec![];
                    }
                }

                let response = ExtractResponse {
                    balance: Balance {
                        total: clients[0].balance.unwrap_or(0),
                        extract_date: utc,
                        currency_limit: clients[0].currency_limit
                    },
                    last_transactions: transactions
                };

                return Ok(HttpResponse::Ok().json(response))
            },
            Err(err) => Ok(HttpResponse::InternalServerError().json(err.to_string()))
        }
    }
}

#[derive(Serialize)]
struct ExtractResponse {
    #[serde(rename = "saldo")]
    balance: Balance,
    #[serde(rename = "ultimas_transacoes")]
    last_transactions: Vec<Transaction>
}

#[derive(Serialize)]
struct Balance {
    total: i64,
    #[serde(rename = "data_extrato")]
    extract_date: DateTime<Utc>,
    #[serde(rename="limite")]
    currency_limit: i32
}