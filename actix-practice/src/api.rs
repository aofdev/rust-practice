use actix_web::web::ServiceConfig;
use actix_web::{web, HttpResponse};

use crate::{constants, db::DB, dto::BookRequest, dto::ResponseBody, services::book_service};

async fn books(db: web::Data<DB>) -> HttpResponse {
    match book_service::books_list(&db).await {
        Ok(message) => HttpResponse::Ok().json(ResponseBody::new(constants::EMPTY, &message)),
        Err(err) => err.response(),
    }
}

async fn create_book(db: web::Data<DB>, body: web::Json<BookRequest>) -> HttpResponse {
    match book_service::create_book(&db, body.0).await {
        Ok(message) => HttpResponse::Ok().json(ResponseBody::new(&message, constants::EMPTY)),
        Err(err) => err.response(),
    }
}

pub fn config_services(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/books").service(
            web::resource("")
                .route(web::get().to(books))
                .route(web::post().to(create_book)),
        ),
    );
}
