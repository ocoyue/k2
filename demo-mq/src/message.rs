#[derive(Debug,Clone,PartialEq)]
pub struct Message {
    pub body: String,
}
impl Message {
    pub fn new(body: impl Into<String>) -> Message {
        Self { body: body.into() }
    }
}