use sqlx::PgPool;
use crate::schemas::client::Client;

#[derive(Clone)]
pub struct ClientRepository;   

impl ClientRepository {

    pub async fn find(pool: PgPool, id: i32) -> Result<Vec<Client>, sqlx::Error> {
        sqlx::query_as(
            "
            WITH all_transactions AS (
                SELECT c.currency_limit,
                       SUM(t.value) AS total_value
                FROM client c
                LEFT JOIN LATERAL (
                    SELECT unnest(c.transactions) AS transaction_id
                ) AS transaction_ids ON true
                LEFT JOIN transaction t ON transaction_ids.transaction_id = t.id
                WHERE c.id = $1
                GROUP BY c.currency_limit
            )
            SELECT at.currency_limit,
                   at.total_value AS balance,
                   t.value,
                   t.role,
                   t.description,
                   t.realized_at
            FROM all_transactions at
            LEFT JOIN LATERAL (
                SELECT unnest(array(SELECT unnest(ct.transactions) ORDER BY 1 DESC LIMIT 10)) AS transaction_id
                FROM client ct
                WHERE ct.id = $1
            ) AS transaction_ids ON true
            LEFT JOIN transaction t ON transaction_ids.transaction_id = t.id;
            "
        )
        .bind(id)
        .fetch_all(&pool)
        .await
    }
}