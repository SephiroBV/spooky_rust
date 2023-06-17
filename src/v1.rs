use std::fmt::{Display, Formatter};

/// A person struct with a primitive uint for weight. We don't know what unit it represents.
/// Kilograms? Pounds? Can we make it flexible enough to represent them both?
#[derive(Clone, Debug)]
pub struct Person {
    name: String,
    weight: u16,
}

impl Person {
    pub fn new(name: impl Into<String>, weight: u16) -> Self {
        Self {
            name: name.into(),
            weight,
        }
    }
}

impl Display for Person {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Name: {}, Weight: {}", self.name, self.weight)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        let sherlock = Person::new("Sherlock", 90);
        assert_eq!("Name: Sherlock, Weight: 90", sherlock.to_string())
    }
}
