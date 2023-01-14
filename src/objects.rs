use async_graphql::{EmptySubscription, Object, Schema};
use diesel::prelude::*;
use diesel::SqliteConnection;
use dotenvy::dotenv;
use lazy_static::lazy_static;
use rust_graphql_example::schema;
use std::env;

pub type SimpleQuerySchema = Schema<Query, Mutation, EmptySubscription>;
pub struct Query;
pub struct Mutation;


lazy_static! {
    static ref PERSONS: Vec<Person> = vec![
        Person {
            id: 0,
            name: String::from("von Riva"),
            forename: String::from("Geralt"),
            age: 100,
        },
        Person {
            id: 1,
            name: String::from("Merigold"),
            forename: String::from("Triss"),
            age: 50,
        },
        Person {
            id: 2,
            name: String::from("von Vengerberg"),
            forename: String::from("Yennefer"),
            age: 95,
        },
        Person {
            id: 3,
            name: String::from("von Kaer Morhen"),
            forename: String::from("Vesemir"),
            age: 250,
        },
        Person {
            id: 4,
            name: String::from("von Vengerberg"),
            forename: String::from("Ciri"),
            age: 25,
        },
    ];
}

pub fn sqlite_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in the .env file");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[Object]
impl Query {
    async fn find_person(
        &self,
        #[graphql(default)] forename: String,
        #[graphql(default)] name: String,
        #[graphql(default)] age: i32,
    ) -> Vec<Person> {
        use self::schema::persons::dsl;
        let connection = &mut sqlite_connection();
        let mut query = dsl::persons.into_boxed();
        if name != "" { query = query.filter(dsl::name.like(name)) };
        if forename != "" { query = query.filter(dsl::forename.eq(forename)) };
        if age !=0 { query = query.filter(dsl::age.ge(age)) };
        let results = query
            .load::<Person>(connection)
            .expect("Error loading persons");
        results
    }
}

#[Object]
impl Mutation {
    async fn create_person(
        &self,
        #[graphql(default)] forename: String,
        #[graphql(default)] name: String,
        #[graphql(default)] age: i32,
    ) -> Vec<Person> {
        let person = Person {
            id: 1000,
            forename,
            name,
            age,
        };
        let mut persons = PERSONS.to_owned();
        persons.push(person);
        persons
    }
}

#[derive(Clone, Debug, Queryable)]
struct Person {
    id: i32,
    name: String,
    forename: String,
    age: i32,
}

#[Object]
impl Person {
    async fn id(&self) -> i32 {
        self.id
    }

    async fn name(&self) -> &str {
        &self.name
    }

    async fn forename(&self) -> &str {
        &self.forename
    }

    async fn age(&self) -> i32 {
        self.age
    }
}
