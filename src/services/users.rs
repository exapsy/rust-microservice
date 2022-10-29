use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::ops::Deref;
use rocket::serde;
use crate::users_service::User;

static DB_USER_TABLE: &'static str = "users";

pub enum Feature {
    MongoDB { db: mongodb::Database },
}

pub enum DbType {
    MongoDb { db: mongodb::Database }
}

pub struct UsersService {
    pub db: DbType
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
            DbType::MongoDb{ db: client } => self.insert_mongodb(user)
        }
    }

    fn insert_mongodb(&self, user: User) -> Result<(), ()> {
        match &self.db {
            DbType::MongoDb{ db: db } => {
                let col = db.collection(DB_USER_TABLE);
                col.insert_one(user, None);
            }
        };
        Ok(())
    }
}