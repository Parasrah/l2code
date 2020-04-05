use std::collections::HashMap;
use crate::extensions::iter::IteratorExtensions;
use super::item::Item;
use super::measurement;
use super::measurement::Measurement;
use super::shopping_list::ShoppingList;

type DesiredContents = HashMap<String, (Box<dyn Item>, Measurement)>;

type Contents = Vec<(Box<dyn Item>, Measurement)>;

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
    pub fn get_shopping_list (self) -> ShoppingList {
        let items = self.contents.into_iter()
            .merge_by(
                |(item, _)| String::from(item.get_barcode()),
                // right now this would panic (which is not good), but
                // I'm short of time to get this ready for you guys
                // do as I say not as I do
                |(item, a), (_, b)| (item, a.add(&b).unwrap()),
            );

        return ShoppingList::new(items);
    }

    pub fn add (&mut self, item: Box<dyn Item>) {

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
