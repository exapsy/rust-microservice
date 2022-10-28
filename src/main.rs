use std::borrow::{Borrow, BorrowMut};
use std::fmt::Pointer;
use std::ops::Deref;

use dotenv::dotenv;
use mongodb::{bson::doc, Client, options::ClientOptions};

use crate::app::AppMongoDBDatabases;
use crate::database::database::Driver;

mod database;
mod app;

#[tokio::main]
async fn main() -> Result<(), ()> {
    dotenv().ok().expect("dotenv failed");

    let mongodb_uri = std::env::var("MONGO_URI").expect("");

    let mut a = app::new();

    async fn connect_mongodb(a: &mut app::App, mongo_uri: String) -> Result<(&Client), ()> {
        let mut mdb = database::mongo::new(<String as Borrow<String>>::borrow(&mongo_uri).to_string());
        mdb.connect(
            <String as Borrow<String>>::borrow(
                &mongo_uri
            ).to_string()
        )
            .await
            .expect("expected MongoDB connection");

        a.add_mongodb(String::from("main"), mdb)
            .expect("failed to add mongo database");

        let db = a.get_mongodbs();
        let maindb = db.get("main").unwrap();
        let mongodb_client = maindb.get_client()
            .unwrap()
            .expect("failed to get mongodb client");

        Ok(mongodb_client)
    }

    if !mongodb_uri.is_empty() {
        connect_mongodb(a.borrow_mut(), mongodb_uri).await
            .expect("failed to connect to MongoDB");
    }


    Ok(())
}
