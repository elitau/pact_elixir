#[allow(unused_imports)]
use pact_matching::models::Request;
use pact_matching::match_request;
use rustc_serialize::json::Json;
use expectest::prelude::*;

#[test]
fn different_method() {
    let pact = Json::from_str(r#"
      {
        "match": false,
        "comment": "Methods is incorrect",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {}
        },
        "actual": {
          "method": "GET",
          "path": "/",
          "query": "",
          "headers": {}
      
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.find("expected").unwrap());
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.find("actual").unwrap());
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn matches() {
    let pact = Json::from_str(r#"
      {
        "match": true,
        "comment": "Methods match",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {}
        },
        "actual": {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {}
      
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.find("expected").unwrap());
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.find("actual").unwrap());
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn method_is_different_case() {
    let pact = Json::from_str(r#"
      {
        "match": true,
        "comment": "Methods case does not matter",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {}
        },
        "actual": {
          "method": "post",
          "path": "/",
          "query": "",
          "headers": {}
      
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.find("expected").unwrap());
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.find("actual").unwrap());
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}
