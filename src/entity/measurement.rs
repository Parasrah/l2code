use std::fmt;

#[derive(Debug)]
pub enum Error {
    MismatchedType,
    InvalidSubtraction,
    EmptyMeasurement,
}

#[derive(Clone)]
pub enum Measurement {
    /// Volume in mL
    Volume(usize),
    /// Weight in grams
    Weight(usize),
    /// Count
    Count(usize),
    Empty,
}


impl Measurement {
    pub fn subtract (&self, other: &Measurement) -> Result<Measurement, Error> {
        let (a, b) = self.get_values(other)?;
        if b > a {
            Err(Error::InvalidSubtraction)
        } else {
            Ok(self.new(a - b))
        }
    }

    pub fn add (&self, other: &Measurement) -> Result<Measurement, Error> {
        unimplemented!()
    }

    /// returns true if self >= other
    pub fn gte (&self, other: &Self) -> Result<bool, Error> {
        unimplemented!()
    }

    /// returns the ratio of other / self
    pub fn get_ratio (&self, other: &Self) -> Result<f32, Error> {
        let (a, b) = self.get_values(other)?;
        if a == 0 || b == 0 {
            Err(Error::EmptyMeasurement)
        } else {
            unimplemented!()
        }
    }

    fn get_values (&self, other: &Self) -> Result<(usize, usize), Error> {
        let pass_through = || Ok((self.get_value(), other.get_value()));

        match (self, other) {
            (Measurement::Empty, _) => pass_through(),
            (_, Measurement::Empty) => pass_through(),
            (Measurement::Volume(_), Measurement::Volume(_)) => pass_through(),
            (Measurement::Weight(_), Measurement::Weight(_)) => pass_through(),
            (Measurement::Count(_), Measurement::Count(_)) => pass_through(),
            // This is an impossible state (we could be subtracting/adding different
            // units, which would end up in this error state). We could have probably
            // modelled our data differently to prevent this state existing in our
            // system. Sometimes impossible state is ok. In this case, if an item
            // had a conversion from volume -> weight etc, having them measured
            // differently would be ok, and could even be considered a feature!
            _ => Err(Error::MismatchedType),
        }
    }

    fn get_value (&self) -> usize {
        match self {
            Measurement::Empty => 0,
            Measurement::Count(count) => count.clone(),
            Measurement::Weight(weight) => unimplemented!(),
            Measurement::Volume(volume) => unimplemented!(),
        }
    }

    fn new (&self, value: usize) -> Self {
        match self {
            Measurement::Empty => Measurement::Empty,
            Measurement::Count(_) => Measurement::Count(value),
            Measurement::Weight(_) => unimplemented!(),
            Measurement::Volume(_) => unimplemented!(),
        }
    }
}

impl fmt::Display for Measurement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unimplemented!()
    }
}

#[cfg(tests)]
mod tests {
    use super::*;

    #[test]
    fn subtract_works () {
        unimplemented!()
    }

    #[test]
    fn subtract_returns_invalid_subtraction () {
        unimplemented!()
    }

    #[test]
    fn subtract_returns_mismatched_type () {
        let res = Measurement::Volume(1).subtract(Measurement::Count(1));
        if let Err(err) = res {
            assert_eq!(err, Error::MismatchedType);
        } else {
            assert!(false, "should have returned error");
        }
    }

    #[test]
    fn add_works () {
        unimplemented!()
    }

    #[test]
    fn add_returns_mismatched_types () {
        unimplemented!()
    }

    #[test]
    fn gte_works () {
        unimplemented!()
    }

    #[test]
    fn gte_returns_mismatched_types () {
        unimplemented!()
    }
}
