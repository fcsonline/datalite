pub struct Transaction<'a> {
    pub tx: &'a rusqlite::Transaction<'a>,
}

impl<'a> Transaction<'a> {
    pub fn new (tx: &'a rusqlite::Transaction) -> Transaction<'a> {
        Transaction {
            tx: tx
        }
    }

    pub fn fact(&mut self, id: &str, attr: &str, value: &str) -> Result<(), &'a str> {
        let tx = "43223";

        self.tx.execute(
            "INSERT INTO facts (id, attr, value, fact, tx) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![id, attr, value, true, tx],
        ).expect("transaction fact error");

        Ok(())
    }

    pub fn unfact(&mut self, id: &str, attr: &str) -> Result<(), &'a str> {
        let tx = "43223";

        self.tx.execute(
            "INSERT INTO facts (id, attr, value, fact, tx) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![id, attr, "", false, tx],
        ).expect("transaction unfact error");

        Ok(())
    }
}
