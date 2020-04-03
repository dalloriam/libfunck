use std::collections::HashMap;
use std::io::BufWriter;

/// Structure for getting data out of a funcktion.
///
/// The Response is a way of getting data out of the funcktion. When the funcktion is exposed
/// via HTTP, the response body is sent directly as an HTTP body, and the metadata map is converted
/// to HTTP headers.
///
/// # Examples
/// ### Building a text response
/// ```
/// use funck::Response;
///
/// let my_response = Response::new().with_text(String::from("Hello, world"));
/// ```
///
/// ### Building a JSON response.
/// ```
/// use funck::Response;
///
/// use serde::Serialize;
///
/// #[derive(Serialize)]
/// struct Message {
///     pub body: String
/// }
///
/// let a_message = Message{body: String::from("Hello, world")};
/// let response = Response::new().with_bytes(serde_json::to_vec(&a_message).unwrap());
///
/// ```
pub struct Response {
    body: Vec<u8>,
    metadata: HashMap<String, String>,
}

impl Default for Response {
    fn default() -> Response {
        Response::new()
    }
}

impl Response {
    pub fn new() -> Response {
        Response {
            body: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    pub fn body(&self) -> &[u8] {
        self.body.as_ref()
    }

    pub fn with_bytes(mut self, mut bytes: Vec<u8>) -> Self {
        self.body.append(&mut bytes);
        self
    }

    pub fn with_text(mut self, data: String) -> Self {
        self.body.append(&mut data.as_bytes().to_vec());
        self
    }

    pub fn writer(&mut self) -> BufWriter<&mut Vec<u8>> {
        BufWriter::new(&mut self.body)
    }

    pub fn get_meta(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    }

    pub fn with_meta(mut self, key: &str, val: &str) -> Self {
        self.metadata.insert(String::from(key), String::from(val));
        self
    }

    pub fn metadata(&self) -> &HashMap<String, String> {
        &self.metadata
    }
}
