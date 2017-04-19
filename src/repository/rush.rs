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

    pub fn find(&self, uuid: String) -> Option<Model> {
        let mut stmt = self.mysql.prepare("SELECT * FROM rush WHERE uuid = :uuid").unwrap();

        let mut result = stmt.execute(params!{
            "uuid" => uuid,
        }).unwrap();

        match result.nth(0) {
            Some(row) => Some(Model::with(row.unwrap().take("uuid").unwrap())),
            None => None,
        }
    }
}
