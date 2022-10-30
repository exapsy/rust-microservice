use users_service::{UserGetByIdResponse, UserGetByIdRequest, User};
use tonic::{Response, Status, Request};
use crate::grpc::users_service::users_service::users_service_server::UsersService;
use crate::services;

pub mod users_service {
    tonic::include_proto!("users_service");
}

pub struct Users {
    pub services: services::Services,
}

#[tonic::async_trait]
impl UsersService for Users {
    async fn get_by_id(&self, request: Request<UserGetByIdRequest>) -> Result<Response<UserGetByIdResponse>, Status> {
        todo!()
    }
}

impl Default for Users {
    fn default() -> Self {
        Self {
            services: services::Services::default(),
        }
    }
}