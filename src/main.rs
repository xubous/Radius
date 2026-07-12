use tokio::net::UdpSocket;
use chrono::Local;

mod auth;
mod packet;
mod password;
mod users;
mod log;

#[tokio::main]
async fn main() {
    println!("Servidor RADIUS ouvindo na porta 1812");

    let socket = UdpSocket::bind("0.0.0.0:1812").await.unwrap();
    let mut buf = [0u8; 4096];

    loop {
        let (len, addr) = socket.recv_from(&mut buf).await.unwrap();

        println!("Len: {} Addr: {}",len, addr);

        let request  = packet::Packet::parse(&buf[..len]);

        match request.code {
            1 => {
                let username = auth::get_username(&request).unwrap();

                println!("Username: {}", username);

                let encrypted = auth::get_password(&request).unwrap();
                let password = password::decrypt_password(&encrypted, b"tanto_faz", &request.authenticator);

                println!("Password: {}", password);

                let verifyed = auth::verify(username.clone(), password.clone());

                if verifyed {
                    println!("Authentication succeeded");
                } else {
                    println!("Authentication failed");
                }

                let now = Local::now().format("%Y-%m-%d %H:%M:%S");

                let log = format!( "{} {} {} {}", now, username, password, verifyed);

                log::save_log_handle_auth(log);

                let code = if verifyed { 2 } else { 3 };
                let mut response = packet::Packet {
                    code,
                    identifier: request.identifier,
                    length: 20,
                    authenticator: request.authenticator,   
                    attributes: vec![],
                };
                let response_bytes = response.to_byte();
                
                response.authenticator = password::calculate_response_authenticator(&response_bytes, b"tanto_faz");

                let response_bytes = response.to_byte();

                socket.send_to(&response_bytes, addr).await.unwrap();
            }

            4 => {
                let log = auth::handle_accounting(&request);

                log::save_log_handle_accounting(log);
                    
                let mut response = packet::Packet {
                    code: 5,
                    identifier: request.identifier,
                    length: 20,
                    authenticator: request.authenticator,
                    attributes: vec![],
                };
                let response_bytes = response.to_byte();

                response.authenticator = password::calculate_response_authenticator(&response_bytes, b"tanto_faz");

                let response_bytes = response.to_byte();

                socket.send_to(&response_bytes, addr).await.unwrap();
            }

            _ => {
                println!("Pacote desconhecido: code {}", request.code);
            }

        }
    }
}
