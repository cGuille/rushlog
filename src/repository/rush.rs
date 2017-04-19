use mysql::conn::pool::Pool;
use model::rush::Rush as Model;

pub struct Rush {
    mysql: Pool,
}

impl Rush {
    pub fn new(mysql_pool: Pool) -> Rush {
        Rush { mysql: mysql_pool }
    }

    pub fn create(&self, rush: &Model) {
        let mut stmt = self.mysql.prepare("INSERT INTO rush(uuid) VALUES(:uuid)").unwrap();

        stmt.execute(params!{
            "uuid" => rush.get_uuid(),
        }).unwrap();
    }
}
