use lettre::{transport::smtp::authentication::Credentials, Message, SmtpTransport, Transport};

use super::{errors::Result, MediaType};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub smtp: String,
    pub imap: String,
    pub user: String,
    pub password: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            smtp: "smtp.gmail.com".to_string(),
            imap: "imap.gmail.com".to_string(),
            user: "who-am-i@gmail.com".to_string(),
            password: "change-me".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub to: String,
    pub cc: Vec<String>,
    pub bcc: Vec<String>,
    pub subject: String,
    pub body: String,
    pub media_type: MediaType,
}

impl Task {
    pub fn new(to: &str, subject: &str, body: &str) -> Self {
        Self {
            to: to.to_string(),
            cc: Vec::new(),
            bcc: Vec::new(),
            subject: subject.to_string(),
            body: body.to_string(),
            media_type: MediaType::default(),
        }
    }
}

impl Config {
    pub const OUT: &'static str = "mailer.smtp";
    pub fn send(&self, task: &Task) -> Result<()> {
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

        let mailer = SmtpTransport::relay(&self.smtp)?
            .credentials(Credentials::new(self.user.clone(), self.password.clone()))
            .build();

        mailer.send(&email)?;
        Ok(())
    }
}
