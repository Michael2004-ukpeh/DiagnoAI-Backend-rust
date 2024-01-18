use crate::{models::{user::User, token::Token}, utils::{rand_token, email::{Email, SendEmailTrait}}};
use bson::oid::ObjectId;
use mongodb::Database;
use serde_json::json;
use crate::dto::SignUpDTO;
use actix_web::{HttpResponse, web};



pub async fn signup(mut signup_dto:web::Json<SignUpDTO>, data: web::Data<Database>) -> HttpResponse{
    let user_payload =  signup_dto.hash_password();
    // // Create user 
   let mut user_doc = User {
    id:None, 
    full_name:user_payload.full_name, 
    email: user_payload.email,
    gender: user_payload.gender,
    year_of_birth: user_payload.year_of_birth,
    password: user_payload.password,
    password_changed_at: None,
    password_reset_token_expires_at: None,
    password_reset_token: None,
    active: Some(true),
    is_verified: false,
    otp: None,
    otp_expiration: None,
   };
   println!("{:?}", user_doc);
   let user =  data.collection("users").insert_one(user_doc.clone(), None).await;
   user_doc.id = user.unwrap().inserted_id.as_object_id();




   let random_token = rand_token::generate_random_hex_string();
   //Create User Token
   let user_token_doc = Token {
      id:Some(ObjectId::new()), 
      user_id:user_doc.id.unwrap().clone(),
      token: random_token, 
      expires_at:todo!()
   };
   let user_token = data.collection("tokens").insert_one(user_token_doc.clone(), None).await;

    // Send Email
    let email_options= Email::new(user_doc.email, "DiagnoAI - Confirm Sign Uo".to_string(), 
   format!(
    r#"
   <html>
   <body>
   <p>Hi, {&user_doc.full_name} </p>
   <p>Welcome to money transfer,</p>
   <p>We are delighted to have you.</p>
   <p>Please verify your email by clicking on the link below:</p>
   <a href="{verify_account_url}">Click here to verify your account</a>
</body>
   </html>
  "#

)
 )
    // Create user jwt token
   HttpResponse::Ok().json(json!({
    "success":true,
    "message":String::from("dearhkh"), 
    "accessToken":"token",
    "data":user_doc
   }))
}
