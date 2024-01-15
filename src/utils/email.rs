use async_trait::async_trait;
use dotenv;
use lettre::transport::smtp::authentication::Credentials;
use lettre::message::header::ContentType;
use lettre::{Message ,SmtpTransport, Transport,AsyncSmtpTransport, Tokio1Executor};

struct Email{
 pub to: String, 
 pub from: String,
 pub subject:String,
 pub template: String

}

#[async_trait]
pub trait SendEmailTrait{
    async fn send_email(&self) -> Result<(), ()>;
}

#[async_trait]
impl SendEmailTrait for Email{
    async fn send_email(&self) -> Result<(),()> {
        dotenv().ok();
        let email_username =
            std::env::var("EMAIL_USER").expect("Please provide email username");
        let email_pass =
            std::env::var("EMAIL_PASS").expect("Please provide email pass");
        let email = Message::builder().to(self.to.parse().unwrap()).from(self.from.parse().unwrap()).subject(format!("{}", self.subject)).header(ContentType::TEXT_HTML).body(format!("{}", self.template)).expect("Failed to build email");
         
         let creds = Credentials::new(email_username.to_string(), email_pass.to_string());

         // Open a remote connection to gmail 
        let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay("smtp.gmail.com").unwrap().credentials(cred).build();
       let return_statement = mailer.send(&email).await.unwrap_or_else(|err|{
        return Err(err)
       });
       if return_statement{
        println!("Email sent");;
       }
      Ok(())
    }
}