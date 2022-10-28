use std::ptr::null;

use async_trait::async_trait;
use mongodb;

use crate::database::database;

static DBTYPE: &'static str = "mongodb";

pub struct Client {
    uri: Option<String>,
    client: Option<mongodb::Client>,
}

pub fn new(mongo_uri: String) -> Client {
    Client { uri: Some(mongo_uri.to_string()), client: None }
}

#[async_trait::async_trait]
impl database::Driver<mongodb::Client> for Client {
    async fn connect(&mut self, host_uri: String) -> Result<(), database::ConnectionError> {
        let mut client_options = mongodb::options::ClientOptions::parse(host_uri).await.expect("Failed to parse");

        let client = mongodb::Client::with_options(client_options)
            .expect("failed to register client");

        client
            .database("admin")
            .run_command(mongodb::bson::doc! {"ping": 1}, None)
            .await.expect("failed to run command");

        self.client = Some(client);

        Ok(())
    }
    fn disconnect(&self) -> Result<(), ()> {
        Ok(())
    }
    fn get_client(&self) -> Result<Option<&mongodb::Client>, ()> {
        Ok(Option::from(&self.client))
    }
    fn get_db_type(&self) -> Result<String, ()> {
        Ok(DBTYPE.to_string())
    }
}

impl Client{
    pub fn get_uri(&self) -> Result<String, ()> {
        match self.uri.clone() {
            Some(uri) => Ok(uri),
            None => Err(()),
        }
    }
}