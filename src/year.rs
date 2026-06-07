pub enum Year {
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
    pub fn from_paper(paper: &str) -> Option<Self> {
        match paper.chars().next()? {
            '1' => Some(Self::IA),
            '2' => Some(Self::IB),
            '3' => Some(Self::IIA),
            '4' => Some(Self::IIB),
            _ => None,
        }
    }
}
