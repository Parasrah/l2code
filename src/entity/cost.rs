use std::fmt;
use super::measurement;
use super::measurement::Measurement;

pub struct Cost {
    // why did I choose to work with cents only, instead of keeping the cents
    // and dollars?
    // consider this question before and after you're finished with this module
    total_cents: usize,
    per_unit: Measurement,
}

impl Cost {
    pub fn combine (&self, other: &Self) -> Result<Self, measurement::Error> {
        unimplemented!()
    }

    pub fn multiply (&self, factor: f32) -> Self {
        unimplemented!()
    }

    pub fn add (&self, dollars: usize, cents: usize) -> Self {
        unimplemented!()
    }

    pub fn dollars (&self) -> usize {
        unimplemented!()
    }

    pub fn cents (&self) -> usize {
        unimplemented!()
    }
}

impl Cost {
    pub fn new (dollars: usize, cents: usize, per_unit: Measurement) -> Self {
        unimplemented!()
    }
}

impl fmt::Display for Cost {
    // how do you want your cost to look? It's totally up to you
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unimplemented!()
    }
}

// the idea of unit TDD (Test Driven Development) is that you write a skeleton like above
// and then write tests for what your module should do. If you've written good tests,
// you know when your module is done and working when all the tests are passing
// start by writing the tests in this module
#[cfg(tests)]
mod tests {
    use super::*;

    // test new

    // test dollars

    // test cents

    // test add

    // test combine
}
