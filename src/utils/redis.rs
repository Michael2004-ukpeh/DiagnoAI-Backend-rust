use redis::{Client, RedisError};

pub async fn create_client(redis_uri:String) -> Result<Client, RedisError>{
    match Client::open(redis_uri) {
        Ok(client) => Ok(client),
        Err(err) => {
            eprintln!("Failed to create Redis client: {}", err);
            Err(err)
        }
    }
}