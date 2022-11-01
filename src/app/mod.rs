use std::collections::HashMap;
use std::env::VarError;

use dotenv::dotenv;
use rocket::{Request};
use rocket::request::{FromRequest, Outcome};

use crate::{database, grpc, http, services};
use crate::app::EnvVarKey::MongoURI;
use crate::services::users::DbType;

enum EnvVarKey {
    MongoURI,
    GrpcHost,
    RocketConfig,
    DbName,
}

impl EnvVarKey {
    pub fn as_str(&self) -> &'static str {
        match self {
            EnvVarKey::MongoURI => "MONGO_URI",
            EnvVarKey::GrpcHost => "GRPC_HOST",
            EnvVarKey::RocketConfig => "ROCKET_CONFIG",
            EnvVarKey::DbName => "DB_NAME",
        }
    }
    pub fn value(&self) -> Result<String, VarError> {
        std::env::var(self.as_str())
    }
}

#[derive()]
pub struct App {
    pub mongo_dbs: HashMap<String, database::mongo::Client>,
    pub services: services::Services,
    grpc: grpc::Grpc,
}

impl App {
    pub fn new() -> Self {
        return Self::default();
    }

    pub async fn init(&mut self) -> Result<&mut Self, ()> {
        dotenv().ok();
        self.init_mongodb().await.expect("failed to init mongodb");
        self.init_http()?;

        Ok(self)
    }

    fn init_http(&mut self) -> Result<(), ()> {
        Ok(())
    }

    async fn init_mongodb(&mut self) -> Result<(), ()> {
        let database_name = std::env::var("DB_NAME")
            .expect("expected to get a database name");
        let mongodb_uri = std::env::var("MONGO_URI")
            .expect("expected to get a mongodb_uri");

        let mongo_options = mongodb::options::ClientOptions::parse(mongodb_uri.to_string())
            .await
            .expect("failed to parse mongodb uri");
        let client = mongodb::Client::with_options(mongo_options)
            .expect("expected mongodb client");
        let db = &client.database(database_name.as_str()).clone();

        self.services = services::Services {
            db: Some(db.clone()),
            users: services::users::UsersService {
                db: Some(DbType::MongoDb { db: db.clone() }),
            },
        };

        Ok(())
    }

    pub async fn run(&mut self) -> Result<(), ()> {
        println!("Starting app");
        let services = self.services.clone();

        let grpc_host = EnvVarKey::GrpcHost.value()
            .expect("grpc host is not set");

        let http_handler =
            http::Handler::new(http::HandlerInitOptions {
                services,
            });
        let grpc_handler =
            grpc::Grpc::new(grpc::Options {
                grpc_host,
            });

        let (rocket_res, grpc_res) = tokio::join!(
                grpc_handler.run(),
                http_handler.run(),
        );

        Ok(())
    }
}

impl Default for App {
    fn default() -> Self {
        App {
            grpc: grpc::Grpc::default(),
            mongo_dbs: HashMap::new(),
            services: services::Services::default(),
        }
    }
}

#[async_trait]
impl<'r> FromRequest<'r> for App {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        request.guard::<Self>().await
    }
}
