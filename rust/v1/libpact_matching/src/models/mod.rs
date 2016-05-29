use std::collections::HashMap;
use std::collections::BTreeMap;
use rustc_serialize::json::Json;
use rustc_serialize::hex::FromHex;
use super::strip_whitespace;
use regex::Regex;
use semver::Version;
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq)]
pub enum PactSpecification {
    Unknown,
    V1
}

#[derive(Debug, Clone)]
pub struct Consumer {
    pub name: String
}

impl Consumer {
    pub fn from_json(pact_json: &Json) -> Consumer {
        let val = match pact_json.find("name") {
            Some(v) => match v.clone() {
                Json::String(s) => s,
                _ => v.to_string()
            },
            None => "consumer".to_string()
        };
        Consumer { name: val.clone() }
    }
}

#[derive(Debug, Clone)]
pub struct Provider {
    pub name: String
}

impl Provider {
    pub fn from_json(pact_json: &Json) -> Provider {
        let val = match pact_json.find("name") {
            Some(v) => match v.clone() {
                Json::String(s) => s,
                _ => v.to_string()
            },
            None => "provider".to_string()
        };
        Provider { name: val.clone() }
    }
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone, PartialEq)]
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
    static ref JSONREGEXP: Regex = Regex::new(r#"^\s*(true|false|null|[0-9]+|"\w*|\{\s*(}|"\w+)|\[\s*)"#).unwrap();
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

#[derive(PartialEq, Debug, Clone)]
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

fn headers_to_json(headers: &HashMap<String, String>) -> Json {
    Json::Object(headers.iter().fold(BTreeMap::new(), |mut map, kv| {
        map.insert(kv.0.clone(), Json::String(kv.1.clone()));
        map
    }))
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

fn build_query_string(query: HashMap<String, Vec<String>>) -> String {
    query.into_iter()
        .sorted_by(|a, b| Ord::cmp(&a.0, &b.0))
        .iter()
        .flat_map(|kv| {
            kv.1.iter()
                .map(|v| format!("{}={}", kv.0, encode_query(v)))
                .collect_vec()
        })
        .join("&")
}

impl Request {
    pub fn from_json(request: &Json) -> Request {
        let method_val = match request.find("method") {
            Some(v) => match *v {
                Json::String(ref s) => s.to_uppercase(),
                _ => v.to_string().to_uppercase()
            },
            None => "GET".to_string()
        };
        let path_val = match request.find("path") {
            Some(v) => match *v {
                Json::String(ref s) => s.clone(),
                _ => v.to_string()
            },
            None => "/".to_string()
        };
        let query_val = match request.find("query") {
            Some(v) => match *v {
                Json::String(ref s) => parse_query_string(s),
                _ => parse_query_string(&v.to_string())
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

    pub fn to_json(&self) -> Json {
        let mut json = btreemap!{
            s!("method") => Json::String(self.method.to_uppercase()),
            s!("path") => Json::String(self.path.clone())
        };
        if self.query.is_some() {
            json.insert(s!("query"), Json::String(build_query_string(self.query.clone().unwrap())));
        }
        if self.headers.is_some() {
            json.insert(s!("headers"), headers_to_json(&self.headers.clone().unwrap()));
        }
        match self.body {
            OptionalBody::Present(ref body) => {
                if self.mimetype() == "application/json" {
                    match Json::from_str(body) {
                        Ok(json_body) => { json.insert(s!("body"), json_body); },
                        Err(err) => {
                            warn!("Failed to parse json body: {}", err);
                            json.insert(s!("body"), Json::String(body.clone()));
                        }
                    }
                } else {
                    json.insert(s!("body"), Json::String(body.clone()));
                }
            },
            OptionalBody::Empty => { json.insert(s!("body"), Json::String(s!(""))); },
            OptionalBody::Missing => (),
            OptionalBody::Null => { json.insert(s!("body"), Json::Null); }
        }
        Json::Object(json)
    }

    pub fn default_request() -> Request {
        Request {
            method: s!("GET"),
            path: s!("/"),
            query: None,
            headers: None,
            body: OptionalBody::Missing,
            matching_rules: None
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
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

    pub fn default_response() -> Response {
        Response {
            status: 200,
            headers: None,
            body: OptionalBody::Missing,
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

#[derive(Debug, Clone, PartialEq)]
pub struct Interaction {
    pub description: String,
    pub provider_state: Option<String>,
    pub request: Request,
    pub response: Response
}

impl Interaction {

    pub fn from_json(index: usize, pact_json: &Json) -> Interaction {
        let description = match pact_json.find("description") {
            Some(v) => match *v {
                Json::String(ref s) => s.clone(),
                _ => v.to_string()
            },
            None => format!("Interaction {}", index)
        };
        let provider_state = match pact_json.find("providerState") {
            Some(v) => match *v {
                Json::String(ref s) => if s.is_empty() {
                    None
                } else {
                    Some(s.clone())
                },
                Json::Null => None,
                _ => Some(v.to_string())
            },
            None => None
        };
        let request = match pact_json.find("request") {
            Some(v) => Request::from_json(v),
            None => Request::default_request()
        };
        let response = match pact_json.find("response") {
            Some(v) => Response::from_json(v),
            None => Response::default_response()
        };
        Interaction {
             description: description,
             provider_state: provider_state,
             request: request,
             response: response
         }
    }

}

#[derive(Debug, Clone)]
pub struct Pact {
    pub consumer: Consumer,
    pub provider: Provider,
    pub interactions: Vec<Interaction>,
    pub metadata: BTreeMap<String, BTreeMap<String, String>>
}

fn parse_meta_data(pact_json: &Json) -> BTreeMap<String, BTreeMap<String, String>> {
    match pact_json.find("metadata") {
        Some(v) => match *v {
            Json::Object(ref obj) => obj.iter().map(|(k, v)| {
                let val = match *v {
                    Json::Object(ref obj) => obj.iter().map(|(k, v)| {
                        match *v {
                            Json::String(ref s) => (k.clone(), s.clone()),
                            _ => (k.clone(), v.to_string())
                        }
                    }).collect(),
                    _ => btreemap!{}
                };
                (k.clone(), val)
            }).collect(),
            _ => btreemap!{}
        },
        None => btreemap!{}
    }
}

fn parse_interactions(pact_json: &Json) -> Vec<Interaction> {
    match pact_json.find("interactions") {
        Some(v) => match *v {
            Json::Array(ref array) => array.iter().enumerate().map(|(index, ijson)| {
                Interaction::from_json(index, ijson)
            }).collect(),
            _ => vec![]
        },
        None => vec![]
    }
}

impl Pact {

    pub fn from_json(pact_json: &Json) -> Pact {
        let metadata = parse_meta_data(pact_json);
        let consumer = match pact_json.find("consumer") {
            Some(v) => Consumer::from_json(v),
            None => Consumer { name: s!("consumer") }
        };
        let provider = match pact_json.find("provider") {
            Some(v) => Provider::from_json(v),
            None => Provider { name: s!("provider") }
        };
        Pact {
            consumer: consumer,
            provider: provider,
            interactions: parse_interactions(pact_json),
            metadata: metadata
        }
    }

    pub fn specification_version(&self) -> PactSpecification {
        match self.metadata.get("pact-specification") {
            Some(spec_ver) => match spec_ver.get("version") {
                Some(ver) => match Version::parse(ver) {
                    Ok(ver) => match ver.major {
                        1 => PactSpecification::V1,
                        _ => PactSpecification::Unknown
                    },
                    _ => PactSpecification::Unknown
                },
                None => PactSpecification::Unknown
            },
            None => PactSpecification::Unknown
        }
    }
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
        } else if c == '+' {
            result.push(' ');
        } else {
            result.push(c);
        }

        ch = chars.next();
    }

    result
}

fn encode_query(query: &str) -> String {
    query.chars().map(|ch| {
        match ch {
            ' ' => s!("+"),
            '-' => ch.to_string(),
            'a'...'z' => ch.to_string(),
            'A'...'Z' => ch.to_string(),
            '0'...'9' => ch.to_string(),
            _ => ch.escape_unicode()
                .filter(|u| u.is_digit(16))
                .batching(|mut it| {
                    match it.next() {
                        None => None,
                        Some(x) => Some((x, it.next().unwrap()))
                    }
                })
                .map(|u| format!("%{}{}", u.0, u.1))
                .collect()
        }
    }).collect()
}

pub fn parse_query_string(query: &String) -> Option<HashMap<String, Vec<String>>> {
    if !query.is_empty() {
        Some(query.split("&").map(|kv| {
            if kv.is_empty() {
                vec![]
            } else if kv.contains("=") {
                kv.split("=").collect::<Vec<&str>>()
            } else {
                vec![kv]
            }
        }).fold(HashMap::new(), |mut map, name_value| {
            if !name_value.is_empty() {
                let name = decode_query(name_value[0]);
                let value = if name_value.len() > 1 {
                    decode_query(name_value[1])
                } else {
                    s!("")
                };
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
