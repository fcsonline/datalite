#[macro_use] extern crate serde_derive;

use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use uuid::Uuid;

use crate::datom::Datom;
use self::transaction::Transaction;

mod datom;
mod transaction;

pub struct Datalite<'a> {
    filename: &'a str,
    facts: Vec<Box<Datom<'a>>>,
    subscriptions: Vec<&'a str>, // FIXME: This should be a hash
    constraints: Vec<&'a str>
}

impl<'a> Datalite<'a> {
    pub fn new (filename: &'a str) -> Datalite {
        Datalite {
            filename: filename,
            facts: vec![],
            subscriptions: vec![],
            constraints: vec![]
        }
    }

    pub fn load(&mut self) -> Result<()> {
        println!("Reading {}...", self.filename);

        let file = File::open(self.filename)?;

        let _lines = BufReader::new(file)
            .lines()
            .map(|line| {
                line.expect("asd")
            });

        // lines.map(|line| {
        //     serde_json::from_str(&line).expect("assf")
        // }).for_each(|datom| {
        //     self.facts.push(Box::new(datom));
        // });

        Ok(())
    }

    pub fn query(&self, _value: &'a str) -> Vec<Vec<&'a str>> {
        self.facts.iter().map(|fact| {
            vec![
                fact.id
            ]
        }).collect()
    }

    pub fn fact(&mut self, id: &'a str, attr: &'a str, value: &'a str) -> Result<()> {
        self.facts.push(Box::new(Datom {
           id: id,
           attr: attr,
           value: value,
           fact: true,
           tx: "T1"
        }));

        Ok(())
    }

    pub fn unfact(&mut self, id: &'a str, attr: &'a str) -> Result<()> {
        self.facts.push(Box::new(Datom {
           id: id,
           attr: attr,
           value: "",
           fact: false,
           tx: "T1"
        }));

        Ok(())
    }

    pub fn subscribe<F>(&mut self, value: &'a str, _f: F) -> Result<()>
    where F: Fn(&Vec<Vec<&'a str>>) {
        self.subscriptions.push(value);

        Ok(())
    }

    pub fn constraint(&mut self, value: &'a str) -> Result<()> {
        self.constraints.push(value);

        Ok(())
    }

    pub fn id(&mut self) -> String {
        Uuid::new_v4().to_hyphenated().to_string()
    }

    pub fn transaction<F>(&mut self, f: F) -> Result<()>
    where F: Fn(&mut Transaction) {
        let mut block = Transaction::new();

        f(&mut block);

        block.facts.into_iter().for_each(|fact| {
            self.facts.push(fact);
        });

        Ok(())
    }
}
