use std::collections::HashMap;
use rustc_serialize::json::Json;
use rustc_serialize::hex::FromHex;
use super::strip_whitespace;
use regex::Regex;

#[allow(dead_code)]
pub struct Consumer {
    pub name: String
}

#[allow(dead_code)]
pub struct Provider {
    pub name: String
}

#[derive(RustcDecodable, Debug, Clone, PartialEq)]
pub enum OptionalBody {
    Missing,
    Empty,
    Null,
    Present(String)
}

impl OptionalBody {

    pub fn is_present(&self) -> bool {
        match *self {
            OptionalBody::Present(_) => true,
            _ => false
        }
    }

    pub fn value(&self) -> String {
        match *self {
            OptionalBody::Present(ref s) => s.clone(),
            _ => s!("")
        }
    }

}

lazy_static! {
    static ref XMLREGEXP: Regex = Regex::new(r"^\s*<\?xml\s*version.*").unwrap();
    static ref HTMLREGEXP: Regex = Regex::new(r"^\s*(<!DOCTYPE)|(<HTML>).*").unwrap();
    static ref JSONREGEXP: Regex = Regex::new(r#"^\s*(true|false|null|[0-9]+|"\w*|\{\s*(}|"\w+)|\[\s*"#).unwrap();
    static ref XMLREGEXP2: Regex = Regex::new(r#"^\s*<\w+\s*(:\w+=["”][^"”]+["”])?.*"#).unwrap();
}

pub trait HttpPart {
    fn headers(&self) -> &Option<HashMap<String, String>>;
    fn body(&self) -> &OptionalBody;
    fn matching_rules(&self) -> &Option<HashMap<String, HashMap<String, String>>>;

    fn mimetype(&self) -> String {
        match *self.headers() {
            Some(ref h) => match h.iter().find(|kv| kv.0.to_lowercase() == s!("content-type")) {
                Some(kv) => match strip_whitespace::<Vec<&str>>(kv.1, ";").first() {
                    Some(v) => s!(*v),
                    None => self.detect_content_type()
                },
                None => self.detect_content_type()
            },
            None => self.detect_content_type()
        }
    }

    fn detect_content_type(&self) -> String {
        match *self.body() {
            OptionalBody::Present(ref body) => {
                let s: String = body.chars().take(32).collect();
                if XMLREGEXP.is_match(s.as_str()) {
                    s!("application/xml")
                } else if HTMLREGEXP.is_match(s.to_uppercase().as_str()) {
                    s!("text/html")
                } else if XMLREGEXP2.is_match(s.as_str()) {
                    s!("application/xml")
                } else if JSONREGEXP.is_match(s.as_str()) {
                    s!("application/json")
                } else {
                    s!("text/plain")
                }
            },
            _ => s!("text/plain")
        }
    }
}

#[derive(RustcDecodable, Debug, Clone)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub query: Option<HashMap<String, Vec<String>>>,
    pub headers: Option<HashMap<String, String>>,
    pub body: OptionalBody,
    pub matching_rules: Option<HashMap<String, HashMap<String, String>>>
}

impl HttpPart for Request {
    fn headers(&self) -> &Option<HashMap<String, String>> {
        &self.headers
    }

    fn body(&self) -> &OptionalBody {
        &self.body
    }

    fn matching_rules(&self) -> &Option<HashMap<String, HashMap<String, String>>> {
        &self.matching_rules
    }
}

fn headers_from_json(request: &Json) -> Option<HashMap<String, String>> {
    match request.find("headers") {
        Some(v) => match *v {
            Json::Object(ref m) => Some(m.iter().map(|(key, val)| {
                match val {
                    &Json::String(ref s) => (key.clone(), s.clone()),
                    _ => (key.clone(), val.to_string())
                }
            }).collect()),
            _ => None
        },
        None => None
    }
}

fn body_from_json(request: &Json) -> OptionalBody {
    match request.find("body") {
        Some(v) => match *v {
            Json::String(ref s) => {
                if s.is_empty() {
                    OptionalBody::Empty
                } else {
                    OptionalBody::Present(s.clone())
                }
            },
            Json::Null => OptionalBody::Null,
            _ => OptionalBody::Present(v.to_string())
        },
        None => OptionalBody::Missing
    }
}

impl Request {
    pub fn from_json(request: &Json) -> Request {
        let method_val = match request.find("method") {
            Some(v) => v.to_string(),
            None => "GET".to_string()
        };
        let path_val = match request.find("path") {
            Some(v) => v.to_string(),
            None => "/".to_string()
        };
        let query_val = match request.find("query") {
            Some(v) => match *v {
                Json::String(ref s) => parse_query_string(s),
                _ => None
            },
            None => None
        };
        Request {
            method: method_val,
            path: path_val,
            query: query_val,
            headers: headers_from_json(request),
            body: body_from_json(request),
            matching_rules: None
        }
    }
}

#[derive(RustcDecodable, Debug)]
pub struct Response {
    pub status: u16,
    pub headers: Option<HashMap<String, String>>,
    pub body: OptionalBody,
    pub matching_rules: Option<HashMap<String, HashMap<String, String>>>
}

impl Response {
    pub fn from_json(response: &Json) -> Response {
        let status_val = match response.find("status") {
            Some(v) => v.as_u64().unwrap() as u16,
            None => 200
        };
        Response {
            status: status_val,
            headers: headers_from_json(response),
            body: body_from_json(response),
            matching_rules: None
        }
    }
}

impl HttpPart for Response {
    fn headers(&self) -> &Option<HashMap<String, String>> {
        &self.headers
    }

    fn body(&self) -> &OptionalBody {
        &self.body
    }

    fn matching_rules(&self) -> &Option<HashMap<String, HashMap<String, String>>> {
        &self.matching_rules
    }
}

#[allow(dead_code)]
pub struct Interaction {
    pub description: String,
    pub provider_state: String,
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

fn decode_query(query: &str) -> String {
    let mut chars = query.chars();
    let mut ch = chars.next();
    let mut result = String::new();

    while ch.is_some() {
        let c = ch.unwrap();
        if c == '%' {
            let c1 = chars.next();
            let c2 = chars.next();
            match (c1, c2) {
                (Some(v1), Some(v2)) => {
                    let mut s = String::new();
                    s.push(v1);
                    s.push(v2);
                    match s.as_str().from_hex() {
                        Ok(n) => result.push(n[0] as char),
                        Err(_) => {
                            result.push('%');
                            result.push(v1);
                            result.push(v2);
                        }
                    }
                },
                (Some(v1), None) => {
                    result.push('%');
                    result.push(v1);
                },
                _ => result.push('%')
            }
        } else {
            result.push(c);
        }

        ch = chars.next();
    }

    result
}

pub fn parse_query_string(query: &String) -> Option<HashMap<String, Vec<String>>> {
    if !query.is_empty() {
        Some(query.split("&").map(|kv| {
            if !kv.is_empty() {
                kv.split("=").collect::<Vec<&str>>()
            } else {
                vec![]
            }
        }).fold(HashMap::new(), |mut map, name_value| {
            if !name_value.is_empty() {
                let name = decode_query(name_value[0]);
                let value = decode_query(name_value[1]);
                map.entry(name).or_insert(vec![]).push(value);
            }
            map
        }))
    } else {
        None
    }
}

#[cfg(test)]
mod tests;
