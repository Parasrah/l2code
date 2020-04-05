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
    /// get the amount of a specific item you have in the fridge, identified
    /// by its barcode
    pub fn get_amount (&self, barcode: &str) -> Result<Measurement, measurement::Error> {
        unimplemented!()
    }

    /// Get a shopping list of items to purchase in order
    /// to meet the desired contents of the fridge
    pub fn get_shopping_list (self) -> Result<ShoppingList, measurement::Error> {
        // dislaimer: I'm not great with rust, this could probably be done much better
        // given more time

        let mut owned_items: HashMap<String, Measurement> = HashMap::new();
        // we might have multiples in the fridge (2 2L skim milk w/ same barcode)
        // so we have to gather these together
        // we will use a HashMap for this
        // might find https://doc.rust-lang.org/std/collections/index.html interesting
        // gives quick overview of the different collections in rust
        for (item, amount) in self.contents.iter() {
            let key = String::from(item.get_barcode());
            if let Some(old) = owned_items.remove(&key) {
                owned_items.insert(key, old.add(&amount).unwrap());
            } else {
                owned_items.insert(key, amount.clone());
            }
        }

        // now we need to figure out what contents we need
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

    pub fn add (&mut self, item: &Rc<dyn Item>) {
        // hint: look at how Rc::clone is used above
        // why is this?
        // what are rusts ownership rules?
        // if you're interested, read https://doc.rust-lang.org/book/ch15-04-rc.html
        // what are the potential issues with reference counting? feel free to ask host
        unimplemented!()
        // btw I didn't have time to implement this one myself, so don't hesitate to ask
        // questions or change the method signature if you need to.
        // What would happen if you removed the & from item?
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
        unimplemented!()
    }

    #[test]
    fn get_amount_fails () {
        unimplemented!()
    }

    #[test]
    fn add_adds_item_to_fridge () {

    }
}
