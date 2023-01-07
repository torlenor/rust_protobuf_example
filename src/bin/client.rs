use std::{env, net::UdpSocket};

use prost::Message;
use prost_address_example::{
    person::{PhoneNumber, PhoneType},
    AddressBook, AddressBookReceived, Person,
};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} hostname", args[0]);
        std::process::exit(1);
    }
    let hostname = &args[1];

    let socket = UdpSocket::bind("[::]:0")?; // for UDP4/6

    let address = get_addressbook();
    let bytes = address.encode_to_vec();

    socket
        .send_to(&bytes, hostname.to_string() + ":2000")
        .expect("Error sending the message");

    let mut buf = [0; 2048];
    let (n_bytes, _) = socket.recv_from(&mut buf)?;

    match AddressBookReceived::decode(&buf[..n_bytes]) {
        Ok(answer) => {
            if answer.ok {
                println!("OK");
            } else {
                println!("NOT OK");
            }
        }
        Err(err) => {
            println!("Error decoding the received answer: {}", err);
        }
    };

    Ok(())
}

fn get_addressbook() -> AddressBook {
    let mut address = AddressBook::default();
    address.people.push(Person {
        name: Some("Some Guy".to_string()),
        id: Some(123),
        email: Some("test@example.com".to_string()),
        phones: vec![PhoneNumber {
            number: Some("123-4567-643".to_string()),
            r#type: Some(PhoneType::Work.into()),
        }],
    });
    address
}
