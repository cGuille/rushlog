use std::fmt;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Rush {
    uuid: String,
}

impl fmt::Display for Rush {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Rush[uuid: {}]", self.uuid)
    }
}

impl Rush {
    pub fn new() -> Rush {
        Rush { uuid: Uuid::new_v4().to_string() }
    }
}
