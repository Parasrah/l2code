use std::rc::Rc;
use std::collections::HashMap;
use super::item::Item;
use super::measurement;
use super::measurement::Measurement;
use super::shopping_list::ShoppingList;

type DesiredContents = HashMap<String, (Rc<dyn Item>, Measurement)>;

type Contents = Vec<(Rc<dyn Item>, Measurement)>;

pub struct Fridge {
    desired_contents: DesiredContents,
    contents: Contents,
}

impl Fridge {
    pub fn get_amount (&self, barcode: &str) -> Result<Measurement, measurement::Error> {
        self.contents.iter()
            .filter(|(item, _)| item.get_barcode() == barcode)
            .map(|(_, measurement)| measurement)
            .fold(Ok(Measurement::Empty), |res, other| res?.add(other))
    }

    /// Get a shopping list of items to purchase in order
    /// to meet the desired contents of the fridge
    pub fn get_shopping_list (self) -> Result<ShoppingList, measurement::Error> {
        let mut owned_items: HashMap<String, Measurement> = HashMap::new();
        for (item, amount) in self.contents.iter() {
            let key = String::from(item.get_barcode());
            if let Some(old) = owned_items.remove(&key) {
                owned_items.insert(key, old.add(&amount).unwrap());
            } else {
                owned_items.insert(key, amount.clone());
            }
        }

        let mut required_contents: Vec<(Rc<dyn Item>, Measurement)> = Vec::new();

        for (key, (item, desired_amount)) in self.desired_contents.iter() {
            if let Some(owned_amount) = owned_items.get(key) {
                if owned_amount.gte(&desired_amount)? {
                    required_contents.push((Rc::clone(item), desired_amount.subtract(&owned_amount)?))
                }
            } else {
                let clone = Rc::clone(&item);
                required_contents.push((clone, desired_amount.clone()));
            }
        }

        return Ok(ShoppingList::new(required_contents));
    }

    pub fn add (&mut self, item: Rc<dyn Item>) {

    }

    pub fn new (desired_contents: DesiredContents) -> Self {
        Fridge {
            desired_contents,
            contents: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_amount_succeeds () {

    }

    #[test]
    fn get_amount_fails () {

    }

    fn seed_fridge () -> Fridge {
        unimplemented!()
    }
}
