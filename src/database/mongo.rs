use async_trait::async_trait;
use mongodb;
use rocket::Request;
use rocket::request::{Outcome, FromRequest};

use crate::database::database;

static DBTYPE: &'static str = "mongodb";

pub struct Client {
    pub uri: Option<String>,
    pub client: Option<mongodb::Client>,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Client {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        request.guard::<Client>().await
    }
}

#[async_trait]
impl database::Driver<mongodb::Client> for Client {
    async fn connect(&mut self) -> Result<(), database::ConnectionError> {
        let uri = self.uri.as_ref()
            .expect("expected uri not to be empty");
        let client_options = mongodb::options::ClientOptions::parse(uri)
            .await
            .expect("Failed to parse");

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
        Ok(self.client.as_ref())
    }
    fn get_db_type(&self) -> Result<String, ()> {
        Ok(DBTYPE.to_string())
    }
}
