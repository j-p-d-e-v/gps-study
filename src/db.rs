use std::sync::LazyLock;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::Surreal;

static DB_INSTANCE: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);

#[derive(Debug)]
pub struct Db {
    instance: LazyLock<Surreal<Client>>,
}

impl Db {
    pub async fn connect() -> Result<(), String> {
        let address: String = String::from("127.0.0.1:8080");
        DB_INSTANCE.connect::<Ws>(address).await.unwrap();
        Ok(())
    }
}

#[cfg(test)]
mod db_tests {
    use super::*;

    #[tokio::test]
    async fn test_connection() {
        Db::connect().await.unwrap();
        assert!(true);
    }
}
