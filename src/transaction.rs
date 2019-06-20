use crate::datom::Datom;

pub struct Transaction<'a> {
    pub facts: Vec<Box<Datom<'a>>>,
}

impl<'a> Transaction<'a> {
    pub fn new () -> Transaction<'a> {
        Transaction {
            facts: vec![]
        }
    }

    pub fn fact(&mut self, id: &'a str, attr: &'a str, value: &'a str) -> Result<(), &'a str> {
        self.facts.push(Box::new(Datom {
           id: id,
           attr: attr,
           value: value,
           fact: true,
           tx: "T1"
        }));

        Ok(())
    }

    pub fn unfact(&mut self, id: &'a str, attr: &'a str) -> Result<(), &'a str> {
        self.facts.push(Box::new(Datom {
           id: id,
           attr: attr,
           value: "",
           fact: false,
           tx: "T1"
        }));

        Ok(())
    }
}
