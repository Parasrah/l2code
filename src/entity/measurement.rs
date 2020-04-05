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
        let (a, b) = self.get_values(other)?;
        Ok(self.new(a + b))
    }

    pub fn gte (&self, other: &Self) -> Result<bool, Error> {
        let (a, b) = self.get_values(other)?;
        return Ok(a > b);
    }

    pub fn get_ratio (&self, other: &Self) -> Result<f32, Error> {
        let (a, b) = self.get_values(other)?;
        if a == 0 || b == 0 {
            Err(Error::EmptyMeasurement)
        } else {
            Ok((b as f32) / (a as f32))
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
            Measurement::Weight(weight) => weight.clone(),
            Measurement::Volume(volume) => volume.clone(),
        }
    }

    fn new (&self, value: usize) -> Self {
        match self {
            Measurement::Empty => Measurement::Empty,
            Measurement::Count(_) => Measurement::Count(value),
            Measurement::Weight(_) => Measurement::Weight(value),
            Measurement::Volume(_) => Measurement::Volume(value),
        }
    }
}

impl fmt::Display for Measurement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Measurement::Volume(ml) => match ml {
                0..=1000 => write!(f, "{}mL", ml),
                _ => write!(f, "{}L", (*ml as f32) / 1000.0)
            },
            Measurement::Weight(grams) => match grams {
                0..=1000 => write!(f, "{}g", grams),
                _ => write!(f, "{}kg", (*grams as f32) / 1000.0)
            },
            Measurement::Count(count) => write!(f, "{}", count),
            Measurement::Empty => write!(f, "0")
        }
    }
}
