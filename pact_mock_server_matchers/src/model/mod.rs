use std::collections::HashMap;
use rustc_serialize::json::Json;
use rustc_serialize::hex::FromHex;
use std::char;

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
            headers: None,
            body: None,
            matching_rules: None
        }
    }
}

#[derive(RustcDecodable, Debug)]
pub struct Response {
    pub status: u16,
    pub headers: Option<HashMap<String, String>>,
    pub body: Option<String>,
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
            headers: None,
            body: None,
            matching_rules: None
        }
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
mod tests {
    use super::*;
    use std::collections::HashMap;
    use rustc_serialize::json::Json;

    #[test]
    fn request_from_json_defaults_to_get() {
        let request_json = Json::from_str(r#"
          {
              "path": "/",
              "query": "",
              "headers": {}
          }
        "#).unwrap();
        let request = Request::from_json(&request_json);
        assert_eq!(request.method, "GET".to_string());
    }

    #[test]
    fn request_from_json_defaults_to_root_for_path() {
        let request_json = Json::from_str(r#"
          {
              "method": "PUT",
              "query": "",
              "headers": {}
          }
        "#).unwrap();
        println!("request_json: {}", request_json);
        let request = Request::from_json(&request_json);
        assert_eq!(request.path, "/".to_string());
    }

    #[test]
    fn response_from_json_defaults_to_status_200() {
        let response_json = Json::from_str(r#"
          {
              "headers": {}
          }
        "#).unwrap();
        let response = Response::from_json(&response_json);
        assert_eq!(response.status, 200);
    }

    #[test]
    fn parse_query_string_test() {
        let query = "a=b&c=d".to_string();
        let mut expected = HashMap::new();
        expected.insert("a".to_string(), vec!["b".to_string()]);
        expected.insert("c".to_string(), vec!["d".to_string()]);
        let result = parse_query_string(&query);
        assert_eq!(result, Some(expected));
    }

    #[test]
    fn parse_query_string_handles_empty_string() {
        let query = "".to_string();
        let expected = None;
        let result = parse_query_string(&query);
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_query_string_handles_missing_values() {
        let query = "a=&c=d".to_string();
        let mut expected = HashMap::new();
        expected.insert("a".to_string(), vec!["".to_string()]);
        expected.insert("c".to_string(), vec!["d".to_string()]);
        let result = parse_query_string(&query);
        assert_eq!(result, Some(expected));
    }

    #[test]
    fn parse_query_string_decodes_values() {
        let query = "a=a%20b%20c".to_string();
        let mut expected = HashMap::new();
        expected.insert("a".to_string(), vec!["a b c".to_string()]);
        let result = parse_query_string(&query);
        assert_eq!(result, Some(expected));
    }

}
