
use crate::extra_classes::Item;

pub struct Player {
    pub health: i32,
    pub intelligence: i32,
    pub strength: i32,
    pub inventory: Vec<Item>,
}

impl Player {
    pub fn get_item(&self, name: String) -> Item {
        let mut found_item: Item = Item {
            code_name: "NullItem".to_string(),
            description: "NullItem".to_string(),
            display_name: "NullItem".to_string(),
        };

        for item in &(*self).inventory {
            if item.code_name == name {
                found_item = (*item).clone();
            }
        }

        return found_item.clone();
    }

    pub fn print_inventory(&self) {
        print!("Your inventory has: ");
        if self.inventory.len() == 0 {
            println!("nothing...");
            return;
        } 

        for item in &(*self).inventory {
            
            if item.code_name == self.inventory[self.inventory.len() - 1].code_name {
                println!("{}.", item.display_name);
            } else {
                print!("{},", item.display_name);
            }
        }
    }
}