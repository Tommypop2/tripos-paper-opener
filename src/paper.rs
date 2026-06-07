use crate::module::Module;

pub struct Paper {
    pub module: Module,
    pub year: u32,
}
pub enum PaperUrl {
    QP,
    Crib,
    Together,
}
impl Paper {
    pub fn new(module: Module, year: u32) -> Self {
        Self { module, year }
    }

    pub fn url(&self, url_type: PaperUrl) -> Option<String> {
        // https://camcribs.com/viewer?year=IA&type=tripos&module=1P1&id=CRIB_2016
        let year = self.module.year()?;
        Some(format!(
            "https://camcribs.com/viewer?year={}&type=tripos&module={}&id={}_{}{}",
            Into::<&str>::into(year).to_uppercase(),
            &self.module.name().to_uppercase(),
            match url_type {
                PaperUrl::QP | PaperUrl::Together => "QP",
                PaperUrl::Crib => "CRIB",
            },
            self.year,
            match url_type {
                PaperUrl::Together => "&both=true",
                _ => "",
            }
        ))
    }

    pub fn open(&self, url_type: PaperUrl) -> Result<(), std::io::Error> {
        webbrowser::open(&self.url(url_type).expect("invalid module code"))
    }
}
