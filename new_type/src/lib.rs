use std::fmt::Display;

pub struct Password(String);

impl Display for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "****************")
    }
}

#[cfg(test)]
mod tests {
    use super::Password;

    #[test]
    fn test_password() {
        let unsecured_password = "ThisIsMyPassword".to_string();
        let secured_password = Password(unsecured_password.clone());

        println!("unsecured_password: {}", unsecured_password);
        println!("secured_password: {}", secured_password);
    }
}
