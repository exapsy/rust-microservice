use std::collections::HashMap;

use dotenv::dotenv;
use rocket::{Error, Ignite, Request, Rocket};
use rocket::request::{FromRequest, Outcome};

use crate::{database, grpc, services, http};
use crate::services::users::DbType;

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
            db: Option::from(db.clone()),
            users: services::users::UsersService {
                db: Option::from(DbType::MongoDb { db: db.clone() }),
            },
        };

        Ok(())
    }

    pub async fn run(&mut self) -> Result<Rocket<Ignite>, Error> {
        println!("Starting server");
        http::Handler::new(http::HandlerInitOptions {
            services: self.services.clone(),
        })
            .run()
            .await
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
