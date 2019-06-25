use std::collections::HashMap;

use crate::parser::ast::{Match, Query};
use crate::datom::{Datom};

// [
//   :find ?title
//   :where
//   [?p :person/name "James Cameron"]
//   [?m :movie/director ?p]
//   [?m :movie/title ?title]
// ]

pub fn query(facts: &Vec<Box<Datom>>, query: Query) -> Vec<Vec<String>> {
    let condition = query.conditions.get(0).unwrap();

    let mut ids: HashMap<String, Vec<String>> = HashMap::new();
    let mut attrs: HashMap<String, Vec<String>> = HashMap::new();
    let mut values: HashMap<String, Vec<String>> = HashMap::new();

    facts
        .iter()
        .for_each(|fact| {
            let by_id = match &condition.id {
                Match::Placeholder => true,
                Match::Binding(_) => true,
                Match::Value(ref s) => fact.id == s
            };

            let by_attr = match &condition.attr {
                Match::Placeholder => true,
                Match::Binding(_) => true,
                Match::Value(ref s) => fact.attr == s
            };

            let by_value = match &condition.value {
                Match::Placeholder => true,
                Match::Binding(_) => true,
                Match::Value(ref s) => fact.value == s
            };

            if by_id && by_attr && by_value {
                match &condition.id {
                    Match::Binding(b) => {
                        ids
                            .entry(b.to_string())
                            .or_insert(Vec::new())
                            .push(fact.id.to_string());
                    },
                    _ => {}
                };

                match &condition.attr {
                    Match::Binding(b) => {
                        attrs
                            .entry(b.to_string())
                            .or_insert(Vec::new())
                            .push(fact.attr.to_string());
                    },
                    _ => {}
                };

                match &condition.value {
                    Match::Binding(b) => {
                        values
                            .entry(b.to_string())
                            .or_insert(Vec::new())
                            .push(fact.value.to_string());
                    },
                    _ => {}
                };
            }
        });

        // let ids = ids.values().cloned().collect::<Vec<Vec<String>>>();
        // let attrs = attrs.values().cloned().collect::<Vec<Vec<String>>>();
        // let values = values.values().cloned().collect::<Vec<Vec<String>>>();

        // attrs.values().cloned().collect()
        // values.values().cloned().collect()
        ids.values().cloned().collect()
}
