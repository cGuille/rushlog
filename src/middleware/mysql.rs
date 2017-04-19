use iron::{BeforeMiddleware, typemap};
use iron::prelude::{IronResult, Request};
use mysql::conn::pool::Pool;

pub struct PoolProvider {
    pub pool: Pool,
}

impl typemap::Key for PoolProvider {
    type Value = Pool;
}

impl BeforeMiddleware for PoolProvider {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        req.extensions.insert::<PoolProvider>(self.pool.clone());
        Ok(())
    }
}
