use std::collections::HashMap;

use crate::database;

#[derive()]
pub struct App {
    mongo_dbs: HashMap<String, database::mongo::Client>,
}

pub struct AppOptions {
    pub name: String,
}

pub fn new() -> App {
    App {
        mongo_dbs: HashMap::new(),
    }
}

pub trait AppMongoDBDatabases {
    fn get_mongodbs(&self) -> &HashMap<String, database::mongo::Client>;
    fn add_mongodb(&mut self, name: String, db: database::mongo::Client) -> Result<(), ()>;
    fn remove_mongodb(&mut self, name: String) -> Result<(), ()>;
}

impl AppMongoDBDatabases for App {
    fn get_mongodbs(&self) -> &HashMap<String, database::mongo::Client> {
        &self.mongo_dbs
    }
    fn add_mongodb(&mut self, name: String, db: database::mongo::Client) -> Result<(), ()> {
        self.mongo_dbs.insert(name, db);
        Ok(())
    }
    fn remove_mongodb(&mut self, name: String) -> Result<(), ()> {
        self.mongo_dbs.remove(&name);
        Ok(())
    }
}