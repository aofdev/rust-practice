use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use crate::schema::users::dsl::*;
use crate::user_rpc::models::User;
use crate::Pool;
use diesel::dsl::{delete, insert_into};
use std::vec::Vec;

pub async fn get_users(db: Pool) -> Vec<User> {
    match db_get_all_users(db).await {
        Ok(items) => items,
        Err(e) => panic!("Error get_users: {}", e),
    }
}

pub async fn get_user_by_id(db: Pool, user_id: i32) -> User {
    match db_get_user_by_id(db, user_id) {
        Ok(item) => item,
        Err(e) => panic!("Error get_user_by_id: {}", e),
    }
}

async fn db_get_all_users(pool: Pool) -> Result<Vec<User>, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let items = users.load::<User>(&conn)?;
    Ok(items)
}

fn db_get_user_by_id(pool: Pool, user_id: i32) -> Result<User, diesel::result::Error> {
    let conn = pool.get().unwrap();
    users.find(user_id).get_result::<User>(&conn)
}
