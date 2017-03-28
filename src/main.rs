#[macro_use]
extern crate log;
extern crate env_logger;

extern crate iron;
extern crate router;

extern crate time;

extern crate uuid;

use iron::prelude::{Chain, Iron};

use router::Router;

mod controller;
mod middleware;
mod model;

use middleware::responsetime::ResponseTime;

fn main() {
    env_logger::init().unwrap();

    let mut router = Router::new();

    router.post("/rush", controller::rush::create, "create_rush");

    let mut chain = Chain::new(router);
    chain.link_before(ResponseTime);
    chain.link_after(ResponseTime);

    let binding = "localhost:3000";
    let _server = Iron::new(chain).http(binding).unwrap();

    info!("Listening on '{}'.", binding);
}
