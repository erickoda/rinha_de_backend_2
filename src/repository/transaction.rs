use actix_web::web::Json;
use sqlx::PgPool;
use crate::schemas::transaction::{ClientLimitAndBalance, NewTransaction};

#[derive(Clone)]
pub struct TransactionRepository;

impl TransactionRepository {
    pub async fn create(pool: PgPool, new_transaction: Json<NewTransaction>, client_id: i32) -> Result<Option<ClientLimitAndBalance>, sqlx::Error> {
        sqlx::query_as(
            "
            WITH inserted_transaction AS (
                INSERT INTO
                transaction (value, role, description, realized_at, client_id)
                VALUES ($1, $2, $3, NOW(), $4)
                RETURNING id, value, role, description, realized_at, client_id
            ), updated_client AS (
                UPDATE client
                SET transactions = array_append(transactions, (SELECT id FROM inserted_transaction))
                WHERE id = $4
                RETURNING currency_limit
            ), client_transactions AS (
                SELECT SUM(value) as balance FROM transaction
                WHERE client_id = $4
            )
            SELECT -(COALESCE(balance, 0) + $1) as balance, currency_limit as limit FROM client_transactions, updated_client
            "
        )
        .bind(new_transaction.value)
        .bind(new_transaction.role.as_str())
        .bind(new_transaction.description.clone())
        .bind(client_id)
        .fetch_optional(&pool)
        .await
    }

    pub async fn is_debit_operation_valid(pool: PgPool, client_id: i32, value: i32) -> bool {

        let result: Result<ClientLimitAndBalance, sqlx::Error> = sqlx::query_as(
            "
            WITH selected_client as (
                SELECT currency_limit FROM client WHERE id = $1
            ), transactions_value as (
                SELECT SUM(value) as balance FROM transaction WHERE client_id = $1
            )
            
            SELECT -(COALESCE(balance, 0) + $2) as balance, currency_limit as limit FROM selected_client, transactions_value
            "
        )
        .bind(client_id)
        .bind(value)
        .fetch_one(&pool)
        .await;

        match result {
            Ok(transaction) => {
                println!("limit: {}, balance: {}", transaction.limit, -transaction.balance);
                if (transaction.limit as i64) < (-transaction.balance) {
                    return false
                }
                true
            }
            Err(_) => {
                false
            }
        }
    }
}