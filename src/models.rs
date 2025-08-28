use crate::schema::spendings;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::spendings)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Spending {
    pub id: i32,
    pub value: f32,
    pub date: String,
    pub name: String,
    pub category: String,
}

#[derive(Insertable)]
#[diesel(table_name = spendings)]
pub struct NewSpent {
    pub value: f32,
    pub date: String,
    pub name: String,
    pub category: String,
}
