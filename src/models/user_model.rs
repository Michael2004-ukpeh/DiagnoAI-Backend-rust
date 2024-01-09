use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;

#[derive(Serialize, Deserialize)]
pub struct User{
  #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
  id: Option<ObjectId>,
  full_name: String,
  email: String,
  gender: Gender,
  

}

pub enum Gender{
  Male, 
  Female
}
