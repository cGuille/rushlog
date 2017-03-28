use std::fmt;
use uuid::Uuid;

#[derive(Debug)]
pub struct Rush {
    uuid: Uuid,
}

impl fmt::Display for Rush {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Rush[uuid: {}]", self.uuid)
    }
}

impl Rush {
    pub fn new() -> Rush {
        Rush { uuid: Uuid::new_v4() }
    }
}
