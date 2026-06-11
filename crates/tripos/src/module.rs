use crate::year::Year;

/// Just wrap a string as there are so many different modules so would be a pain to encode all in an enum
pub struct Module(String);

impl Module {
    pub fn new(name: String) -> Self {
        Self(name)
    }

    pub fn name(&self) -> &str {
        &self.0
    }
    pub fn year(&self) -> Option<Year> {
        match self.name().chars().next()? {
            '1' => Some(Year::IA),
            '2' => Some(Year::IB),
            '3' => Some(Year::IIA),
            '4' => Some(Year::IIB),
            _ => None,
        }
    }
}
