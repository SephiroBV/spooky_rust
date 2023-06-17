use std::fmt::{Display, Formatter};

const KILOGRAMS_TO_POUNDS_RATIO: f64 = 2.20462;
const POUNDS_TO_KILOGRAMS_RATIO: f64 = 0.453592;

/// So this works, we have an enum for Weight,
/// and each variant can represent a unit of measurement for weight and their associated quantity.
/// But there's a bit of a code smell. Why can I call the function `represent_in_kilograms` if the person has already specified their weight in kilograms?
/// Essentially, if the Person has a certain 'state', then certain operations are not really applicable.
/// This also applies to the Weight enum, calling 'to_pounds' when the weight is already in pounds is not helful.

/// We can do better!
#[derive(Clone, Debug)]
pub struct Person {
    name: String,
    weight: Weight,
}

impl Person {
    pub fn new(name: impl Into<String>, weight: Weight) -> Self {
        Self {
            name: name.into(),
            weight,
        }
    }

    pub fn represent_in_kilograms(&mut self) {
        self.weight = self.weight.to_kilograms()
    }

    pub fn represent_in_pounds(&mut self) {
        self.weight = self.weight.to_pounds()
    }
}

impl Display for Person {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Name: {}, Weight: {}", self.name, self.weight)
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Weight {
    Kilogram(u16),
    Pound(u16),
}

impl Weight {
    pub fn to_pounds(self) -> Self {
        if let Weight::Kilogram(k) = self {
            let amount = f64::from(k) * KILOGRAMS_TO_POUNDS_RATIO;
            Self::Pound(amount.round() as u16)
        } else {
            self
        }
    }

    pub fn to_kilograms(self) -> Self {
        if let Weight::Pound(k) = self {
            let amount = f64::from(k) * POUNDS_TO_KILOGRAMS_RATIO;
            Self::Kilogram(amount.round() as u16)
        } else {
            self
        }
    }
}

impl Display for Weight {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Weight::Kilogram(v) => write!(f, "{v} kg"),
            Weight::Pound(v) => write!(f, "{v} pound"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        let mut sherlock = Person::new("Sherlock", Weight::Kilogram(90));
        assert_eq!("Name: Sherlock, Weight: 90 kg", sherlock.to_string());
        sherlock.represent_in_pounds();
        assert_eq!("Name: Sherlock, Weight: 198 pound", sherlock.to_string());
        sherlock.represent_in_kilograms();
        assert_eq!("Name: Sherlock, Weight: 90 kg", sherlock.to_string());
    }
}
