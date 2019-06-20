use std::fmt;

#[derive(Serialize, Deserialize)]
pub struct Datom<'a> {
    pub id: &'a str,
    pub attr: &'a str,
    pub value: &'a str,
    pub fact: bool,
    pub tx: &'a str
}

impl<'a> fmt::Display for Datom<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.id, self.attr, self.value)
    }
}
