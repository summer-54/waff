use std::ops::Deref;

pub struct Token(pub Box<str>);
impl Deref for Token {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
