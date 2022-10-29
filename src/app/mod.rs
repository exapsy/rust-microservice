use std::collections::HashMap;
use rocket::{Request};
use rocket::request::{FromRequest, Outcome};

use crate::{database, services};

#[derive()]
pub struct App {
    pub mongo_dbs: HashMap<String, database::mongo::Client>,
    pub services: services::Services,
}

#[async_trait]
impl<'r> FromRequest<'r> for App {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        request.guard::<Self>().await
    }
}
