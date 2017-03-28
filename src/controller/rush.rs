use iron::request::Request;
use iron::response::Response;
use iron::IronResult;
use iron::status;

use model::rush::Rush;

pub fn create(_: &mut Request) -> IronResult<Response> {
    let rush = Rush::new();

    info!("created {}.", rush);
    debug!("TODO: save {} somewhere.", rush);

    Ok(Response::with((status::Ok, "I have to create a rush here.\n")))
}
