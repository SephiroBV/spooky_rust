use std::{
    fmt::{Display, Formatter},
    marker::PhantomData,
};

const KILOGRAMS_TO_POUNDS_RATIO: f64 = 2.20462;
const POUNDS_TO_KILOGRAMS_RATIO: f64 = 0.453592;

/// Beautiful
#[derive(Clone, Debug)]
pub struct Person<T> {
    name: String,
    weight: Weight<T>,
}

impl<T> Person<T> {
    pub fn new(name: impl Into<String>, weight: u16) -> Self {
        Self {
            name: name.into(),
            weight: Weight::new(weight),
        }
    }
}

impl Person<Pound> {
    pub fn represent_in_kilograms(self) -> Person<Kilogram> {
        Person {
            name: self.name,
            weight: self.weight.to_kilograms(),
        }
    }
}

impl Person<Kilogram> {
    pub fn represent_in_pounds(self) -> Person<Pound> {
        Person {
            weight: self.weight.to_pounds(),
            name: self.name,
        }
    }
}

impl<T> Display for Person<T>
where
    Weight<T>: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Name: {}, Weight: {}", self.name, self.weight)
    }
}

pub struct Kilogram;
pub struct Pound;

#[derive(Copy, Clone, Debug)]
pub struct Weight<T> {
    _data: PhantomData<fn() -> T>,
    value: u16,
}

impl<T> Weight<T> {
    fn new(value: u16) -> Self {
        Self {
            _data: PhantomData,
            value,
        }
    }
}

impl Weight<Pound> {
    pub fn to_kilograms(self) -> Weight<Kilogram> {
        let amount = f64::from(self.value) * POUNDS_TO_KILOGRAMS_RATIO;
        Weight::new(amount.round() as u16)
    }
}

impl Weight<Kilogram> {
    pub fn to_pounds(self) -> Weight<Pound> {
        let amount = f64::from(self.value) * KILOGRAMS_TO_POUNDS_RATIO;
        Weight::new(amount.round() as u16)
    }
}

impl Display for Weight<Kilogram> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} kg", self.value)
    }
}

impl Display for Weight<Pound> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} pound", self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        let sherlock: Person<Kilogram> = Person::new("Sherlock", 90);
        assert_eq!("Name: Sherlock, Weight: 90 kg", sherlock.to_string());
        let sherlock = sherlock.represent_in_pounds();
        assert_eq!("Name: Sherlock, Weight: 198 pound", sherlock.to_string());
        let sherlock = sherlock.represent_in_kilograms();
        assert_eq!("Name: Sherlock, Weight: 90 kg", sherlock.to_string());
    }
}
