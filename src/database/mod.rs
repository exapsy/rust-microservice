use async_trait::async_trait;
use mongodb;

pub mod mongo;

pub mod database {
    use std::fmt::{Debug, Formatter};

    pub struct ConnectionError {
        pub(crate) source: Option<String>,
    }

    impl Debug for ConnectionError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
           Ok(())
        }
    }

    #[async_trait::async_trait]
    pub trait Driver<OfficialDbDriver> {
        async fn connect(&mut self, host_uri: String) -> Result<(), ConnectionError>;
        fn disconnect(&self) -> Result<(), ()>;
        fn get_client(&self) -> Result<Option<&OfficialDbDriver>, ()>;
        fn get_db_type(&self) -> Result<String, ()>;
    }
}