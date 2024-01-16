use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;
use mongodb::bson::DateTime;
use validator::{Validate};

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct User{
  #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
  id: Option<ObjectId>,
  full_name: String,
  #[validate(email)]
  email: String,
  gender: Gender,
  year_of_birth:DateTime,
  #[validate(length(min = 8, message = "Password must be eight(8) characters long"))]
  password: String, 
  password_changed_at:DateTime,
  password_reset_token_expires_at:Option<DateTime>,
  password_reset_token:Option<String>,
  active: Option<bool>,
  status:Option<bool>,
  is_verified: Option<bool>, 
  otp: Option<u32>,
  otp_expiration: Option<DateTime>
  

}


#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub enum Gender{
  Male, 
  Female
}
