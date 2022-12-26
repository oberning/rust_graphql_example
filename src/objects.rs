use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema};


pub type SimpleQuerySchema = Schema<Query, EmptyMutation, EmptySubscription>;
pub struct Query;

#[Object]
impl Query {
    async fn hello(&self, message: String) -> String {
        // &'static str
        message
    }

    async fn find_person_by_age(&self, #[graphql(default)] age: i32) -> Option<Person> {
        person_data().into_iter().find(|x| x.age == age)
    }
}

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