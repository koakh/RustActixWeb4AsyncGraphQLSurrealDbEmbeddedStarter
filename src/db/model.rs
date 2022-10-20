use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct MetaData {
    pub field: String,
}

#[derive(Debug, Serialize)]
pub struct Person {
    pub name: String,
    pub age: u8,
    pub meta_data: MetaData,
}
