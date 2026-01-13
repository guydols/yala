use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Item {
    pub name: String,
    pub completed: bool,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GroceryList {
    pub name: String,
    pub items: Vec<Item>,
    pub show_completed: bool,
}

#[derive(Deserialize)]
pub struct CreateForm {
    pub name: String,
}

#[derive(Deserialize)]
pub struct AddItemForm {
    pub item: String,
}
