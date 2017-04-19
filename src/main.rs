#[macro_use]
extern crate log;
extern crate env_logger;

extern crate iron;
#[macro_use]
extern crate mysql;
extern crate router;

extern crate iron_json_response as ijr;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate time;

extern crate uuid;

use iron::prelude::{Chain, Iron};
use mysql::conn::pool::Pool;
use router::Router;

mod controller;
mod middleware;
mod model;
mod repository;

use middleware::mysql::PoolProvider;
use middleware::responsetime::ResponseTime;
use ijr::JsonResponseMiddleware;

fn main() {
    env_logger::init().unwrap();

    let mysql_pool = Pool::new("mysql://rushlog:rushlog@tea-db/rushlog").unwrap();
    let mut router = Router::new();

    router.get("/rush", controller::rush::fetch, "fetch_rush");
    router.post("/rush", controller::rush::create, "create_rush");

    let mut chain = Chain::new(router);
    chain.link_before(ResponseTime);
    chain.link_before(PoolProvider { pool: mysql_pool.clone() });
    chain.link_after(JsonResponseMiddleware {});
    chain.link_after(ResponseTime);

    let port = match std::env::var("PORT") {
        Ok(port) => port.parse::<u16>().unwrap(),
        Err(_) => 3000,
    };
    let binding = ("localhost", port);

    let _server = Iron::new(chain).http(binding).unwrap();

    info!("Listening on {:?}.", binding);
}
