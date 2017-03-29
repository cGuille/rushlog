use iron::request::Request;
use iron::response::Response;
use iron::IronResult;
use iron::status;

use ijr::JsonResponse;

use model::rush::Rush;

pub fn create(_: &mut Request) -> IronResult<Response> {
    let rush = Rush::new();

    // TODO save into storage
    info!("created {}.", rush);

    Ok(Response::with((status::Ok, JsonResponse::json(rush))))
}
