# Example with GraphQL in Rust
This is a rather simple example for GraphQL in Rust - only using Queries.
For the data, a vector is used with the help of the `lazy_static!` macro (or crate respectively).

Worth mentioning: The creation and usage of an own macro to create a closure for a primitive search for values for the given fields in the GraphQL query.

Just perform a `cargo run` and browse to http://127.0.0.1:8000.

In the query field just enter the following example:
```
{
   findPerson(name: "von Vengerberg", age: 25)
   { forename, name, age  }
}
```

If `, age:25` is removed from the previous query, the query returns two persons.