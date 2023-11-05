#[derive(Clone)]

pub struct Item {
    pub name: String,
    pub description: String,
    pub tags: Vec<String>,
}

#[derive(Clone)]
pub struct RoomObject {
    pub name: String,
    pub description: String,
}
