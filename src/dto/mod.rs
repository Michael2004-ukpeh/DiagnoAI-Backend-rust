use crate::models::user::Gender;
use bcrypt::{hash, DEFAULT_COST};
use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Clone)]
pub struct SignUpDTO {
    pub full_name: String,
    pub email: String,
    pub year_of_birth: DateTime,
    pub gender: Gender,
    pub password: String,
}

impl SignUpDTO {
    pub fn hash_password(&mut self) -> Self {
        let hashed_password = hash(&self.password, DEFAULT_COST).unwrap();
        Self {
            password: hashed_password,
            ..self.clone()
        }
    }
}
