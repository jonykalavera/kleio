#[derive(Debug)]
#[derive(Clone)]
pub struct Entity {
    uuid: String
}

impl Entity {
    pub fn new(id: String) -> Entity {
        Entity {
            id
        }
    }

    pub fn uuid(&self) -> &String {
        &self.uuid
    }
}