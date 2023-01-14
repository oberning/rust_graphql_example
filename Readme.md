# Example with GraphQL in Rust
This is a more advanced but yet simple example for GraphQL in Rust.
For the data, SQLite and the [diesel](https://diesel.rs) framework is used.

Just perform a `cargo run` and browse to http://127.0.0.1:8000.

In the query field just enter the following example:
```
{
   findPerson(name: "von V%", age: 25)
   { forename, name, age  }
}
```
For the age the query uses `>=` (`ge`) and for the name `like` with possibility to use the SQL wildcard `%`.