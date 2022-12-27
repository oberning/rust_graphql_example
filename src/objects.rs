use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema};

#[macro_use]
mod utils;

pub type SimpleQuerySchema = Schema<Query, EmptyMutation, EmptySubscription>;
pub struct Query;

#[Object]
impl Query {
    async fn hello(&self, message: String) -> String {
        message
    }

    async fn find_person(
        &self, 
        #[graphql(default)] forename: String,
        #[graphql(default)] name: String,
        #[graphql(default)] age: i32) -> Option<Person> {
        
        let mut person_found: Option<Person> = None;
        let is_age = bool_lambda!(age, "integer");
        let is_forename = bool_lambda!(forename, "string");
        let is_name = bool_lambda!(name, "string");
        for person in person_data().iter() {
            if is_age(person.age, age) 
                && is_forename(&person.forename, &forename)
                && is_name(&person.name, &name) {
                person_found = Some(person.clone());
                break;
            }
        }
        person_found
    }
}

#[derive(Clone)]
struct Person {
    name: String,
    forename: String,
    age: i32
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

fn person_data() -> Vec<Person> {
    vec![
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
    ]
}