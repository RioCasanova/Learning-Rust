/*
    Author: Rio Casanova
    Date Created: January 3, 2025
    Last Edited: Jan 3, 2025
    Language: Rust

    Purpose of program:

        The purpose of this program is to aid in the understanding
        of the Rust language as a personal project.

        This program is supposed to act as a chatroom where
        users can send messages one another via channels over
        the local network


    Libraries

        Rocket:

            A web framework for Rust that simplifies building
            web applications and APIs. It provides routing, request/response
            handling, templating, and more, and supports asynchronous programming.

        Serde:

            A serialization/deserialization library for Rust.
            It allows you to convert Rust data structures to
            and from formats like JSON, YAML, or TOML, making
            it easier to work with external data in a 
            structured way.

        Tokio:

            An asynchronous runtime for Rust, enabling
            efficient concurrency and handling async I/O
            operations (like networking and file access).
            It powers async applications by scheduling and
            managing async tasks.

    
    Traits - as seen in attribute #[derive(**Traits**)]

        Debug: 

            Allows us to print out the value of our custom type using the macros
            such as println! and format! 
            
            Example: println!("{:?}", var) or println!("{:#?}", var)

        Clone:

            Allows us to create a copy of an object, this also supports the use
            of the .clone() method which duplicates data. In the example below,
            there is no transfer of ownership, but a duplication, in this case, 
            an additional allocation on the heap (dynamic memory).

            Example: let a = SomeStruct { value: String::from("Hello")};
                     let b = a.clone()

        (Rocket) FromForm:

            Used to parse form data from HTTP requests - from form data into a 
            rust struct or other type.

            Example: #[post("/", data = "<form>")]
                     fn submit(form: Form<MyFormStruct>) {
                        // auto parse form data into MyFormStruct
                     }

        (Serde) Serialize: 

            Converts a Rust data structure (struct, enum, string, etc..)
            into another format (JSON, YAML, etc..)

            Example: let json = serde_json::to_string(&person).unwrap();

        (Serde) Deserialize:

            Converts JSON, YAML etc, into a Rust data structure (opposite of serialize).

            Example: let person: Person = serde_json::from_str(json_data).unwrap();


*/






#[macro_use] extern crate rocket;

use rocket::{tokio::sync::broadcast::{channel, Sender, error::RecvError}, serde::{Deserialize, Serialize}, State, Shutdown, response::stream::{EventStream, Event}, fs::{FileServer, relative}};
use rocket::tokio::select;
use rocket::form::Form;


#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
/*
    By specifying #[serde(crate = "rocket::serde")],
    we are telling Serde to use Rocket's version of serde instead
    of the default serde crate that might be in our Cargo.toml.
    This ensures that serialization and deserialization are handled in a
    way that is compatible with Rocket's ecosystem and settings. 
*/
#[serde(crate = "rocket::serde")] // use #[field()]
struct Message {
    #[field(validate = len(..30))] // less than 29 chars
    pub room: String,
    #[field(validate = len(..20))] // less than 19 chars
    pub username: String,
    pub message: String,
}

// this is a route that when visited returns
// the value from the 'world()' fn - its an endpoint
// #[get("/world")]
// fn world() -> &'static str {
    // The lifetime param 'static tells the 
    // runtime compiler that we want the following
    // text to last throughout the program
    // "Hello World!"
    // The reason it is not implicit is because 
    // while the &str type is mostly of lifetime
    // 'static, the program does not know if the
    // &str has been dynamically created, meaning
    // the lifetime could vary - its xtra safe
// }

// post req to the /message path and accepts form data
#[post("/message", data = "<form>")]
// Form<Message> form into the message struct
// Server state, allowing to send messages
fn post(form: Form<Message>, queue: &State<Sender<Message>>) {
    // If the send fails it is because there is no Receivers active
    // Sending the message to all receivers
    let _res = queue.send(form.into_inner());
}


// routes get requests to the events path
#[get("/events")]
// EventStream! [] is an infinite stream of server-sent events - allows clients
// to open a longlived connetion with the server and then the server can send
// data to the clients whenever it wants
// clients cant send data back to the server though

// server sent events are produced asynchronosly 
async fn events(queue: &State<Sender<Message>>, mut end: Shutdown) -> EventStream! [] {
    let mut rx = queue.subscribe();

    EventStream! {
        loop {
            let msg = select! {
                msg = rx.recv() => match msg {
                    Ok(msg) => msg,
                    Err(RecvError::Closed) => break,
                    Err(RecvError::Lagged(_)) => continue,
                },
                _ = &mut end => break,
            };
            yield Event::json(&msg);
        }
    }
}

// using the launch attribute macro, it is telling
// our program that this is the entry point for the
// program, in this case we do not need a main fn.
#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(channel::<Message>(1024).0)
        .mount("/", routes![post, events])
        .mount("/", FileServer::from(relative!("static")))
}






// not needed because of launch macro
// fn main() {
//     println!("Hello, world!");
// }
