#![feature(proc_macro_hygiene)]

extern crate regex;
extern crate datalite;
extern crate plex;

use datalite::Datalite;

fn main() {
    let mut db = Datalite::memory();

    let query = r#"
        [
          :find ?title ?id
          :where
          [?id :person/name "James Cameron"]
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

    let _james = db.id();

    db.fact("12", ":person/name", "James Cameron").expect("Unable to insert fact");
    db.fact("13", ":person/name", "Quentin Tarantino").expect("Unable to insert fact");
    db.unfact("12", ":person/name").expect("Unable to insert unfact");

    db.transaction(|block| {
        block.fact("14", ":person/name", "Alfred Hitchcock").expect("Unable to insert fact (tx)");
        block.fact("14", ":person/name", "Alfred Hitchcock").expect("Unable to insert fact (tx)");
        block.fact("15", ":person/name", "Martin Scorsese").expect("Unable to insert fact (tx)");
        block.unfact("13", ":person/name").expect("Unable to insert fact (tx)");
    }).expect("Unable to execute transaction");

    db.fact("18", ":movie/title", "Titanic").expect("Unable to insert fact");
    db.fact("18", ":movie/director", "12").expect("Unable to insert fact");
    db.fact("19", ":movie/title", "Kill Bill").expect("Unable to insert fact");
    db.fact("19", ":movie/director", "13").expect("Unable to insert fact");

    let constraint = r#"
        {:find [name]
         :where [[p1 :username name]
                 [count(p1) < 1]]}
    "#;
    db.constraint(constraint.to_string()).expect("Unable to setup a constraint");

    db.subscribe(query.to_string(), |changes| {
        println!("{:?}", changes);
    }).expect("Unable to subscribe");

    let results = db.query(query.to_string());

    println!("{:?}", results);

    db.listen("0.0.0.0:18081").expect("Unable to listen");
}
