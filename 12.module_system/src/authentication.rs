use crate::server::Connection;

#[derive(Debug)]
pub struct UserCredentials {
    pub username: String,
    pub password: String
}

pub fn authenticate(credentials: UserCredentials,
            connection: &Connection) -> bool {
        println!("[*] Authenticating for {:?} {:?}", credentials, connection);
        if connection.is_success {
            println!("[*] Authenticating was success full.");
            true    
        } else {
            false
        }

}