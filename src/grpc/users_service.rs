use users_service_server::{UsersService};
use tonic::{Response, Status, Request};
use crate::services;

tonic::include_proto!("users_service");

pub struct Users {
    pub services: services::Services,
}

#[tonic::async_trait]
impl UsersService for Users {
    ///
    ///
    /// # Arguments
    ///
    /// * `request`:
    ///
    /// returns: Result<Response<UserGetByIdResponse>, Status>
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    async fn get_by_id(&self, request: Request<UserGetByIdRequest>) -> Result<Response<UserGetByIdResponse>, Status> {
        let request = request.into_inner();
        let user_id = request.id;
        let user = self.services.users.get_by_id(user_id).await.expect("error getting user by id");
        Ok(Response::new(UserGetByIdResponse { user: Some(user.clone()) }))
    }
}

impl Default for Users {
    fn default() -> Self {
        Self {
            services: services::Services::default(),
        }
    }
}