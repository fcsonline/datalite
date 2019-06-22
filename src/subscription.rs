pub struct Subscription<'a> {
    pub query: String,
    pub callback: Box<Fn(&Vec<Vec<&'a str>>)>
}
