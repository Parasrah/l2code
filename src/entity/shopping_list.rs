use super::item::Item;
use super::measurement::Measurement;
use std::fmt;
use std::rc::Rc;

pub struct ShoppingList {
    list: Vec<Entry>,
}

impl ShoppingList {
    pub fn new(items: Vec<(Rc<dyn Item>, Measurement)>) -> Self {
        ShoppingList {
            list: items.into_iter().map(|(item, measurement)| Entry {
                item,
                amount: measurement,
            }).collect()
        }
    }
}

pub struct Entry {
    item: Rc<dyn Item>,
    amount: Measurement,
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unimplemented!()
    }
}

impl fmt::Display for ShoppingList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for entry in self.list.iter() {
            writeln!(f, "{}", entry)?;
        }
        Ok(())
    }
}
