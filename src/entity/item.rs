use std::fmt;
use chrono::{NaiveDate,DateTime, Utc};
use super::cost::Cost;

/// An item is something that could be stored in the fridge
/// Items are identified by their barcode
///
/// For example, you might be able to buy a single apple at the
/// grocery store, or buy 12 apples together. These are different
/// items because they have different barcodes (and as a result,
/// different prices)
pub trait Item {
    fn get_name (&self) -> &str;
    /// a barcode uniquely identifies this item
    fn get_barcode (&self) -> &str;
    fn get_cost (&self) -> &Cost;
}

impl fmt::Display for dyn Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "'{}' {} {}", self.get_barcode(), self.get_name(), self.get_cost())
    }
}

pub struct NonPerishable {
    name: String,
    barcode: String,
    cost: Cost,
}

impl NonPerishable {
    pub fn new (name: String, barcode: String, cost: Cost) -> Self {
        NonPerishable { name, barcode, cost }
    }
}

impl Item for NonPerishable {
    fn get_name(&self) -> &str { &self.name }
    fn get_barcode(&self) -> &str { &self.barcode }
    fn get_cost(&self) -> &Cost { &self.cost }
}

pub struct Perishable {
    name: String,
    barcode: String,
    cost: Cost,
    expiry_date: DateTime<Utc>,
}

impl Perishable {
    pub fn new (name: String, barcode: String, cost: Cost, expiry_date: DateTime<Utc>) -> Self {
        Perishable { name, barcode, cost, expiry_date }
    }
}

impl Perishable {
    pub fn is_expired (&self, time: DateTime<Utc>) -> bool {
        time < self.expiry_date
    }
}

impl Item for Perishable {
    fn get_name(&self) -> &str { &self.name }
    fn get_barcode(&self) -> &str { &self.barcode }
    fn get_cost(&self) -> &Cost { &self.cost }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::measurement::Measurement;

    #[test]
    fn is_expired () {
        let target = Perishable::new(
            get_name(),
            get_barcode(),
            get_cost(),
            get_earlier_date(),
        );
        let result = target.is_expired(get_later_date());
        assert_eq!(result, true);
    }

    #[test]
    fn is_not_expired () {
        let target = Perishable::new(
            get_name(),
            get_barcode(),
            get_cost(),
            get_later_date(),
        );
        let result = target.is_expired(get_earlier_date());
        assert_eq!(result, false);
    }

    fn get_earlier_date () -> DateTime<Utc> {
        DateTime::<Utc>::from_utc(NaiveDate::from_ymd(2020, 1, 1).and_hms(0, 0, 0), Utc)
    }

    fn get_later_date () -> DateTime<Utc> {
        DateTime::<Utc>::from_utc(NaiveDate::from_ymd(2020, 2, 1).and_hms(0, 0, 0), Utc)
    }

    fn get_name () -> String { String::from("banana") }

    fn get_barcode () -> String { String::from("8ds90fds0") }

    fn get_cost () -> Cost { Cost::new(12, 99, Measurement::Weight(250)) }
}
