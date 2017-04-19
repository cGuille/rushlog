use iron::request::Request;
use iron::response::Response;
use iron::IronResult;
use iron::status;
use router::Router;

use ijr::JsonResponse;

use middleware::mysql::PoolProvider;
use model::rush::Rush;
use repository;

pub fn create(request: &mut Request) -> IronResult<Response> {
    let mysql_pool = request.extensions.get::<PoolProvider>().unwrap().clone();
    let repo = repository::rush::Rush::new(mysql_pool);

    let rush = Rush::new();

    repo.create(&rush);
    info!("created {}.", rush);

    Ok(Response::with((status::Ok, JsonResponse::json(rush))))
}

pub fn fetch(request: &mut Request) -> IronResult<Response> {
    let mysql_pool = request.extensions.get::<PoolProvider>().unwrap().clone();
    let repo = repository::rush::Rush::new(mysql_pool);

    let uuid = request.extensions.get::<Router>().unwrap().find("uuid");

    if uuid.is_none() {
        return Ok(Response::with((status::BadRequest)));
    }

    match repo.find(uuid.unwrap().to_string()) {
        Some(rush) => Ok(Response::with((status::Ok, JsonResponse::json(rush)))),
        None => Ok(Response::with((status::NotFound))),
    }
}
