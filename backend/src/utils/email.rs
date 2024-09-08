use std::env;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};


pub struct EmailMessage {
    pub email_to: String,
    pub email_from: String,
    pub email_subject: String,
    pub email_body: String,
}

impl EmailMessage {
    pub fn recover_acc_email(recover_code: i32, usr_email: String) -> Self {
        dotenv::dotenv().expect("Failed to read .env file");

        EmailMessage {
            email_to: usr_email,
            email_from: env::var("smtp_email").expect("error smtp_email"),
            email_subject: "Recover Account".to_string(),
            email_body: format!(
                "Hello!! \n\n

                 We have received a password change request, 
                 you must use the following code to recover your account.\n\n
                 
                {}",
                recover_code)
        }
    }
}

fn open_conn_email() -> SmtpTransport {
    
    let admin_cred: Credentials = Credentials::new(
        (env::var("smtp_email").expect("")).to_string().to_owned(),
        (env::var("smtp_key").expect("")).to_owned(),    );

    return SmtpTransport::relay(&env::var("smtp_host").expect("").clone())
        .unwrap()
        .credentials(admin_cred)
        .build();
}

pub fn send_email(msg_structure: EmailMessage) -> bool {
    let smtp_transp: SmtpTransport = open_conn_email();

    let email_msg = Message::builder()
        .from(msg_structure.email_from.parse().unwrap())
        .to(msg_structure.email_to.parse().unwrap())
        .subject(msg_structure.email_subject)
        .header(ContentType::TEXT_PLAIN)
        .body(String::from(msg_structure.email_body.clone()))
        .unwrap();

    match smtp_transp.send(&email_msg) {
        Ok(_) => {
            return true;
        }       
        Err(_) => {
            return false;
        }
    }
}