mod handlers;
mod models;

use crate::user_rpc::handlers::{get_user_by_id, get_users};
use chrono::*;
use uuid::Uuid;

use tonic::{Request, Response, Status};

use crate::user::{user_service_server::UserService, Empty, UserReply, UserRequest, Users};
// use crate::user::{
//     user_service_server::UserService, CreateUserReply, CreateUserRequest, DeleteUserReply, Empty,
//     UpdateUserReply, UpdateUserRequest, UserReply, UserRequest, Users,
// };
use crate::Pool;
pub struct User {
    pub pool: Pool,
}

#[tonic::async_trait]
impl UserService for User {
    async fn get_user(&self, request: Request<UserRequest>) -> Result<Response<UserReply>, Status> {
        println!("Got a request: {:#?}", &request);
        let UserRequest { id } = &request.into_inner();
        let parse_id = id.to_string().parse::<i32>().unwrap();

        let user = get_user_by_id(self.pool.clone(), parse_id).await;

        let reply = UserReply {
            id: user.id.to_string(),
            first_name: user.first_name.to_string(),
            last_name: user.last_name.to_string(),
            email: user.email.to_string(),
            created_at: user.email.to_string(),
        };

        Ok(Response::new(reply))
    }

    async fn list_users(&self, request: Request<Empty>) -> Result<Response<Users>, Status> {
        println!("Got a request: {:#?}", &request);

        let users = get_users(self.pool.clone()).await;
        let result: Vec<UserReply> = users
            .iter()
            .map(|u| UserReply {
                id: u.id.to_string(),
                first_name: u.first_name.to_string(),
                last_name: u.last_name.to_string(),
                email: u.email.to_string(),
                created_at: u.created_at.to_string(),
            })
            .collect::<Vec<UserReply>>();

        let reply = Users { users: result };
        Ok(Response::new(reply))
    }
}
