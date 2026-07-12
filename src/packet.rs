#[derive(Debug)]
pub struct Attributes {
    pub typ: u8,
    pub length: u8,
    pub data: Vec<u8>
}

#[derive(Debug)]
pub struct Packet {
    pub code: u8,
    pub identifier: u8,
    pub length: u16,
    pub authenticator: [u8; 16],
    pub attributes: Vec<Attributes>,
}

impl Packet {
    pub fn parse(bytes: &[u8]) -> Packet {
        let code = bytes[0];
        let identifier = bytes[1];
        let length: u16 = u16::from_be_bytes([ bytes[2], bytes[3] ]);
        let mut authenticator: [u8; 16] = [0; 16];
        let mut index = 0;
        let mut attributes = Vec::new();

        for &u in &bytes[4..=19] {
            authenticator[index] = u;
            index += 1;
        }

        // authenticator.copy_from_slice(&bytes[4..20]);

        index = 20;

        while index < length as usize {
            let typ = bytes[index];
            let length = bytes[index + 1] as usize;
            let data = bytes[index + 2..index + length].to_vec();

            attributes.push(Attributes{
                typ,
                length: length as u8,
                data,
            });

            index += length;
        }

        Packet {
            code,
            identifier,
            length,
            authenticator,
            attributes,
        }
    }

    pub fn to_byte(&self) -> Vec<u8> {
        let mut out = Vec::new();

        out.push(self.code);
        out.push(self.identifier);
        out.extend_from_slice(&self.length.to_be_bytes());
        out.extend_from_slice(&self.authenticator);

        for attribute in self.attributes.iter() {
            out.push(attribute.typ);
            out.push(attribute.length);
            out.extend_from_slice(&attribute.data);
        }

        out
    }
}

fn _main() {
    println!("Hello Packet");

    let teste: [u8; 20] = [
        1,                                                          // code = 1 (Access-Request)
        5,                                                          // identifier = 5
        0, 20,                                                      // length = 20 (em 2 bytes, big-endian)
        0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88,            // authenticator
        0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x00,            // (16 bytes no total)
    ];  
    
    let return_parse = Packet::parse(&teste);

    println!("{:?}", return_parse);
} 
