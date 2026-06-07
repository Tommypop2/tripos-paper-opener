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
impl From<u32> for Year {
    fn from(value: u32) -> Self {
        match value {
            1 => Self::IA,
            2 => Self::IB,
            3 => Self::IIA,
            4 => Self::IIB,
            _ => panic!("invalid year"),
        }
    }
}
