use std::iter::Iterator;
use std::collections::HashMap;
use std::hash::Hash;

pub trait IteratorExtensions: Iterator {
    /*
     * You're welcome to use this code as a reference,
     * but this is a bad example of an iterator extension
     * method. This method should return an iterator, and
     * the only reason it doesn't is because I'm not skilled
     * enough in rust to know how to do that given the
     * limited time I had to get this ready for you.
     */
    fn merge_by <Key, ToKey, Merge> (self, to_key: ToKey, merge: Merge) -> Vec<Self::Item>
        where Self: Sized,
              Key: Eq + Hash,
              ToKey: Fn(&Self::Item) -> Key,
              Merge: Fn(Self::Item, Self::Item) -> Self::Item

    {
        let mut map: HashMap<Key, Self::Item> = HashMap::new();
        for item in self {
            let key = to_key(&item);
            if let Some(old) = map.remove(&key) {
                map.insert(key, merge(old, item));
            } else {
                map.insert(key, item);
            }
        }

        let mut list: Vec<Self::Item> = Vec::new();
        for (_, value) in map.into_iter() {
            list.push(value);
        }

        return list;
    }
}

// Iterator is something defined in the standard library (std::)
// and yet here we have added our own method to it. This is a
// common pattern shared by some other languages, and has
// it's own benefits and drawbacks. I'm short on time right
// now but if you're interested please ask 
impl <T: Sized> IteratorExtensions for T where T: Iterator {}
