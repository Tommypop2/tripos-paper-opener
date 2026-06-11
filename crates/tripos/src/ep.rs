pub enum EPs{
    Mechanics,
    Structures,
    Materials,
    Thermofluids,
    Electrical,
    Information,
    Maths,
    ElMech,
    ElElectrical,
    ElInformation,
    ElBio,
    ElManufacturing
}

impl From<EPs> for &str {
    fn from(value: EPs) -> Self {
        match value {
            EPs::Mechanics => "mechanics",
            EPs::Structures => "structures",
            EPs::Materials => "materials",
            EPs::Thermofluids => "thermofluids",
            EPs::Electrical => "electrical",
            EPs::Information => "information",
            EPs::Maths => "maths",
            EPs::ElMech => "el_mech",
            EPs::ElElectrical => "el_electrical",
            EPs::ElInformation => "el_information",
            EPs::ElBio => "el_bio",
            EPs::ElManufacturing => "el_manufacturing",
        }
    }
}