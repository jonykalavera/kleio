#[derive(Debug)]
#[derive(Clone)]
pub struct Actor {
    id: String
    name: String
}

impl Actor {
    pub fn new(id: String, name: String) -> Actor {
        Actor {
            id,
            name
        }
    }

    pub fn id(&self) -> &String {
        &self.id
    }
}