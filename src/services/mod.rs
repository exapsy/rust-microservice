pub mod users;

pub static ERR_NO_DB_FOUND: &'static str = "No database was found";
pub static ERR_NO_MONGODB_FOUND: &'static str = "No mongodb database was found";

pub struct Services {
    pub db: Option<mongodb::Database>,
    pub users: users::UsersService,
}

impl Clone for Services {
    fn clone(&self) -> Self {
        Services {
            db: self.db.clone(),
            users: self.users.clone(),
        }
    }
}

impl Default for Services {
    fn default() -> Self {
        Self {
            db: None,
            users: users::UsersService::default(),
        }
    }
}