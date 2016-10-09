#[allow(unused_imports)]
use pact_matching::models::*;
#[allow(unused_imports)]
use env_logger;
#[allow(unused_imports)]
use pact_matching::match_request;
#[allow(unused_imports)]
use rustc_serialize::json::Json;
#[allow(unused_imports)]
use expectest::prelude::*;

#[test]
fn method_is_different_case() {
    env_logger::init().unwrap_or(());
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

    let expected = Request::from_json(&pact.find("expected").unwrap(), &PactSpecification::V1);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.find("actual").unwrap(), &PactSpecification::V1);
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
    env_logger::init().unwrap_or(());
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

    let expected = Request::from_json(&pact.find("expected").unwrap(), &PactSpecification::V1);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.find("actual").unwrap(), &PactSpecification::V1);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn different_method() {
    env_logger::init().unwrap_or(());
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

    let expected = Request::from_json(&pact.find("expected").unwrap(), &PactSpecification::V1);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.find("actual").unwrap(), &PactSpecification::V1);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}
