use crate::year::Year;
use crate::ep::EPs;

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

    pub fn examples_paper(&self) -> Option<EPs>{
            match self.name() {
            "mechanics" => Some(EPs::Mechanics),
            "structures" => Some(EPs::Structures),
            "materials" => Some(EPs::Materials),
            "thermofluids" => Some(EPs::Thermofluids),
            "electrical" => Some(EPs::Electrical),
            "information" => Some(EPs::Information),
            "maths" => Some(EPs::Maths),
            "el_mech" => Some(EPs::ElMech),
            "el_electrical" => Some(EPs::ElElectrical),
            "el_information" => Some(EPs::ElInformation),
            "el_bio" => Some(EPs::ElBio),
            "el_manufacturing" => Some(EPs::ElMech),
            _ => None,
        }
    }

}
