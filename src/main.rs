#[macro_use]
extern crate log;
extern crate env_logger;

extern crate iron;

use iron::prelude::*;
use iron::status;

fn main() {
    env_logger::init().unwrap();

    fn hello_world(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Hello World!\n")))
    }

    let binding = "localhost:3000";
    let _server = Iron::new(hello_world).http(binding).unwrap();

    info!("Listening on '{}'.", binding);
}
