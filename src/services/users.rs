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
    pub async fn insert(&self, user: User) -> Result<(), String> {
        match &self.db {
            Some(DbType::MongoDb { ref db }) => self.insert_mongodb(user).await,
            None => Err(String::from(("no database found"))),
        }
    }

    async fn insert_mongodb(&self, user: User) -> Result<(), String> {
        match &self.db {
            Some(DbType::MongoDb { ref db }) => {
                let col = db.collection(DB_USER_TABLE);
                let r = col.insert_one(user, None).await;
                match r {
                    Ok(_) => Ok(()),
                    Err(err) => Err(format!("failed to insert user")),
                }
            },
            _ => Err(String::from("no mongodb database found"))
        }
    }
}