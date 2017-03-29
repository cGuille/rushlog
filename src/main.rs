#[macro_use]
extern crate log;
extern crate env_logger;

extern crate iron;
extern crate router;

extern crate iron_json_response as ijr;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate time;

extern crate uuid;

use iron::prelude::{Chain, Iron};

use router::Router;

mod controller;
mod middleware;
mod model;

use middleware::responsetime::ResponseTime;
use ijr::JsonResponseMiddleware;

fn main() {
    env_logger::init().unwrap();

    let mut router = Router::new();

    router.post("/rush", controller::rush::create, "create_rush");

    let mut chain = Chain::new(router);
    chain.link_before(ResponseTime);
    chain.link_after(JsonResponseMiddleware{});
    chain.link_after(ResponseTime);

    let binding = "localhost:3000";
    let _server = Iron::new(chain).http(binding).unwrap();

    info!("Listening on '{}'.", binding);
}
