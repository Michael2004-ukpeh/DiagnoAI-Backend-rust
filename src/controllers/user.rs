use crate::models::user::User;
use mongodb::Database;
use serde_json::json;
use crate::dto::SignUpDTO;
use actix_web::{HttpResponse, web};


pub async fn signup(signup_dto:web::Json<SignUpDTO>, data: web::Data<Database>) -> HttpResponse{
   HttpResponse::Ok().body(json!({
    "susess":true,
    "message":String::from("dearhkh")
   }))
}
