use mongodb::bson::DateTime;
use crate::models::user::Gender;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SignUpDTO{
    full_name:String, 
    email:String, 
    year_of_birth:DateTime, 
    gender:Gender
}

