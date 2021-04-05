#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Unit {
    Shankha,
    Padma,
    Neel,
    Kharab,
    Arab,
    Crore,
    Lakh,
    Hajaar,
    Saya,
    None,
}

#[derive(Debug)]
pub struct Scale {
    pub unit: Unit,
    pub amount: u64,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Chunk {
    pub unit: Unit,
    pub amount: u8,
}
