use std::collections::HashMap;
use std::fmt;
use rustc_serialize::json::Json;

#[allow(dead_code)]
pub struct Consumer {
    pub name: String
}

#[allow(dead_code)]
pub struct Provider {
    pub name: String
}

#[derive(RustcDecodable, Debug)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub query: Option<HashMap<String, Vec<String>>>,
    pub headers: Option<HashMap<String, String>>,
    pub body: Option<String>,
    pub matching_rules: Option<HashMap<String, HashMap<String, String>>>
}

impl Request {
    #[allow(unused_variables, dead_code)]
    pub fn from_json(request: &Json) -> Request {
        Request {
            method: "".to_string(),
            path: "".to_string(),
            query: None,
            headers: None,
            body: None,
            matching_rules: None
        }
    }
}

#[derive(RustcDecodable, Debug)]
pub struct Response {
    pub status: u8,
    pub headers: Option<HashMap<String, String>>,
    pub body: Option<String>,
    pub matching_rules: Option<HashMap<String, HashMap<String, String>>>
}

impl Response {
    #[allow(unused_variables, dead_code)]
    pub fn from_json(response: &Json) -> Response {
        Response {
            status: 200,
            headers: None,
            body: None,
            matching_rules: None
        }
    }
}

#[allow(dead_code)]
pub struct Interaction {
    pub description: String,
    pub providerState: String,
    pub request: Request,
    pub response: Response
}

#[allow(dead_code)]
pub struct Pact {
    pub consumer: Consumer,
    pub provider: Provider,
    pub interations: Vec<Interaction>,
    pub metadata: HashMap<String, HashMap<String, String>>
}
