#[macro_use]
extern crate rocket;
use rocket::http::Status;
use rocket::Request;
use std::sync::{Arc, Mutex};
use telnet::{Event, Telnet};

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Sorry, '{}' is not a valid path.", req.uri())
}

#[get("/run/<task>")]
fn run(task: String) -> rocket::http::Status {
    let result = Telnet::connect(("127.0.0.1", 30100), 256);
    let mut connection = match result {
        Ok(res) => res,
        Err(_) => return Status::InternalServerError,
    };

    let command = format!("authenticate 1\nrun {}", task);
    connection.write(command.as_bytes()).unwrap();
    Status::NoContent
}

#[rocket::main]
async fn main() {
    rocket::build()
        .register("/", catchers![not_found])
        .mount("/", routes![run])
        .launch()
        .await;

    /*loop {
        let event = connection.read_nonblocking().expect("Read error");
        if let Event::Data(buffer) = event {
            // Debug: print the data buffer
            println!("{:?}", buffer);
            // process the data buffer
        }
    }*/
}
