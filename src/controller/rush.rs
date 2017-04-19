use iron::request::Request;
use iron::response::Response;
use iron::IronResult;
use iron::status;

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

pub fn fetch(_: &mut Request) -> IronResult<Response> {
    let rush = Rush::new();// TODO retrieve from storage

    info!("retrieved {}.", rush);

    Ok(Response::with((status::Ok, JsonResponse::json(rush))))
}
