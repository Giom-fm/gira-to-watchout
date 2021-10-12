#[macro_use]
extern crate rocket;
use rocket::http::Status;
use rocket::Request;
use telnet::{Telnet,Event};

const TELNET_HOST: &str = "127.0.0.1";
const TELNET_PORT: u16 = 30100;

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

    
    connection.write("authenticate 1".as_bytes()).unwrap();
    let event = connection.read().expect("Read error");
    if let Event::Data(buffer) = event {
        // Debug: print the data buffer
        println!("{:?}", buffer);
        // process the data buffer
    }
    connection.write(format!("run {}", task).as_bytes()).unwrap();
    Status::NoContent
}

#[rocket::main]
async fn main() {
    rocket::build()
        .register("/", catchers![not_found])
        .mount("/", routes![run])
        .launch()
        .await;


     
}
