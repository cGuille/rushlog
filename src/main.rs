#[macro_use]
extern crate log;
extern crate env_logger;

extern crate iron;
extern crate router;

extern crate time;

use iron::prelude::*;
use iron::status;

use router::Router;

mod middleware;

use middleware::responsetime::ResponseTime;

fn main() {
    env_logger::init().unwrap();

    fn hello_world(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Hello World!\n")))
    }

    fn create_event(_: &mut Request) -> IronResult<Response> {
        debug!("TODO: create event.");
        Ok(Response::with((status::Ok, "I have to create an event here.\n")))
    }

    let mut router = Router::new();
    router.get("/", hello_world, "hello_world");
    router.post("/events", create_event, "create_event");

    let mut chain = Chain::new(router);
    chain.link_before(ResponseTime);
    chain.link_after(ResponseTime);

    let binding = "localhost:3000";
    let _server = Iron::new(chain).http(binding).unwrap();

    info!("Listening on '{}'.", binding);
}
