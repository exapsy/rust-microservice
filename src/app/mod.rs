use std::collections::HashMap;
use rocket::{Request, request, State};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};

use crate::database;

#[derive()]
pub struct App {
    pub mongo_dbs: HashMap<String, database::mongo::Client>,
}

#[async_trait]
impl<'r> FromRequest<'r> for App {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        request.guard::<Self>().await
    }
}

pub struct AppOptions {
    pub name: String,
}

pub fn new() -> App {
    App {
        mongo_dbs: HashMap::new(),
    }
}
