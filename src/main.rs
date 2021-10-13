#[macro_use]
extern crate rocket;
use rocket::http::Status;
use rocket::Request;
use std::io::prelude::*;
use std::net::TcpStream;
use telnet::{Event, Telnet};

const TELNET_HOST: &str = "192.168.1.11";
const TELNET_PORT: u16 = 3039;

fn authenticate(connection: &mut telnet::Telnet) -> Result<(), ()> {
    if let Err(_) = send(connection, "authenticate 1\n") {
        return Err(());
    }

    loop {
        let result = connection.read_nonblocking();
        let event = match result {
            Ok(event) => event,
            Err(_) => return Err(()),
        };

        if let Event::Data(buffer) = event {
            let response = String::from_utf8_lossy(&buffer);
            println!("{:?}", response);
            break;
        }
    }
    Ok(())
}

fn send(connection: &mut telnet::Telnet, command: &str) -> Result<(), ()> {
    if let Err(_) = connection.write(command.as_bytes()) {
        return Err(());
    };
    Ok(())
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Sorry, '{}' is not a valid path.", req.uri())
}

#[get("/run/<task>")]
fn run(task: String) -> rocket::http::Status {
    let result = Telnet::connect((TELNET_HOST, TELNET_PORT), 256);

    let mut connection = match result {
        Ok(res) => res,
        Err(_) => return Status::InternalServerError,
    };

    if let Err(_) = authenticate(&mut connection) {
        return Status::InternalServerError;
    }

    if let Err(_) = send(&mut connection, &format!("run {}\n", task)) {
        return Status::InternalServerError;
    };

    Status::NoContent
}

#[get("/monitor/<state>")]
fn monitor(state: String) -> rocket::http::Status {
    let mut stream = TcpStream::connect("192.168.1.55:1515").unwrap();

    let start: u8 = 0xAA;
    let cmd: u8 = 0x11;
    let id: u8 = 0x00;
    let data_length: u8 = 0x01;
    let data: u8;
    let checksum: u8;

    if state == "on" {
        data = 0x01;
        checksum = 0x13;
    } else if state == "off" {
        data = 0x00;
        checksum = 0x12;
    } else {
        return Status::NotFound;
    }

    let buffer = [start, cmd, id, data_length, data, checksum];
    println!("{:?}", buffer);

    stream.write(&buffer).unwrap();
    //stream.write("TEST".as_bytes()).unwrap();
    Status::NoContent
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .register("/", catchers![not_found])
        .mount("/", routes![run, monitor])
}
