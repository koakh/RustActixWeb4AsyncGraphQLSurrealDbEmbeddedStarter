use std::collections::BTreeMap;

use surrealdb::sql::{Id, Number, Strand, Thing, Value};

use super::InputFilter as PersonInputFilter;

// TODO: this is related to person, move to person model

// this will mutate ast and vars
// TODO add to default trail implementation for models
pub fn add_filter_to_ast(
    table: String,
    filter: &Option<PersonInputFilter>,
    ast: &mut String,
    vars: &mut BTreeMap<String, Value>,
) {
    if let Some(f) = filter {
        let mut filter_fields: Vec<&str> = Vec::new();
        // id
        if let Some(v) = &f.id {
            filter_fields.push("id = $id");
            vars.insert(
                "id".to_string(),
                // thing(format!("{}:{}", "person", v).as_str()),
                // TODO:: you can use `surrealdb::sql::thing("table:id")` instead of manually constructing `Value::Thing`
                Value::Thing(Thing {
                    tb: table,
                    id: { Id::String(v.to_string()) },
                }),
            );
        }
        // name
        if let Some(v) = &f.name {
            filter_fields.push("name = $name");
            vars.insert("name".to_string(), Value::Strand(Strand(v.to_string())));
        }
        // age
        if let Some(v) = &f.age {
            filter_fields.push("age = $age");
            vars.insert("age".to_string(), Value::Number(Number::Int(*v as i64)));
        }
        // loop filter fields and inject conditions on ast
        for (i, el) in filter_fields.iter().enumerate() {
            // if is first where condition prefix with WHERE
            if i == 0 {
                ast.push_str(" WHERE ");
            }
            // add where condition
            if i > 0 {
                ast.push_str(" AND ");
            }
            ast.push_str(el);
        }
    }
}

// old dbutil with full surrealdb::sql::Value match values with all match arms
// before refactor all code to be inside src/db/models/person.rs

// use super::{MetaData, Person};
// use log::debug;
// use surrealdb::sql::Value;

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
