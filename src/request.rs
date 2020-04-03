use std::collections::HashMap;

/// Structure for getting data into a funcktion.
///
/// The Request is normally constructed automatically by the funck server, altough it can be useful
/// to use the struct directly in unit tests. A Request is mainly composed of two things:
/// a body (As `Vec<u8>`) and metadata (As `HashMap<String, String>`).
///
/// When a funcktion is exposed as an HTTP route, the headers of the request are converted by the
/// server into request metadata.
pub struct Request {
    body: Vec<u8>,
    metadata: HashMap<String, String>,
}

impl Request {
    pub fn new(body: Vec<u8>, metadata: HashMap<String, String>) -> Request {
        Request { body, metadata }
    }

    pub fn body(&self) -> &[u8] {
        self.body.as_slice()
    }

    pub fn metadata(&self) -> &HashMap<String, String> {
        &self.metadata
    }
}
