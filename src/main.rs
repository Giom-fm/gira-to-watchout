#[macro_use]
extern crate rocket;
use rocket::http::Status;
use rocket::Request;
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

#[get("/kill")]
fn halt() -> rocket::http::Status {
    let result = Telnet::connect((TELNET_HOST, TELNET_PORT), 256);
    let mut connection = match result {
        Ok(res) => res,
        Err(_) => return Status::InternalServerError,
    };

    if let Err(()) = authenticate(&mut connection) {
        return Status::InternalServerError;
    }

    if let Err(_) = send(&mut connection, "kill\n") {
        return Status::InternalServerError;
    };

    Status::NoContent
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .register("/", catchers![not_found])
        .mount("/", routes![run, halt])
}
