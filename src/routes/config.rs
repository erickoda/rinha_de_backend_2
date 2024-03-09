use actix_web::web::{get, post, resource, scope, ServiceConfig};

use crate::controllers::{client::ClientController, transaction::TransactionController};

pub fn services_config(service_config: &mut ServiceConfig) {
    service_config
        .service(scope("/clientes/{id}")
            .service(resource("/extrato").route(get().to(ClientController::find)))
            .service(resource("/transacoes").route(post().to(TransactionController::create)))
        );
}
