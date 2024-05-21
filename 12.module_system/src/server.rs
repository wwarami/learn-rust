#[derive(Debug)]
pub enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
pub struct IpAddr {
    pub kind: IpAddrKind,
    pub address: String,
}

#[derive(Debug)]
pub struct Connection {
    pub address: IpAddr,
    pub is_success: bool
}
impl Connection {
    pub fn send_data(&self, data: String) -> (bool, String) {
        println!("[*] Sending data '{}'", data);
        (true, data)
    }
}

pub fn connect(ip_addr: IpAddr) -> Connection {
    println!("[*] Connecting to {:?}", ip_addr);
    Connection { address: ip_addr , is_success: true}
}
