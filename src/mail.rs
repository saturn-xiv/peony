use uuid::Uuid;

use lettre::{transport::smtp::authentication::Credentials, Message, SmtpTransport, Transport};

use super::{
    errors::Result,
    protos::auth::{ContentType, EmailTask},
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Smtp {
    pub host: String,
    pub user: String,
    pub password: String,
}

impl Default for Smtp {
    fn default() -> Self {
        Self {
            host: "smtp.gmail.com".to_string(),
            user: "who-am-i@gmail.com".to_string(),
            password: "change-me".to_string(),
        }
    }
}

impl Smtp {
    pub const QUEUE: &'static str = "emails";
    pub fn push(to: &str, subject: &str, body: &str) -> (String, EmailTask) {
        (
            Uuid::new_v4().to_string(),
            EmailTask {
                to: to.to_string(),
                subject: subject.to_string(),
                body: body.to_string(),
                content_type: ContentType::PROTOBUF,
                ..Default::default()
            },
        )
    }
    pub fn send(&self, task: &EmailTask) -> Result<()> {
        debug!("send email {:?} to {}", task.subject, task.to);
        let mut email = Message::builder()
            .from(self.user.parse()?)
            // .reply_to(self.user.parse()?)
            .to(task.to.parse()?)
            .subject(&task.subject);
        for it in task.cc.iter() {
            email = email.cc(it.parse()?);
        }
        for it in task.bcc.iter() {
            email = email.bcc(it.parse()?);
        }
        let email = email.body(&task.body)?;

        let mailer = SmtpTransport::relay(&self.host)?
            .credentials(Credentials::new(self.user.clone(), self.password.clone()))
            .build();

        mailer.send(&email)?;
        Ok(())
    }
}
