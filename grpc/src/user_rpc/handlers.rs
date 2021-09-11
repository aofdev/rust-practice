use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use crate::schema::users::dsl::*;
use crate::user_rpc::models::{InputUser, NewUser, User};
use crate::Pool;
use diesel::dsl::{delete, insert_into, update};
use std::vec::Vec;

pub async fn get_users(db: Pool) -> Vec<User> {
    match get_all_users_db(db).await {
        Ok(items) => items,
        Err(e) => panic!("Error get_users: {}", e),
    }
}

pub async fn get_user(db: Pool, user_id: i32) -> User {
    match get_user_db(db, user_id).await {
        Ok(item) => item,
        Err(e) => panic!("Error get_user: {}", e),
    }
}

pub async fn add_user(db: Pool, input_user: InputUser) -> usize {
    match add_user_db(db, input_user).await {
        Ok(item) => item,
        Err(e) => panic!("Error add_user: {}", e),
    }
}

pub async fn update_user(db: Pool, user_id: i32, input_user: InputUser) -> usize {
    match update_user_db(db, user_id, input_user).await {
        Ok(item) => item,
        Err(e) => panic!("Error update_user: {}", e),
    }
}

pub async fn delete_user(db: Pool, user_id: i32) -> usize {
    match delete_user_db(db, user_id).await {
        Ok(item) => item,
        Err(e) => panic!("Error delete_user: {}", e),
    }
}

async fn get_all_users_db(pool: Pool) -> Result<Vec<User>, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let items = users.load::<User>(&conn)?;
    Ok(items)
}

async fn get_user_db(pool: Pool, user_id: i32) -> Result<User, diesel::result::Error> {
    let conn = pool.get().unwrap();
    users.find(user_id).get_result::<User>(&conn)
}

async fn add_user_db(pool: Pool, item: InputUser) -> Result<usize, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let new_user = NewUser {
        first_name: &item.first_name,
        last_name: &item.last_name,
        email: &item.email,
        created_at: chrono::Local::now().naive_local(),
    };
    let res: usize = insert_into(users).values(&new_user).execute(&conn)?;
    Ok(res)
}

async fn update_user_db(
    pool: Pool,
    user_id: i32,
    item: InputUser,
) -> Result<usize, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let updated = update(users.find(user_id))
        .set(User {
            id: user_id,
            first_name: item.first_name,
            last_name: item.last_name,
            email: item.email,
            created_at: chrono::Local::now().naive_local(),
        })
        .execute(&conn)?;
    Ok(updated)
}

async fn delete_user_db(pool: Pool, user_id: i32) -> Result<usize, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let count = delete(users.find(user_id)).execute(&conn)?;
    Ok(count)
}
