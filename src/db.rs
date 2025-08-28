use std::env;

use diesel::prelude::*;
use diesel::{Connection, SqliteConnection};
use dotenvy::dotenv;

use crate::models::*;
use crate::schema::spendings;

pub fn init() -> SqliteConnection {
    dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL env variable not set, are you kidding me?");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub async fn getSpendings() {
    let connection = &mut init();
    let spendings_result = spendings::dsl::spendings
        .select(Spending::as_select())
        .load(connection)
        .expect("Something is terribly wrong");

    println!("Displaying {} spendings", spendings_result.len());

    let mut total: f32 = 0.0;
    for spent in spendings_result {
        total += spent.value;
        println!("--------------");
        println!("{} - {} - {}", spent.date, spent.name, spent.value);
        println!("-------------");
    }

    println!("total: {}", total.to_string());
}

pub async fn insertSpending(value: f32, date: String, name: String, category: String) -> Spending {
    let spent = NewSpent {
        date,
        name,
        value,
        category,
    };
    let connection = &mut init();
    diesel::insert_into(spendings::table)
        .values(&spent)
        .returning(Spending::as_returning())
        .get_result(connection)
        .expect("Insert resulted in a major fuckup")
}
