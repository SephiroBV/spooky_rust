use std::{
    fmt::{Display, Formatter},
    marker::PhantomData,
};

const KILOGRAMS_TO_POUNDS_RATIO: f64 = 2.20462;
const POUNDS_TO_KILOGRAMS_RATIO: f64 = 0.453592;

/// Beautiful but with more leveraging the std traits. Does it help or hurt readability?
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

impl From<Person<Pound>> for Person<Kilogram> {
    fn from(person: Person<Pound>) -> Self {
        Self {
            name: person.name,
            weight: person.weight.into(),
        }
    }
}

impl From<Person<Kilogram>> for Person<Pound> {
    fn from(person: Person<Kilogram>) -> Self {
        Self {
            name: person.name,
            weight: person.weight.into(),
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

impl From<Weight<Pound>> for Weight<Kilogram> {
    fn from(weight: Weight<Pound>) -> Self {
        let amount = f64::from(weight.value) * POUNDS_TO_KILOGRAMS_RATIO;
        Weight::new(amount.round() as u16)
    }
}

impl From<Weight<Kilogram>> for Weight<Pound> {
    fn from(weight: Weight<Kilogram>) -> Self {
        let amount = f64::from(weight.value) * KILOGRAMS_TO_POUNDS_RATIO;
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
        let sherlock: Person<Pound> = sherlock.into();
        assert_eq!("Name: Sherlock, Weight: 198 pound", sherlock.to_string());
        let sherlock: Person<Kilogram> = sherlock.into();
        assert_eq!("Name: Sherlock, Weight: 90 kg", sherlock.to_string());
    }
}
