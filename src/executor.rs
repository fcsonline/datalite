use crate::parser::ast::Query;
use crate::builder;
use rusqlite::{Connection};

pub fn query(conn: &Connection, query: Query) -> Vec<Vec<String>> {
    let _condition = query.conditions.get(0).unwrap();
    let sql = builder::build(&query);

    let mut stmt = conn.prepare(sql.as_str()).expect("Query error");
    let iter = stmt.query_map(params![], |row| {
        let size = query.bindings.len();
        let mut item: Vec<String> = Vec::with_capacity(size as usize);

        for i in 0..size {
            item.push(row.get(i).expect("unable to get field"))
        }

        Ok(item)
    }).expect("Values");

    iter.map(|row| { row.unwrap() }).collect()
}
