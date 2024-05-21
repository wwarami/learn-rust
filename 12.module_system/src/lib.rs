mod authentication;
mod server;

use authentication::{UserCredentials, authenticate};
use server::{Connection, IpAddr, IpAddrKind, connect};

pub fn send_updates(username: String, password: String, data: String) {   
    // create credentials
    let credentials: UserCredentials = UserCredentials {
        username, 
        password};
    println!("[*] Auth credentials created: {:?}", credentials);

    // create ip address
        let ip_address: IpAddr = IpAddr { 
        kind: IpAddrKind::V4, 
        address: String::from("0.0.0.0")};
    println!("[*] IP Address created: {:?}", ip_address);

    // create a connection
    let connection: Connection = connect(ip_address);
    println!("[*] Connection created: {:?}", connection);

    // authenticate
    let is_authenticated = authenticate(credentials, &connection);
    
    if is_authenticated {
        println!("[*] Sending data...");
        connection.send_data(data);
    } else {
        println!("[*] Could not authenticate.")
    }
}
