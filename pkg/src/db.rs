use anyhow::{Error, Result};
use async_trait::async_trait;
use sea_orm::{Database, DatabaseConnection};

#[async_trait]
pub trait ORM: Sync + Send {
    async fn get_db(&self) -> &DatabaseConnection;
}

pub struct Mysql {
    db: DatabaseConnection,
}

#[async_trait]
impl ORM for Mysql {
    async fn get_db(&self) -> &DatabaseConnection {
        &self.db
    }
}

impl Mysql {
    pub async fn new() -> Result<Self, Error> {
        let db_url = std::env::var("DATABASE_URL").unwrap();
        let db = Database::connect(&db_url).await?;
        let mdb = Mysql { db };
        Ok(mdb)
    }
}
