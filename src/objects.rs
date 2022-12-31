use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema};
use lazy_static::lazy_static;

#[macro_use]
mod utils;

pub type SimpleQuerySchema = Schema<Query, EmptyMutation, EmptySubscription>;
pub struct Query;

lazy_static! {
    static ref PERSONS: Vec<Person> = vec![
        Person {
            name: String::from("von Riva"),
            forename: String::from("Geralt"),
            age: 100,
        },
        Person {
            name: String::from("Merigold"),
            forename: String::from("Triss"),
            age: 50,
        },
        Person {
            name: String::from("von Vengerberg"),
            forename: String::from("Yennefer"),
            age: 95,
        },
        Person {
            name: String::from("von Kaer Morhen"),
            forename: String::from("Vesemir"),
            age: 250,
        },
        Person {
            name: String::from("von Vengerberg"),
            forename: String::from("Ciri"),
            age: 25,
        },
    ];
}

#[Object]
impl Query {
    async fn hello(&self, message: String) -> String {
        message
    }

    async fn find_person(
        &self,
        #[graphql(default)] forename: String,
        #[graphql(default)] name: String,
        #[graphql(default)] age: i32,
    ) -> Vec<Person> {
        let mut persons_found = Vec::<Person>::new();
        let is_age = bool_lambda!(age, i32, 0);
        let is_forename = bool_lambda!(forename.as_str(), &str, "");
        let is_name = bool_lambda!(name.as_str(), &str, "");
        for person in PERSONS.iter() {
            if is_age(person.age, age)
                && is_forename(&person.forename, &forename)
                && is_name(&person.name, &name)
            {
                persons_found.push(person.clone());
            }
        }
        persons_found
    }
}

#[derive(Clone, Debug)]
struct Person {
    name: String,
    forename: String,
    age: i32,
}

#[Object]
impl Person {
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
