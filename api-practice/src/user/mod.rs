mod handlers;
mod models;

use crate::user::handlers::{add_user, delete_user, get_user_by_id, get_users};
use actix_web::web;

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .service(web::resource("/get-all").route(web::get().to(get_users)))
            .service(web::resource("/get/{id}").route(web::get().to(get_user_by_id)))
            .service(web::resource("/add").route(web::post().to(add_user)))
            .service(web::resource("/del/{id}").route(web::delete().to(delete_user))),
    );
}
