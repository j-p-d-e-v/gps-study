use crate::config::{Config, DatabaseConfig};
use std::sync::LazyLock;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

#[derive(Debug)]
pub struct Db {
    pub client: Surreal<Client>,
}

impl Db {
    pub async fn connect() -> Result<Self, String> {
        let config: Config = Config::load().await?;
        let db_config: DatabaseConfig = config.database;
        let client: Surreal<Client> = Surreal::init();
        match client.connect::<Ws>(db_config.host).await {
            Ok(_) => {
                match client
                    .signin(Root {
                        username: &db_config.username,
                        password: &db_config.password,
                    })
                    .await
                {
                    Ok(_) => match client
                        .use_ns(db_config.namespace)
                        .use_db(&db_config.database)
                        .await
                    {
                        Ok(_) => Ok(Self { client }),
                        Err(error) => Err(format!("database error: {:?}", error)),
                    },
                    Err(error) => Err(error.to_string()),
                }
            }
            Err(error) => Err(format!("database error: {:?}", error)),
        }
    }
}

#[cfg(test)]
mod db_tests {

    use super::*;

    #[tokio::test]
    async fn test_connection() {
        let db = Db::connect().await;
        assert!(db.is_ok(), "{:?}", db.err());
    }
}
