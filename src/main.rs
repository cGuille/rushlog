#[macro_use]
extern crate log;
extern crate env_logger;

extern crate iron;

extern crate time;

use iron::prelude::*;
use iron::status;
use iron::{BeforeMiddleware, AfterMiddleware, typemap};

use time::precise_time_ns;

struct ResponseTime;

impl typemap::Key for ResponseTime { type Value = u64; }

fn main() {
    env_logger::init().unwrap();

    impl BeforeMiddleware for ResponseTime {
        fn before(&self, req: &mut Request) -> IronResult<()> {
            req.extensions.insert::<ResponseTime>(precise_time_ns());
            Ok(())
        }
    }

    impl AfterMiddleware for ResponseTime {
        fn after(&self, req: &mut Request, res: Response) -> IronResult<Response> {
            let delta = precise_time_ns() - *req.extensions.get::<ResponseTime>().unwrap();
            info!("{} {} {}ms", req.method, req.url, (delta as f64) / 1000000.0);
            Ok(res)
        }
    }

    fn hello_world(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Hello World!\n")))
    }

    let mut chain = Chain::new(hello_world);
    chain.link_before(ResponseTime);
    chain.link_after(ResponseTime);

    let binding = "localhost:3000";
    let _server = Iron::new(chain).http(binding).unwrap();

    info!("Listening on '{}'.", binding);
}
