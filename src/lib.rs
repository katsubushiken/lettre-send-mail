use std::error::Error;

use lettre::{
    message::header::ContentTransferEncoding,
    message::{header, SinglePart},
    transport::smtp::authentication::Credentials,
    Message, SmtpTransport, Transport,
};

pub struct MyMailbox<'a> {
    host: &'a str,
    user: &'a str,
    password: &'a str,
}
pub struct MyMessage<'a> {
    from: &'a str,
    to: &'a str,
    subject: &'a str,
    body: &'a str,
    encoding: ContentTransferEncoding,
    content_type: &'a str,
}
impl<'a> Default for MyMessage<'a> {
    fn default() -> Self {
        Self {
            from: "",
            to: "",
            subject: "",
            body: "",
            encoding: header::ContentTransferEncoding::Base64,
            content_type: "text/plain; charset=utf-8",
        }
    }
}
pub fn send_mail(mailbox: &MyMailbox, message: &MyMessage) -> Result<(), Box<dyn Error>> {
    let email = Message::builder()
        .from(message.from.parse()?)
        .to(message.to.parse()?)
        .subject(message.subject)
        .singlepart(
            SinglePart::builder()
                .header(message.encoding)
                .header(header::ContentType(message.content_type.parse()?))
                .body(message.body),
        )?;

    let creds = Credentials::new(mailbox.user.to_string(), mailbox.password.to_string());

    let mailer = SmtpTransport::relay(&mailbox.host)?
        .credentials(creds)
        .build();

    mailer.send(&email).map(|_| Ok(()))?
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mailbox = MyMailbox {
            host: "ホスト名",
            user: "ユーザー名",
            password: "パスワード",
        };
        let message = MyMessage {
            from: "差出アドレス",
            to: "宛先アドレス",
            subject: "テストメール",
            body: "テスト",
            ..Default::default()
        };
        assert!(send_mail(&mailbox, &message).is_ok());
    }
}
