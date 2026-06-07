use crate::module::Module;



pub struct Paper {
    pub module: Module,
    pub year: u32,
}

impl Paper {
    pub fn new(module: Module, year: u32) -> Self {
        Self { module, year }
    }

    pub fn to_url(&self, crib: bool) -> Option<String> {
        // https://camcribs.com/viewer?year=IA&type=tripos&module=1P1&id=CRIB_2016
        let year = self.module.year()?;
        Some(format!(
            "https://camcribs.com/viewer?year={}&type=tripos&module={}&id={}_{}",
            Into::<&str>::into(year),
            &self.module.name(),
            if crib { "CRIB" } else { "QP" },
            self.year
        ))
    }

    pub fn open(&self, crib: bool) -> Result<(), std::io::Error> {
        webbrowser::open(&self.to_url(crib).expect("invalid module code"))
    }
}
