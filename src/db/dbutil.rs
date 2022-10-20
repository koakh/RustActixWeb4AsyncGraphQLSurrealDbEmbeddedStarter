use super::{MetaData, Person};
use log::debug;
use surrealdb::sql::Value;

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

// impl Person {
//     pub fn from_value(value: Value) -> Self {
//         let mut model = Self {
//             name: String::from(""),
//             age: 28,
//             meta_data: MetaData {
//                 field: String::from(""),
//             },
//         };
//         match value {
//             Value::None => {}
//             Value::Null => {}
//             Value::False => {}
//             Value::True => {}
//             Value::Number(_number) => {}
//             Value::Strand(_strand) => {}
//             Value::Duration(_duration) => {}
//             Value::Datetime(_datetime) => {}
//             Value::Uuid(_uuid) => {}
//             Value::Array(_array) => {}
//             Value::Object(object) => {
//                 // output: value: Some(Strand(Strand("Jamie")))
//                 debug!("value: {:?}", object.get("name"));
//                 for (okey, ovalue) in object.iter() {
//                     debug!("value okey: {:?}, ovalue: {:?}", okey, ovalue);
//                     match okey.as_str() {
//                         "name" => model.name = ovalue.clone().as_string(),
//                         "age" => model.age = ovalue.clone().as_int() as u8,
//                         "meta_data" => model.meta_data = MetaData::from_value(ovalue.clone()),
//                         _ => {}
//                     }
//                 }
//             }
//             Value::Geometry(_geometry) => {}
//             Value::Param(_param) => {}
//             Value::Idiom(_idiom) => {}
//             Value::Table(_table) => {}
//             Value::Thing(_thing) => {}
//             Value::Model(_model) => {}
//             Value::Regex(_regex) => {}
//             Value::Range(_range) => {}
//             Value::Edges(_edges) => {}
//             Value::Function(_function) => {}
//             Value::Subquery(_subquery) => {}
//             Value::Expression(_expression) => {}
//         };
//         model
//     }
// }

// impl MetaData {
//     pub fn from_value(value: Value) -> Self {
//         let mut model = Self {
//             field: String::from(""),
//         };

//         if let Value::Object(object) = value {
//             for (okey, ovalue) in object.iter() {
//                 debug!("value okey: {:?}, ovalue: {:?}", okey, ovalue);
//                 // TODO: add to notes and static value must me on the left, and dynamic value on the right else it won't work
//                 if let "field" = okey.as_str() {
//                     model.field = ovalue.clone().as_string();
//                 }
//             }
//         }
//         model
//     }
// }
