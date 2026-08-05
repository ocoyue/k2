#[derive(Debug, PartialEq, Eq)]
pub enum Request {
    Hello { name: String },
}

#[derive(Debug, PartialEq, Eq)]
pub enum Response {
    Greeting { name: String },
}