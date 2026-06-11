use crate::{module::Module, paper, year::Year};

pub struct Paper {
    pub module: Module,
    pub year: u32,
    pub paper_type: PaperType,
}
pub enum PaperUrl {
    QP,
    Crib,
    Together,
}

pub enum PaperType{
    TriposPaper,
    ExamplesPaper(u32),
}
impl Paper {
    pub fn new(module: Module, year: u32, paper_type: PaperType) -> Self {
        Self { module, year, paper_type}
    }

    pub fn url_tripos(&self, url_type: PaperUrl) -> Option<String> {
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

    pub fn url_examples(&self, url_type: PaperUrl, v: u32) -> Option<String> {
        // https://camcribs.com/viewer?year=IB&type=examples&module=thermofluids&id=EP01
        let year = self.module.year().unwrap_or(Year::IB);
        Some(format!(
            "https://camcribs.com/viewer?year={}&type=examples&module={}&id=EP{:02}",
            <Year as Into<&str>>::into(year).to_uppercase(),
            &self.module.name(),
            v,
        ))
    }
    
    pub fn url(&self, url_type: PaperUrl) -> Option<String>{
        match self.paper_type {
            PaperType::ExamplesPaper(v) => self.url_examples(url_type, v),
            PaperType::TriposPaper => self.url_tripos(url_type),
        }
    }


    pub fn open(&self, url_type: PaperUrl) -> Result<(), std::io::Error> {
        webbrowser::open(&self.url(url_type).expect("invalid module code"))
    }
}
