#[macro_use] extern crate serde_derive;

extern crate regex;

mod datalite;
mod datom;
mod transaction;

fn main() {
    let mut db = datalite::Datalite::new("example/database");

    db.load().expect("Unable to load previous state from filesystem");

    let query = r#"
        [
          :find ?title
          :where
          [?p :person/name "James Cameron"]
          [?m :movie/director ?p]
          [?m :movie/title ?title]
        ]
    "#;

    // Macro
    //
    // let query = datalog!(
    //     [
    //       :find ?title
    //       :where
    //       [?p :person/name "James Cameron"]
    //       [?m :movie/director ?p]
    //       [?m :movie/title ?title]
    //     ]
    // );

    let james = db.id();
    db.fact(james, ":person/name", "James Cameron").expect("Unable to insert fact");
    db.fact("13", ":person/name", "Quentin Tarantino").expect("Unable to insert fact");
    db.unfact(james, ":person/name").expect("Unable to insert unfact");

    db.transaction(|block| {
        block.fact("14", ":person/name", "Alfred Hitchcock").expect("Unable to insert fact (tx)");
        block.fact("14", ":person/name", "Alfred Hitchcock").expect("Unable to insert fact (tx)");
        block.fact("15", ":person/name", "Martin Scorsese").expect("Unable to insert fact (tx)");
        block.unfact("13", ":person/name").expect("Unable to insert fact (tx)");
    }).expect("Unable to execute transaction");

    let constraint = r#"
        {:find [name]
         :where [[p1 :username name]
                 [count(p1) < 1]]}
    "#;
    db.constraint(constraint).expect("Unable to setup a constraint");

    db.subscribe(query, |changes| {
        println!("{:?}", changes);
    }).expect("Unable to subscribe");

    let results = db.query(query);

    println!("{:?}", results);
}
