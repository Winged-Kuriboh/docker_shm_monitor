#[cfg(test)]
mod tests {
    use docker_monitor::{send_email, EmailInfo};

    #[test]
    fn test_send_email() {
        let _ = send_email();
    }
    #[test]
    fn test_from_file(){

        let email_info = EmailInfo::from_file("config");
        println!("{:?}",email_info)
    }
}
