use std::collections::HashMap;
use rocket::{Request};
use rocket::request::{FromRequest, Outcome};

use crate::{database, services};

#[derive()]
pub struct App {
    pub mongo_dbs: HashMap<String, database::mongo::Client>,
    pub services: services::Services,
}

impl Default for App {
    fn default() -> Self {
        App {
            mongo_dbs: HashMap::new(),
            services: services::Services {
                db: None,
                users: services::users::UsersService {
                    db: None,
                }
            }
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
