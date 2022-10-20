use serde::Serialize;
use log::debug;
use surrealdb::sql::Value;

#[derive(Debug, Serialize)]
pub struct Person {
    pub name: String,
    pub age: u8,
    pub meta_data: MetaData,
}

impl Person {
    pub fn from_value(value: Value) -> Self {
        let mut model = Self {
            name: String::from(""),
            age: 28,
            meta_data: MetaData {
                field: String::from(""),
            },
        };

        if let Value::Object(object) = value {
            // output: value: Some(Strand(Strand("Jamie")))
            debug!("value: {:?}", object.get("name"));
            for (okey, ovalue) in object.iter() {
                debug!("value okey: {:?}, ovalue: {:?}", okey, ovalue);
                match okey.as_str() {
                    "name" => model.name = ovalue.clone().as_string(),
                    "age" => model.age = ovalue.clone().as_int() as u8,
                    "meta_data" => model.meta_data = MetaData::from_value(ovalue.clone()),
                    _ => {}
                }
            }
        }

        model
    }
}

#[derive(Debug, Serialize)]
pub struct MetaData {
    pub field: String,
}

impl MetaData {
    pub fn from_value(value: Value) -> Self {
        let mut model = Self {
            field: String::from(""),
        };

        if let Value::Object(object) = value {
            for (okey, ovalue) in object.iter() {
                debug!("value okey: {:?}, ovalue: {:?}", okey, ovalue);
                // TODO: add to notes and static value must me on the left, and dynamic value on the right else it won't work
                if let "field" = okey.as_str() {
                    model.field = ovalue.clone().as_string();
                }
            }
        }
        model
    }
}
