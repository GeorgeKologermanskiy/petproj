use crate::service_state::EmailAdapterCredentials;
use lettre::transport::smtp::authentication::Credentials;
use lettre::SmtpTransport;

trait EmailNotifierAdapter {
    fn send_confirm_email_message(&self, email_to: String, link: String);
}

pub struct EmailAdapter {
    pub mailer: SmtpTransport,
}

impl EmailAdapter {
    pub fn new(credentials: &EmailAdapterCredentials) -> Self {
        let creds = Credentials::new(credentials.1.clone(), credentials.2.clone());

        let mailer = SmtpTransport::relay(&credentials.0)
            .unwrap()
            .credentials(creds)
            .build();

        Self { mailer }
    }
}

impl EmailNotifierAdapter for EmailAdapter {
    fn send_confirm_email_message(&self, _email_to: String, _link: String) {}
}
