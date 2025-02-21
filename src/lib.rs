use lettre::message::SinglePart;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::process;

#[derive(Debug)]
pub struct EmailInfo {
    pub from_email: String,
    pub to_email: String,
    pub smtp_server: String,
    pub smtp_port: u16,
    pub smtp_user: String,
    pub smtp_password: String,
    pub subject: String,
    pub body: String,
}

impl EmailInfo {
    pub fn new() -> Self {
        EmailInfo {
            from_email: String::new(),
            to_email: String::new(),
            smtp_server: String::new(),
            smtp_port: 0,
            smtp_user: String::new(),
            smtp_password: String::new(),
            subject: String::new(),
            body: String::new(),
        }
    }

    pub fn from_file(file_path: &str) -> Result<Self, io::Error> {
        let path = Path::new(file_path);
        let file = File::open(path)?;
        let reader = io::BufReader::new(file);

        let mut emailinfo = EmailInfo::new();

        for line in reader.lines() {
            let line = line?; // Read line and handle error
            if let Some((key, value)) = parse_line(&line) {
                match key {
                    "from_email" => emailinfo.from_email = value,
                    "to_email" => emailinfo.to_email = value,
                    "smtp_server" => emailinfo.smtp_server = value,
                    "smtp_port" => {
                        emailinfo.smtp_port = value.parse().unwrap_or_else(|err| {
                            println!("端口号错误：{}", err);
                            process::exit(1)
                        })
                    }
                    "smtp_user" => emailinfo.smtp_user = value,
                    "smtp_password" => emailinfo.smtp_password = value,
                    "subject" => emailinfo.subject = value,
                    "body" => emailinfo.body = value,
                    _ => (),
                }
            }
        }

        Ok(emailinfo)
    }
}

// 解析键值对
pub fn parse_line(line: &str) -> Option<(&str, String)> {
    let parts: Vec<&str> = line.splitn(2, '=').collect();
    if parts.len() == 2 {
        Some((parts[0].trim(), parts[1].trim().to_string()))
    } else {
        None
    }
}

pub fn send_email() -> Result<(), Box<dyn std::error::Error>> {
    let email_info = EmailInfo::from_file("config")?;
    let email = Message::builder()
        .from(email_info.from_email.parse()?)
        .to(email_info.to_email.parse()?)
        .subject(email_info.subject)
        .singlepart(SinglePart::plain(email_info.body))?;

    let creds = Credentials::new(
        email_info.smtp_user.to_string(),
        email_info.smtp_password.to_string(),
    );

    let mailer = SmtpTransport::relay(&email_info.smtp_server.to_string())?
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => println!("邮件发送成功！"),
        Err(e) => eprintln!("邮件发送失败: {:?}", e),
    }
    Ok(())
}
