use rocket::{Build, Error, Ignite, Response, Rocket};
use rocket::response::content::RawJson;
use crate::grpc::users_service::users_service::User;
use crate::services;

pub struct Handler {
    pub services: services::Services,
    pub rocket: Option<rocket::Rocket<rocket::Build>>
}

pub struct HandlerInitOptions {
    pub services: services::Services,
}

impl Handler {
    pub fn new(opts: HandlerInitOptions) -> Self {
        Self {
            services: opts.services.clone(),
            rocket: None,
        }
    }
    pub async fn run(self) -> Result<Rocket<Ignite>, Error> {
        rocket::build()
            .manage(State{
                services: self.services.clone(),
            })
            .mount("/", routes![adduser])
            .launch()
            .await
    }
}

pub struct State {
    pub services: services::Services,
}

#[get("/adduser")]
async fn adduser(state: &rocket::State<State>) -> RawJson<String> {
    let user = User {
        username: String::from("dum dum"),
    };
    state.services.users.insert(user)
        .await
        .expect("failed to insert user");

    RawJson("{ \"response\": \"user added\" }".to_string())
}

