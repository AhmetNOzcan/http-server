use std::collections::HashMap;

pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>,
}

pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}