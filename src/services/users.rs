use serde::ser::{Serialize, SerializeStruct, Serializer};
use rocket::serde;
use crate::users_service::User;

static DB_USER_TABLE: &'static str = "users";

pub enum DbType {
    MongoDb { db: mongodb::Database }
}

pub struct UsersService {
    pub db: Option<DbType>
}

impl Serialize for User {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("User", 3)?;
        s.serialize_field("username", &self.username)?;
        s.end()
    }
}

impl UsersService {
    pub fn insert(&self, user: User) -> Result<(), ()> {
        match &self.db {
            Some(DbType::MongoDb { ref db }) => self.insert_mongodb(user),
            None => Err(()),
        }
    }

    fn insert_mongodb(&self, user: User) -> Result<(), ()> {
        match &self.db {
            Some(DbType::MongoDb { ref db }) => {
                let col = db.collection(DB_USER_TABLE);
                let _ = col.insert_one(user, None);
            },
            None => (),
        };
        Ok(())
    }
}