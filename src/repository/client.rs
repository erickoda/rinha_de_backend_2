use sqlx::PgPool;
use crate::schemas::client::Client;

#[derive(Clone)]
pub struct ClientRepository;   

impl ClientRepository {

    pub async fn find(pool: PgPool, id: i32) -> Result<Option<Client>, sqlx::Error> {
        sqlx::query_as(
            "SELECT * FROM client WHERE id = $1" //pegar cliente, pegar transctions ids no client
        )
        .bind(id)
        .fetch_optional(&pool)
        .await
    }
}