#[derive(PartialEq)]
pub enum PersistenceType {
    File,
    Queue,
    Pathway,
}

pub struct PersistenceAttribute {
    pub mtype: PersistenceType,
    pub value: String,
}
