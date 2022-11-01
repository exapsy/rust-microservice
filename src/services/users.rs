use serde::ser::{Serialize, SerializeStruct, Serializer};
use rocket::serde;
use crate::grpc::users_service::{User};
use crate::services::{ERR_NO_DB_FOUND, ERR_NO_MONGODB_FOUND};

static DB_USER_TABLE: &'static str = "users";

pub enum DbType {
    MongoDb { db: mongodb::Database }
}

impl Clone for DbType {
    fn clone(&self) -> Self {
        match self {
            DbType::MongoDb { db } => DbType::MongoDb { db: db.clone() }
        }
    }
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
    pub fn mongodb(&self) -> Result<mongodb::Database, String> {
        match &self.db {
            Some(DbType::MongoDb { ref db }) => {
                Ok(db.clone())
            },
            _ => Err(ERR_NO_MONGODB_FOUND.to_string())
        }

    }

    pub async fn get_by_id(&self, id: String) -> Result<User, String> {
        match &self.db {
            Some(DbType::MongoDb {db }) => self.get_by_id_mongodb(id).await,
            None => Err(ERR_NO_DB_FOUND.to_string()),
        }
    }
    pub async fn get_by_id_mongodb(&self, id: String) -> Result<User, String> {
        let db = self.mongodb()?;
        // db.
        todo!()
    }
    pub async fn insert(&self, user: User) -> Result<(), String> {
        match &self.db {
            Some(DbType::MongoDb { ref db }) => self.insert_mongodb(user).await,
            None => Err(ERR_NO_DB_FOUND.to_string()),
        }
    }

    async fn insert_mongodb(&self, user: User) -> Result<(), String> {
        match &self.db {
            Some(DbType::MongoDb { ref db }) => {
                let col = db.collection(DB_USER_TABLE);
                let r = col.insert_one(user, None).await;
                match r {
                    Ok(_) => Ok(()),
                    Err(err) => Err(format!("failed to insert user: {:?}", err)),
                }
            },
            _ => Err(ERR_NO_MONGODB_FOUND.to_string())
        }
    }
}

impl Clone for UsersService {
    fn clone(&self) -> Self {
        UsersService { db: self.db.clone() }
    }
}

impl Default for UsersService {
    fn default() -> Self {
        UsersService { db: None }
    }
}