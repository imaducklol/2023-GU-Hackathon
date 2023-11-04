use crate::item_class::Item;

pub struct Player {
    pub health: i32,
    pub intelligence: i32,
    pub strength: i32,
    pub inventory: Vec<Item>,
}