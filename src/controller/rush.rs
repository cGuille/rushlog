use iron::request::Request;
use iron::response::Response;
use iron::IronResult;
use iron::status;

pub fn create(_: &mut Request) -> IronResult<Response> {
    debug!("TODO: create rush.");
    Ok(Response::with((status::Ok, "I have to create a rush here.\n")))
}
