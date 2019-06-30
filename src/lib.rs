#![feature(proc_macro_hygiene)]
#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate rusqlite;

extern crate uuid;

use rusqlite::{Connection, Result};
use uuid::Uuid;

use crate::transaction::Transaction;
use crate::subscription::Subscription;
use crate::constraint::Constraint;

mod datom;
mod transaction;
mod subscription;
mod constraint;
mod parser;
mod executor;
mod builder;

pub struct Datalite<'a> {
    conn: Connection,
    subscriptions: Vec<Subscription<'a>>,
    constraints: Vec<Constraint>
}

impl<'a> Datalite<'a> {
    pub fn new (path: &'a str) -> Datalite {
        let conn = Connection::open(&path).expect("Unable to open database");

        Datalite {
            conn: conn,
            subscriptions: vec![],
            constraints: vec![]
        }
    }

    pub fn memory() -> Datalite<'static> {
        let conn = Connection::open_in_memory().expect("Unable to initialize memory database");

        let mut instance = Datalite {
            conn: conn,
            subscriptions: vec![],
            constraints: vec![]
        };

        instance.boot().expect("Unable to boot memory database");

        instance
    }

    pub fn boot(&mut self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE facts (
                id      TEXT NOT NULL,
                attr    TEXT NOT NULL,
                value   TEXT NOT NULL,
                fact    BOOLEAN,
                tx      TEXT NOT NULL
            )",
            params![],
        )?;

        Ok(())
    }

    pub fn listen(&mut self, address: &'a str) -> Result<()> {
        println!("Listening on {}...", address);

        Ok(())
    }

    pub fn query(&self, query: String) -> Vec<Vec<String>> {
        let lexer = parser::lexer::Lexer::new(&query).inspect(|tok| eprintln!("tok: {:?}", tok));
        let program: parser::ast::Program = parser::parser::parse(lexer).unwrap();

        executor::query(&self.conn, program.query)
    }

    pub fn fact(&mut self, id: &'a str, attr: &'a str, value: &'a str) -> Result<()> {
        let tx = "43223";

        self.conn.execute(
            "INSERT INTO facts (id, attr, value, fact, tx) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![id, attr, value, true, tx],
        )?;

        Ok(())
    }

    pub fn unfact(&mut self, id: &'a str, attr: &'a str) -> Result<()> {
        let tx = "43223";

        self.conn.execute(
            "INSERT INTO facts (id, attr, value, fact, tx) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![id, attr, "", false, tx],
        )?;

        Ok(())
    }

    pub fn subscribe<F: 'static>(&mut self, query: String, callback: F) -> Result<()>
    where F: Fn(&Vec<Vec<&'a str>>) {
        self.subscriptions.push(Subscription {
            query: query,
            callback: Box::new(callback)
        });

        Ok(())
    }

    pub fn constraint(&mut self, query: String) -> Result<()> {
        self.constraints.push(Constraint {
            query: query
        });

        Ok(())
    }

    pub fn id(&mut self) -> String {
        Uuid::new_v4().to_hyphenated().to_string()
    }

    pub fn transaction<F>(&mut self, f: F) -> Result<()>
    where F: Fn(&mut Transaction) {
        let tx = self.conn.transaction()?;
        let mut block = Transaction::new(&tx);

        f(&mut block);

        tx.commit().expect("Unable to commit transaction");

        Ok(())
    }
}
