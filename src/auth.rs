use crate::packet::{Attributes, Packet};
use crate::users::get_users;

pub fn get_username(packet: &Packet) -> Option<String> {
    for attribute in packet.attributes.iter() {
        if attribute.typ == 1 {
            let username = String::from_utf8_lossy(&attribute.data).to_string();

            return Some(username);
        }
    }

    None
}

pub fn get_password(packet: &Packet) -> Option<Vec<u8>> {
    for attribute in packet.attributes.iter() {
        if attribute.typ == 2 {
            return Some(attribute.data.clone());
        }
    }

    None
}

pub fn verify(username: String, password: String) -> bool {
    let users = get_users();

    for current_user in users.iter() {
        if current_user.username == username && current_user.password == password {
            return true;
        }
    }

    false
}

pub fn handle_accounting(packet: &Packet) -> String {
    let mut username: String = String::new();
    let mut session_id: String = String::new();
    let mut status: u32 = 0;

    for attributes in &packet.attributes {
        if attributes.typ == 1 {
            username = String::from_utf8_lossy(&attributes.data).to_string();
        } else if attributes.typ == 44 {
            session_id = String::from_utf8_lossy(&attributes.data).to_string();
        } else if attributes.typ == 40 {
            status = u32::from_be_bytes([
                attributes.data[0],
                attributes.data[1],
                attributes.data[2],
                attributes.data[3],
            ]);
        }
    }

    let status_label = if status == 1 { "START" } else { "STOP" };

    format!("Name: {}, session_id: {}, status: {}", username, session_id, status_label)
}

fn _main() {
    println!("Hello Auth");

    let packet = Packet {
        code: 1,
        identifier: 1,
        length: 0,
        authenticator: [0; 16],
        attributes: vec![
            Attributes { typ: 1, length: 6, data: b"joao".to_vec() },
            Attributes { typ: 2, length: (b"123456".len() + 2) as u8, data: b"123456".to_vec() },
        ],
    };

    let username = get_username(&packet).unwrap();
    let password_bytes = get_password(&packet).unwrap();
    let password = String::from_utf8_lossy(&password_bytes).to_string();

    if verify(username, password) {
        println!("Aceito");
    } else {
        println!("Rejeitado");
    }
}
