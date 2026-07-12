use md5::{Md5, Digest};

pub fn calculate_response_authenticator(response_bytes: &[u8], shared_secret: &[u8]) -> [u8; 16] {
    let mut hasher = Md5::new();

    hasher.update(response_bytes); 
    hasher.update(shared_secret); 
    
    let result = hasher.finalize();
    let mut auth = [0u8; 16];

    auth.copy_from_slice(&result);

    auth 
}

pub fn decrypt_password(encrypted: &[u8], shared_secret: &[u8], authenticator: &[u8; 16]) -> String {
    let mut hasher = Md5::new();

    hasher.update(shared_secret); 
    hasher.update(authenticator); 

    let block = hasher.finalize();
    let decrypted: Vec<u8> = encrypted.
    iter()
    .zip(block.iter())
    .map(|(e,b)| e ^ b)
    .take_while(|&b| b != 0)
    .collect();

    String::from_utf8_lossy(&decrypted).to_string()
}

fn _main() {
    println!("Hello Password");
}
