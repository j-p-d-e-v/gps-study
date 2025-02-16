use serde::Deserialize;
use surrealdb::RecordId;
use crate::db::Db;

#[derive(Debug,Clone, Deserialize)]
pub struct UserData {
    pub id: Option<RecordId>,
    pub name: String,
    pub username: String,
    pub password: String,
    pub client_id: u32,
}

#[derive(Debug)]
pub struct User {
    db: Db
}

impl User {

    pub async fn new() -> Result<Self,String>{
        let db = Db::connect().await?;
        Ok(Self {
            db
        })
    }

    fn get_table(&self) -> String {
        String::from("users")
    }

    pub async fn get_by_client_id(&self,client_id: u32) -> Result<UserData,String> {
        
        match self.db.client.query("SELECT `id`,`client_id`,`name`,`username`,`password` FROM type::table($table) WHERE `client_id`=$client_id")
            .bind(("table",self.get_table()))
            .bind(("client_id",client_id)).await {
            Ok(mut result) => {

                match result.take::<Option<UserData>>(0) {
                    Ok(record)=> {
                        if record.is_none() {
                            return Err("user not found".to_string());
                        }
                        Ok(record.unwrap())
                    }
                    Err(error) => {
                        Err(format!("user.get_by_client_id error: {:?}",error))    
                    }
                }
                
            }
            Err(error) => {            
                Err(format!("user.get_by_client_id  error: {:?}",error))    
            }
        }
    }

    pub async fn get_by_username_and_password(&self,username: &str, password: &str ) -> Result<UserData,String> {
         match self.db.client.query("SELECT `id`,`client_id`,`name`,`username`,`password` FROM type::table($table) WHERE `username`=$username AND `password`=$password")
            .bind(("table",self.get_table()))
            .bind(("username",username.to_string()))
            .bind(("password",password.to_string())).await {
            Ok(mut result) => {
                match result.take::<Option<UserData>>(0) {
                    Ok(record)=> {
                        if record.is_none() {
                            return Err("user not found".to_string());
                        }
                        Ok(record.unwrap())
                    }
                    Err(error) => {
                        Err(format!("user.get_by_username_and_password error: {:?}",error))    
                    }
                } 
            }
            Err(error) => {            
                Err(format!("user.get_by_username_and_password error: {:?}",error))    
            }
        }
    }
}


#[cfg(test)]
mod test_user {
    use super::*;

    #[tokio::test]
    async fn test_get_user_by_username_and_password()  {

        let user = User::new().await;
        assert!(user.is_ok(),"{:?}",user.err());

        let user = user.unwrap();

        let data = user.get_by_username_and_password("root", "notsecurepassword").await;
        assert!(data.is_ok(),"{:?}",data.err());

        let data = user.get_by_client_id(data.unwrap().client_id).await;
        assert!(data.is_ok(),"{:?}",data.err());
        println!("{:#?}",data);


    }
}
