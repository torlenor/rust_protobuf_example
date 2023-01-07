use prost::Message;
use prost_address_example::{AddressBook, AddressBookReceived};

use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:2000")?;
    let mut buf = [0; 64 * 1024];

    loop {
        let (n_bytes, src) = socket.recv_from(&mut buf)?;

        let address_deserialized = AddressBook::decode(&buf[..n_bytes]);
        let received = match address_deserialized {
            Ok(address) => {
                println!("Received the following address book content: {:?}", address);
                AddressBookReceived { ok: true }
            }
            Err(err) => {
                println!("Error decoding the received message: {}", err);
                AddressBookReceived { ok: false }
            }
        };

        let buf = received.encode_to_vec();
        match socket.send_to(&buf, src) {
            Ok(_) => {}
            Err(err) => println!("Could not send answer to client: {}", err),
        }
    }
}
