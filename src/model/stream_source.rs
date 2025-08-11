use std::collections::HashMap;

pub struct FileStreamSource {
    path: String
}

pub struct HttpStreamSource {
    url: String,
    extra_headers: HashMap<String, String>,
}

impl HttpStreamSource {
    pub fn redact() {}
}

pub struct FilterStreamSource {
    filter: String
}

pub enum StreamSource {
    File(FileStreamSource),
    Http(HttpStreamSource),
    Filter(FilterStreamSource)
}