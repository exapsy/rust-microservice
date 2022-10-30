use rocket::form::FromForm;

pub mod users;

pub struct Services {
    pub db: Option<mongodb::Database>,
    pub users: users::UsersService,
}
