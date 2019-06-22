# Datalite

## What is Datalite?

:warning: This project is just a POC and is not production ready!

An immutable in-memory database and Datalog query engine written in Rust.

- Flexible database based schemaless store
- Immutable database with timetraveling features
- Subscription base database for realtime web applications

Datalite is inspired in many features from Datomic/Datalog, Crux and many
others. Datalite tries to simplify all the infrastructure requirement needed to
achive this kind of databases.

## Usage

Put this in your Cargo.toml:

```
[dependencies]
datalite = "0.1.0"
```

And then start using it:

```rust
let mut db = Datalite::new("example/database");

db.fact("12", ":person/name", "James Cameron")?;
db.fact("13", ":person/name", "Quentin Tarantino")?;
db.unfact("12", ":person/name")?;

db.transaction(|block| {
    block.fact("14", ":person/name", "Alfred Hitchcock")?;
    block.fact("14", ":person/name", "Alfred Hitchcock")?;
    block.fact("15", ":person/name", "Martin Scorsese")?;
    block.unfact("13", ":person/name")?;
})?;

let constraint = r#"
    {:find [name]
     :where [[p1 :username name]
             [count(p1) < 1]]}
"#;
db.constraint(constraint)?;

db.subscribe(query, |changes| {
    println!("{:?}", changes);
})?;

let query = r#"
    [
      :find ?title
      :where
      [?p :person/name "James Cameron"]
      [?m :movie/director ?p]
      [?m :movie/title ?title]
    ]
"#;

let results = db.query(query);

println!("{:?}", results);
```

## API

`query(q: String)`: Retrieve information from the database

`id() -> String`: Generate a new random id for facts

`fact(id: String, attr: String, value: String)`: Add new facts to the database for a specific id and attribute

`unfact(id: String, attr: String)`: Remove facts from the database for a specific id and attribute

`transaction`: Execute statements in block

`constraint(q: String)`: Add constraints to the database to avoid data violations

`subscribe(q: String, |changes| {} )`: Subscribe to interesting data changes to be pushed to exernal services

## Use cases

`TODO`

## Examples

- Datalite integration with a Rocket.rs server: [link](https://github.com/fcsonline/datarocket)
