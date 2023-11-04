use crate::extra_classes::Item;

pub struct Player {
    pub health: i32,
    pub intelligence: i32,
    pub strength: i32,
    pub inventory: Vec<Item>,
}