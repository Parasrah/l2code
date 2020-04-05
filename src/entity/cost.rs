use std::fmt;
use super::measurement;
use super::measurement::Measurement;

pub struct Cost {
    total_cents: usize,
    per_unit: Measurement,
}

impl Cost {
    pub fn combine (&self, other: &Self) -> Result<Self, measurement::Error> {
        match (&self.per_unit, &other.per_unit) {
            (Measurement::Empty, _) => Err(measurement::Error::EmptyMeasurement),
            (_, Measurement::Empty) => Err(measurement::Error::EmptyMeasurement),
            (a, b) => {
                let ratio = a.get_ratio(&b)?;
                let adjusted = other.multiply(ratio);
                return Ok(Self{
                    total_cents: self.total_cents + adjusted.total_cents,
                    per_unit: self.per_unit.clone(),
                });
            },
        }
    }

    pub fn multiply (&self, factor: f32) -> Self {
        let total_cents = ((self.total_cents as f32) * factor).round().abs() as usize;

        Self {
            total_cents,
            per_unit: self.per_unit.clone(),
        }
    }

    pub fn add (&self, dollars: usize, cents: usize) -> Self {
        Self {
            total_cents: self.total_cents + 100 * dollars + cents,
            per_unit: self.per_unit.clone(),
        }
    }

    pub fn dollars (&self) -> usize {
        self.total_cents / 100
    }

    pub fn cents (&self) -> usize {
        self.total_cents % 100
    }
}

impl Cost {
    pub fn new (dollars: usize, cents: usize, per_unit: Measurement) -> Self {
        Cost {
            total_cents: dollars * 100 + cents,
            per_unit,
        }
    }
}

impl fmt::Display for Cost {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "${}.{}/{}", self.dollars(), self.cents(), self.per_unit)
    }
}
