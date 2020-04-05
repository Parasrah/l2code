extern crate l2code;
extern crate rand;

use std::collections::HashMap;
use std::rc::Rc;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use chrono::{Duration,Utc};
use l2code::entity::fridge::Fridge;
use l2code::entity::measurement::Measurement;
use l2code::entity::item::*;
use l2code::entity::cost::*;

fn main () {
    let fridge = create_fridge();
    let shopping_list = fridge.get_shopping_list().unwrap();
    println!("{}", shopping_list);
}

fn create_fridge () -> Fridge {
    let bananas = create_perishable_item("bananas", Cost::new(12, 99, Measurement::Weight(500)), 14);
    let apples = create_perishable_item("apples", Cost::new(5, 79, Measurement::Weight(350)), 10);
    let spam = create_non_perishable_item("spam", Cost::new(3, 98, Measurement::Weight(200)));
    let skim_milk_1L = create_perishable_item("skim milk", Cost::new(2, 45, Measurement::Volume(1000)), 12);
    let skim_milk_2L = create_perishable_item("skim milk", Cost::new(3, 65, Measurement::Volume(2000)), 12);
    let gross_milk = create_perishable_item("2% milk", Cost::new(3, 75, Measurement::Volume(2000)), 10);
    let eggs = create_perishable_item("eggs", Cost::new(4, 99, Measurement::Count(12)), 32);

    let mut desired_contents = HashMap::new();
    insert_desired_contents(&mut desired_contents, bananas, Measurement::Weight(1000));
    insert_desired_contents(&mut desired_contents, apples, Measurement::Weight(350));
    insert_desired_contents(&mut desired_contents, spam, Measurement::Weight(300));
    insert_desired_contents(&mut desired_contents, skim_milk_2L, Measurement::Volume(8000));

    Fridge::new(desired_contents)
}

fn create_perishable_item (name: &str, cost: Cost, days: i64) -> Rc<dyn Item> {
    let expiry_date = Utc::now() + Duration::days(days);
    Rc::new(Perishable::new(String::from(name), create_barcode(), cost, expiry_date))
}

fn create_non_perishable_item (name: &str, cost: Cost) -> Rc<dyn Item> {
    Rc::new(NonPerishable::new(String::from(name), create_barcode(), cost))
}

fn insert_desired_contents (desired_contents: &mut HashMap<String, (Rc<dyn Item>, Measurement)>, item: Rc<dyn Item>, amount: Measurement) {
    desired_contents.insert(String::from(item.get_barcode()), (item, amount));
}

fn create_barcode () -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .collect()
}
