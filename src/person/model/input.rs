use async_graphql::InputObject;

// TODO:
// use crate::user::scalar::Id;

#[derive(InputObject)]
pub struct InputFilter {
    /// The ID of the Person
    pub id: Option<String>,
    /// The name for the Person.
    pub name: Option<String>,
    /// The age of the Person
    pub age: Option<i8>,
}

// #[derive(InputObject)]
// pub struct CreatePersonInput {
//     /// The name for the Person.
//     pub name: String,
//     /// The age of the Person
//     pub age: Option<i8>,
// }

// #[derive(InputObject)]
// pub struct UpdatePersonInput {
//     /// The ID of the Person to modify.
//     pub id: Id,
//     /// The name for the Person.
//     pub name: String,
//     /// The age of the Person
//     pub age: Option<i8>,
// }

// #[derive(InputObject)]
// pub struct DeletePersonInput {
//     /// The ID of the Person to delete.
//     pub id: Id,
// }
