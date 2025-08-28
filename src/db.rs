use std::env;

use diesel::prelude::*;
use diesel::{Connection, SqliteConnection};
use dotenvy::dotenv;

use crate::models::*;
use crate::schema::incomes;
use crate::schema::spendings;

pub fn init() -> SqliteConnection {
    dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL env variable not set, are you kidding me?");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub async fn getSpendings(period: Option<&str>) -> (Vec<Spending>, f32) {
    let connection = &mut init();

    let spendings_result: Vec<Spending>;
    match period {
        Some(p) => {
            spendings_result = spendings::dsl::spendings
                .filter(spendings::dsl::date.like(format!("{}%", p)))
                .select(Spending::as_select())
                .load(connection)
                .expect("Something is terribly wrong");
        }
        None => {
            spendings_result = spendings::dsl::spendings
                .select(Spending::as_select())
                .load(connection)
                .expect("Something is terribly wrong");
        }
    }

    println!("Displaying {} spendings", spendings_result.len());

    let mut total: f32 = 0.0;
    for spent in spendings_result.iter() {
        total += spent.value;
        println!("--------------");
        println!("{} - {} - {}", spent.date, spent.name, spent.value);
        println!("-------------");
    }

    (spendings_result, total)
}

pub async fn getIncomes(period: Option<&str>) -> (Vec<Income>, f32) {
    let connection = &mut init();

    let incomes_result: Vec<Income>;
    match period {
        Some(p) => {
            incomes_result = incomes::dsl::incomes
                .filter(incomes::dsl::date.like(format!("{}%", p)))
                .select(Income::as_select())
                .load(connection)
                .expect("Something is terribly wrong");
        }
        None => {
            incomes_result = incomes::dsl::incomes
                .select(Income::as_select())
                .load(connection)
                .expect("Something is terribly wrong");
        }
    }

    println!("Displaying {} spendings", incomes_result.len());

    let mut total: f32 = 0.0;
    for income in incomes_result.iter() {
        total += income.value;
        println!("--------------");
        println!("{} - {} - {}", income.date, income.name, income.value);
        println!("-------------");
    }

    (incomes_result, total)
}

pub async fn insertIncome(value: f32, date: String, name: String, category: String) -> Income {
    let income = NewIncome {
        date,
        name,
        value,
        category,
    };

    let connection = &mut init();
    diesel::insert_into(incomes::table)
        .values(&income)
        .returning(Income::as_returning())
        .get_result(connection)
        .expect("Insert resulted in a major fuckup")
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
