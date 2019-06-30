use crate::parser::ast::{Match, Query};

// [
//   :find ?title
//   :where
//   [?p :person/name "James Cameron"]
//   [?m :movie/director ?p]
//   [?m :movie/title ?title]
// ]

pub fn build(query: &Query) -> String {
    let _condition = query.conditions.get(0).unwrap();
    let sql = "
        SELECT datom3.value as title, datom1.id as id
        FROM facts as datom1, facts as datom2, facts as datom3
        WHERE
            datom1.attr = ':person/name' AND
            datom1.value = 'James Cameron' AND

            datom2.attr = ':movie/director' AND

            datom3.attr = ':movie/title' AND

            datom1.id = datom2.value AND
            datom2.id = datom3.id

    ";

    sql.to_string()

    // facts
    //     .iter()
    //     .for_each(|fact| {
    //         let by_id = match &condition.id {
    //             Match::Placeholder => true,
    //             Match::Binding(_) => true,
    //             Match::Value(ref s) => fact.id == s
    //         };

    //         let by_attr = match &condition.attr {
    //             Match::Placeholder => true,
    //             Match::Binding(_) => true,
    //             Match::Value(ref s) => fact.attr == s
    //         };

    //         let by_value = match &condition.value {
    //             Match::Placeholder => true,
    //             Match::Binding(_) => true,
    //             Match::Value(ref s) => fact.value == s
    //         };

    //         if by_id && by_attr && by_value {
    //             match &condition.id {
    //                 Match::Binding(b) => {
    //                     ids
    //                         .entry(b.to_string())
    //                         .or_insert(Vec::new())
    //                         .push(fact.id.to_string());
    //                 },
    //                 _ => {}
    //             };

    //             match &condition.attr {
    //                 Match::Binding(b) => {
    //                     attrs
    //                         .entry(b.to_string())
    //                         .or_insert(Vec::new())
    //                         .push(fact.attr.to_string());
    //                 },
    //                 _ => {}
    //             };

    //             match &condition.value {
    //                 Match::Binding(b) => {
    //                     values
    //                         .entry(b.to_string())
    //                         .or_insert(Vec::new())
    //                         .push(fact.value.to_string());
    //                 },
    //                 _ => {}
    //             };
    //         }
    //     });



}
