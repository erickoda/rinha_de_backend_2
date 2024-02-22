use actix_web::{web::{Data, Path}, Error, HttpResponse};

use crate::{repository::client::ClientRepository, AppData};

pub struct ClientController;

impl ClientController {
    pub async fn find(repository: Data<AppData>, path: Path<i32>) -> Result<HttpResponse, Error> {
        let id = path.into_inner();
        match ClientRepository::find(repository.pool.clone(), id).await {
            Ok(founded_client) => match founded_client {
                Some(client) => Ok(HttpResponse::Found().json(client)),
                None => Ok(HttpResponse::NotFound().json(""))
            },
            Err(_) => Ok(HttpResponse::InternalServerError().json("Internal Server Error"))
        }
    }
}