use super::item::Item;
use super::measurement::Measurement;

pub struct ShoppingList {
    list: Vec<Entry>,
}

impl ShoppingList {
    pub fn new(items: Vec<(Box<dyn Item>, Measurement)>) -> Self {
        ShoppingList {
            list: items.into_iter().map(|(item, measurement)| Entry {
                item,
                amount: measurement,
            }).collect()
        }
    }
}

pub struct Entry {
    item: Box<dyn Item>,
    amount: Measurement,
}
