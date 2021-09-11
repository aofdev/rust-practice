mod handlers;
mod models;

use crate::user_rpc::handlers::{add_user, delete_user, get_user, get_users, update_user};

use crate::user::{
    user_service_server::UserService, CreateUserReply, CreateUserRequest, DeleteUserReply, Empty,
    UpdateUserReply, UpdateUserRequest, UserReply, UserRequest, Users,
};
use crate::user_rpc::models::InputUser;
use crate::Pool;
use tonic::{Request, Response, Status};
pub struct User {
    pub pool: Pool,
}

#[tonic::async_trait]
impl UserService for User {
    async fn get_user(&self, request: Request<UserRequest>) -> Result<Response<UserReply>, Status> {
        println!("Got a request: {:#?}", &request);
        let UserRequest { id } = &request.into_inner();
        let user_id = id.to_string().parse::<i32>().unwrap();
        let user = get_user(self.pool.clone(), user_id).await;

        let reply = UserReply {
            id: user.id.to_string(),
            first_name: user.first_name.to_string(),
            last_name: user.last_name.to_string(),
            email: user.email.to_string(),
            created_at: user.created_at.to_string(),
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
    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<CreateUserReply>, Status> {
        println!("Got a request: {:#?}", &request);
        let CreateUserRequest {
            first_name,
            last_name,
            email,
        } = &request.into_inner();

        let input_user = InputUser {
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            email: email.to_string(),
        };

        let number_of_rows_affected = add_user(self.pool.clone(), input_user).await;

        let reply = if number_of_rows_affected == 0_usize {
            CreateUserReply {
                message: "Fail to create user".to_string(),
            }
        } else {
            CreateUserReply {
                message: format!("Create {} user", &number_of_rows_affected),
            }
        };

        Ok(Response::new(reply))
    }

    async fn update_user(
        &self,
        request: Request<UpdateUserRequest>,
    ) -> Result<Response<UpdateUserReply>, Status> {
        println!("Got a request: {:#?}", &request);
        let UpdateUserRequest {
            id,
            first_name,
            last_name,
            email,
        } = &request.into_inner();

        let user_id = id.to_string().parse::<i32>().unwrap();
        let input_user = InputUser {
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            email: email.to_string(),
        };

        let number_of_rows_affected = update_user(self.pool.clone(), user_id, input_user).await;

        let reply = if number_of_rows_affected == 0_usize {
            UpdateUserReply {
                message: format!("Fail to update the user with id {}.", user_id),
            }
        } else {
            UpdateUserReply {
                message: format!(
                    "Update {} user with id {}.",
                    &number_of_rows_affected, &user_id
                ),
            }
        };

        Ok(Response::new(reply))
    }

    async fn delete_user(
        &self,
        request: Request<UserRequest>,
    ) -> Result<Response<DeleteUserReply>, Status> {
        println!("Got a request: {:#?}", &request);
        let UserRequest { id } = &request.into_inner();
        let user_id = id.to_string().parse::<i32>().unwrap();

        let number_of_rows_affected = delete_user(self.pool.clone(), user_id).await;
        let reply = if number_of_rows_affected == 0_usize {
            DeleteUserReply {
                message: format!("Fail to delete the user with id {}.", user_id),
            }
        } else {
            DeleteUserReply {
                message: format!("Remove the user with id {}.", user_id),
            }
        };

        Ok(Response::new(reply))
    }
}
