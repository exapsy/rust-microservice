pub mod users_service;

pub struct Grpc {
    pub users: users_service::Users,
}

impl Default for Grpc {
    fn default() -> Self {
        Grpc {
            users: users_service::Users::default(),
        }
    }
}