use serde::Serialize;
use struct_iterable::Iterable;

#[derive(Iterable, Debug, Clone, Serialize)]
pub struct Header {
    pub title: String,
    pub subtitle: String,
    pub author: String,
    pub date: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Writing {
    pub header: Header,
    pub content: String,
}
