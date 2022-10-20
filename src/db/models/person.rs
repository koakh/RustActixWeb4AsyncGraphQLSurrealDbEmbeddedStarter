use serde::Serialize;
use surrealdb::sql::Value;

#[derive(Debug, Serialize)]
pub struct Person {
    pub id: String,
    pub name: String,
    pub age: u8,
    pub meta_data: Option<MetaData>,
}

impl From<Value> for Person {
    fn from(value: Value) -> Self {
        let mut model = Self {
            id: String::from(""),
            name: String::from(""),
            age: 28,
            meta_data: None,
        };

        if let Value::Object(object) = value {
            for (k, v) in object.into_iter() {
                // this will convert String to &str, nice improvement, a lot cleaner
                match &k[..] {
                    "id" => model.id = v.as_string(),
                    "name" => model.name = v.as_string(),
                    "age" => model.age = v.as_int() as u8,
                    "meta_data" => model.meta_data = Some(MetaData::from(v)),
                    _ => {}
                }
            }
        }
        model
    }
}

#[derive(Debug, Serialize)]
pub struct MetaData {
    pub field: Option<String>,
}

impl From<Value> for MetaData {
    fn from(value: Value) -> Self {
        let mut model = Self {
            field: None,
        };

        if let Value::Object(object) = value {
            for (k, v) in object.into_iter() {
                if let "field" = &k[..] {
                    model.field = Some(v.as_string());
                }
            }
        }
        model
    }
}
