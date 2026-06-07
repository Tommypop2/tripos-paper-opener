enum Year {
    IA,
    IB,
    IIA,
    IIB,
}
impl From<Year> for &str {
    fn from(value: Year) -> Self {
        match value {
            Year::IA => "IA",
            Year::IB => "IB",
            Year::IIA => "IIA",
            Year::IIB => "IIB",
        }
    }
}
impl Year {
    fn from_paper(paper: &str) -> Option<Self> {
        match paper.chars().next()? {
            '1' => Some(Self::IA),
            '2' => Some(Self::IB),
            '3' => Some(Self::IIA),
            '4' => Some(Self::IIB),
            _ => None,
        }
    }
}
pub struct Paper {
    pub module: String,
    pub year: u32,
}

impl Paper {
    pub fn new(module: String, year: u32) -> Self {
        Self { module, year }
    }

    pub fn to_url(&self, crib: bool) -> Option<String> {
        // https://camcribs.com/viewer?year=IA&type=tripos&module=1P1&id=CRIB_2016
        let year = Year::from_paper(&self.module)?;
        Some(format!(
            "https://camcribs.com/viewer?year={}&type=tripos&module={}&id={}_{}",
            Into::<&str>::into(year),
            &self.module,
            if crib { "CRIB" } else { "QP" },
            self.year
        ))
    }

    pub fn open(&self, crib: bool) -> Result<(), std::io::Error> {
        webbrowser::open(&self.to_url(crib).expect("invalid module code"))
    }
}
