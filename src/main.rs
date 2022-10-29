#[macro_use]
extern crate rocket;

use std::borrow::{Borrow};
use std::collections::HashMap;

use tonic::{Response, Status, Request};
use dotenv::dotenv;
use mongodb::{bson::doc};
use rocket::{Error, State};
use rocket::figment::value::Value;
use rocket::response::content::RawJson;

use users_service::{UserGetByIdResponse, UserGetByIdRequest, User};

pub mod users_service {
    tonic::include_proto!("users_service");
}

pub struct MyUsersService;
#[tonic::async_trait]
impl UsersService for MyUsersService {
    async fn get_by_id(&self, request: Request<UserGetByIdRequest>) -> Result<Response<UserGetByIdResponse>, Status> {
        todo!()
    }
}

use crate::app::App;
use crate::database::database::Driver;
use crate::database::mongo::Client;
use crate::users_service::users_service_server::UsersService;

mod database;
mod app;
mod services;

pub fn init_app() -> App {
    dotenv().ok().expect("dotenv failed");

    let mongodb_uri = std::env::var("MONGO_URI").expect("");

    App {
        mongo_dbs: HashMap::new(),
    }
}

#[get("/app")]
async fn approute(app: &State<App>) -> RawJson<String> {
    let main_mongodb = app
        .mongo_dbs
        .get("main")
        .unwrap();
    let uri = main_mongodb
        .get_uri()
        .expect("failed to get uri");

    let db = main_mongodb.client.as_ref().expect("expected client reference").database("logifox-stores-users");
    let collection = db.collection("users");
    collection.insert_one(User { username: String::from("lol") }, None).await.expect("failed to insert");

    RawJson(uri.to_string())
}

#[rocket::main]
async fn main() -> Result<(), Error> {
    let mut app = init_app();
    let mongodb_uri = std::env::var("MONGO_URI").expect("");
    let mut mongo_options = mongodb::options::ClientOptions::parse(mongodb_uri.to_string()).await.expect("failed to parse mongodb uri");
    let client = mongodb::Client::with_options(mongo_options).expect("expected mongodb client");
    app.mongo_dbs.insert(
        String::from("main"),
        Client{
            uri: Option::from(mongodb_uri.to_string()),
            client: Option::from(client),
        }
    );

    let _rocket = rocket::build()
        .manage(app)
        .mount("/", routes![approute])
        .launch()
        .await?;

    Ok(())
}
