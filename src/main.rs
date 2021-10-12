#[macro_use]
extern crate rocket;
use rocket::http::Status;
use rocket::Request;
use telnet::{Telnet,Event};

const TELNET_HOST: &str = "192.168.1.11";
const TELNET_PORT: u16 = 3039;

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

    connection.write("authenticate 1\n".as_bytes()).unwrap();
    loop {
        let result = connection.read_nonblocking();
        let event = match result {
            Ok(res) => res,
            Err(_) => return Status::InternalServerError,
        };
        
        if let Event::Data(buffer) = event {
            println!("{:?}", buffer);
            break;
        }
    }
    connection.write(format!("run {}\n", task).as_bytes()).unwrap();
    Status::NoContent
}

#[get("/halt")]
fn halt() -> rocket::http::Status {
    let result = Telnet::connect((TELNET_HOST, TELNET_PORT), 256);
    let mut connection = match result {
        Ok(res) => res,
        Err(_) => return Status::InternalServerError,
    };

    connection.write("authenticate 1\n".as_bytes()).unwrap();
    connection.write("halt\n".as_bytes()).unwrap();
    Status::NoContent
}

#[rocket::main]
async fn main() {
    rocket::build()
        .register("/", catchers![not_found])
        .mount("/", routes![run, halt])
        .launch()
        .await;


     
}
