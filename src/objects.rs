use async_graphql::{EmptySubscription, Object, Schema};
use diesel::insert_into;
use diesel::prelude::*;
use diesel::SqliteConnection;
use dotenvy::dotenv;
use rust_graphql_example::schema;
use std::env;
use self::schema::persons::dsl;
use diesel::result::Error;

pub type SimpleQuerySchema = Schema<Query, Mutation, EmptySubscription>;
pub struct Query;
pub struct Mutation;

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
        //use self::schema::persons::dsl;
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
    ) -> Result<usize, Error> {
        let connection = &mut sqlite_connection();
        let person = Person {
            id: None,
            forename,
            name,
            age,
        };
        let result = insert_into(dsl::persons)
            .values(person.clone())
            .execute(connection);
        result
    }
}

#[derive(Clone, Debug, Queryable, Insertable)]
#[diesel(table_name = schema::persons)]
struct Person {
    id: Option<i32>, // Changed toid -> Nullable<Integer> in schema.rs
    name: String,
    forename: String,
    age: i32,
}

#[Object]
impl Person {
    async fn id(&self) -> Option<i32> {
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
