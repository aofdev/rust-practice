use actix_web::http::StatusCode;
use actix_web::web;

use crate::{db::DB, dto::BookRequest, error::ServiceError, models::Book};

pub async fn books_list(db: &web::Data<DB>) -> Result<Vec<Book>, ServiceError> {
    match db.fetch_books().await {
        Ok(result) => Ok(result),
        Err(_) => Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "could not fetch books from database".to_string(),
        )),
    }
}

pub async fn create_book(db: &web::Data<DB>, body: BookRequest) -> Result<String, ServiceError> {
    match db.create_book(&body).await {
        Ok(_) => Ok("book created".to_string()),
        Err(err) => Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            err.to_string(),
        )),
    }
}
