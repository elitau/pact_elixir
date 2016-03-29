use super::*;
use std::collections::HashMap;
use rustc_serialize::json::Json;
use expectest::prelude::*;

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

#[test]
fn request_mimetype_is_based_on_the_content_type_header() {
    let request = Request { method: s!("GET"), path: s!("/"), query: None, headers: None,
        body: OptionalBody::Missing, matching_rules: None };
    expect!(request.mimetype()).to(be_equal_to("text/plain"));
    expect!(Request {
        headers: Some(hashmap!{ s!("Content-Type") => s!("text/html") }), .. request.clone() }.mimetype())
        .to(be_equal_to("text/html"));
    expect!(Request {
        headers: Some(hashmap!{ s!("Content-Type") => s!("application/json; charset=UTF-8") }), .. request.clone() }.mimetype())
        .to(be_equal_to("application/json"));
    expect!(Request {
        headers: Some(hashmap!{ s!("Content-Type") => s!("application/json") }), .. request.clone() }.mimetype())
        .to(be_equal_to("application/json"));
    expect!(Request {
        headers: Some(hashmap!{ s!("CONTENT-TYPE") => s!("application/json ; charset=UTF-8") }), .. request.clone() }.mimetype())
        .to(be_equal_to("application/json"));
    expect!(Request {
        body: OptionalBody::Present(s!("{\"json\": true}")), .. request.clone() }.mimetype())
        .to(be_equal_to("application/json"));
    expect!(Request {
        body: OptionalBody::Present(s!("{}")), .. request.clone() }.mimetype())
        .to(be_equal_to("application/json"));
    expect!(Request {
        body: OptionalBody::Present(s!("[]")), .. request.clone() }.mimetype())
        .to(be_equal_to("application/json"));
    expect!(Request {
        body: OptionalBody::Present(s!("[1,2,3]")), .. request.clone() }.mimetype())
        .to(be_equal_to("application/json"));
    expect!(Request {
        body: OptionalBody::Present(s!("\"string\"")), .. request.clone() }.mimetype())
        .to(be_equal_to("application/json"));
    expect!(Request {
        body: OptionalBody::Present(s!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<json>false</json>")), .. request.clone() }.mimetype())
        .to(be_equal_to("application/xml"));
    expect!(Request {
        body: OptionalBody::Present(s!("<json>false</json>")), .. request.clone() }.mimetype())
        .to(be_equal_to("application/xml"));
    expect!(Request {
        body: OptionalBody::Present(s!("this is not json")), .. request.clone() }.mimetype())
        .to(be_equal_to("text/plain"));
    expect!(Request {
        body: OptionalBody::Present(s!("<html><body>this is also not json</body></html>")), .. request.clone() }.mimetype())
        .to(be_equal_to("text/html"));
}
