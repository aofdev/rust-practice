#[macro_use]
extern crate diesel;
extern crate chrono;
extern crate dotenv;

pub mod user {
    tonic::include_proto!("user");
}

use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use tonic::transport::Server;

use user::user_service_server::UserServiceServer;

extern crate uuid;

extern crate console;
use console::Style;

mod schema;
mod user_rpc;
use crate::user_rpc::User;
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let addr = "0.0.0.0:50051".parse().unwrap();

    let blue = Style::new().blue();

    println!("\nRust gRPC Server ready at {}", blue.apply_to(addr));

    // create db connection pool
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let user = User { pool };
    Server::builder()
        .add_service(UserServiceServer::new(user))
        .serve(addr)
        .await?;

    Ok(())
}
