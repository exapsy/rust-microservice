use tonic::transport::Server;

pub mod users_service;

pub struct Grpc {
    pub users: users_service::Users,
    grpc_addr: String,
}

pub struct Options {
    pub grpc_host: String,
}

impl Grpc {
    pub fn new(opts: Options) -> Self {
        let mut grpc_addr = "[::1]:50051".to_string();
        if !opts.grpc_host.is_empty() {
            grpc_addr = opts.grpc_host;
        }

        return Grpc {
            users: users_service::Users::default(),
            grpc_addr,
        }
    }

    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let users_service_grpc = users_service::Users::default();

        println!("grpc server listening to {}", self.grpc_addr);

        Server::builder()
            .add_service(
                users_service::users_service_server::UsersServiceServer::new(users_service_grpc)
            )
            .serve(self.grpc_addr.parse().unwrap())
            .await?;

        Ok(())
    }
}

impl Default for Grpc {
    fn default() -> Self {
        Grpc {
            users: users_service::Users::default(),
            grpc_addr: "[::1]:50051".to_string(),
        }
    }
}