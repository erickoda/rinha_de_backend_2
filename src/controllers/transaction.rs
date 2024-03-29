use actix_web::{web::{Data, Json, Path}, HttpResponse};

use crate::{repository::transaction::TransactionRepository, schemas::transaction::{NewTransaction, TransactionRoles}, AppData};



pub struct TransactionController;

impl TransactionController {
    pub async fn create(repository: Data<AppData>, request_transaction: Json<NewTransaction>, client_id: Path<i32>) -> HttpResponse {
        match &request_transaction.role {
            TransactionRoles::D => {
                match TransactionRepository::is_debit_operation_valid(repository.pool.clone(), *client_id, request_transaction.value).await {
                    true => {},
                    false => return HttpResponse::UnprocessableEntity().json("Metodo invalido")
                }
            },
            TransactionRoles::C => {}
        }

        match TransactionRepository::create(repository.pool.clone(), request_transaction, client_id.into_inner()).await {
            Ok(new_transaction) => {
                match new_transaction {
                    Some(transaction) => HttpResponse::Ok().json(transaction),
                    None => HttpResponse::UnprocessableEntity().json("Erro ao Criar")
                }
            }
            Err(err) => HttpResponse::InternalServerError().json(err.to_string())
        }
    }
}