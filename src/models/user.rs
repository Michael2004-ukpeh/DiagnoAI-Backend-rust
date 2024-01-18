use mongodb::bson::oid::ObjectId;
use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::dto::SignUpDTO;

#[derive(Serialize, Deserialize, Debug, Validate, Clone)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub full_name: String,
    #[validate(email(message = "Email must be a valid email type"))]
    pub email: String,
    pub gender: Gender,
    pub year_of_birth: DateTime,
    #[validate(length(min = 8, message = "Password must be eight(8) characters long"))]
    pub password: String,
    pub password_changed_at: Option<DateTime>,
    pub password_reset_token_expires_at: Option<DateTime>,
    pub password_reset_token: Option<String>,
    pub active: Option<bool>,

    pub is_verified: bool,
    pub otp: Option<u32>,
    pub otp_expiration: Option<DateTime>,
}

// impl From<SignUpDTO> for User {
//   fn from(source: SignUpDTO) -> Self{
//     User{
//       id:
//     }
//   }
// }
#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub enum Gender {
    Male,
    Female,
}
