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
        let method_val = match request.find("method") {
            Some(v) => v.to_string(),
            None => "GET".to_string()
        };
        let path_val = match request.find("path") {
            Some(v) => v.to_string(),
            None => "/".to_string()
        };
        Request {
            method: method_val,
            path: path_val,
            query: None,
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
    #[allow(unused_variables, dead_code)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_from_json_defaults_to_get() {
        let request_json = json!(
          {
              "path": "/",
              "query": "",
              "headers": {}
          }
        );
        let request = Request::from_json(&request_json);
        assert_eq!(request.method, "GET".to_string());
    }

    #[test]
    fn request_from_json_defaults_to_root_for_path() {
        let request_json = json!(
          {
              "method": "PUT",
              "query": "",
              "headers": {}
          }
        );
        let request = Request::from_json(&request_json);
        assert_eq!(request.path, "/".to_string());
    }

    #[test]
    fn response_from_json_defaults_to_status_200() {
        let response_json = json!(
          {
              "headers": {}
          }
        );
        let response = Response::from_json(&response_json);
        assert_eq!(response.status, 200);
    }

}
