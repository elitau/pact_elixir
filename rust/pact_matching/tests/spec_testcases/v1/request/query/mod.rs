#[allow(unused_imports)]
use pact_matching::models::*;
#[allow(unused_imports)]
use pact_matching::match_request;
#[allow(unused_imports)]
use rustc_serialize::json::Json;
#[allow(unused_imports)]
use expectest::prelude::*;

#[test]
fn trailing_amperand() {
    let pact = Json::from_str(r#"
      {
        "match": true,
        "comment": "Query strings are matched using basic string equality, these are not equal. (not supported)",
        "expected" : {
          "method": "GET",
          "path": "/path",
          "query": "alligator=Mary&hippo=John",
          "headers": {}
      
        },
        "actual": {
          "method": "GET",
          "path": "/path",
          "query": "alligator=Mary&hippo=John&",
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
    let pact = Json::from_str(r#"
      {
        "match": true,
        "comment": "Queries are the same",
        "expected" : {
          "method": "GET",
          "path": "/path",
          "query": "alligator=Mary&hippo=John",
          "headers": {}
      
        },
        "actual": {
          "method": "GET",
          "path": "/path",
          "query": "alligator=Mary&hippo=John",
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
fn different_param_values() {
    let pact = Json::from_str(r#"
      {
        "match": false,
        "comment": "Queries are not the same - hippo is Fred instead of John",
        "expected" : {
          "method": "GET",
          "path": "/path",
          "query": "alligator=Mary&hippo=John",
          "headers": {}
      
        },
        "actual": {
          "method": "GET",
          "path": "/path",
          "query": "alligator=Mary&hippo=Fred",
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
fn different_param_order() {
    let pact = Json::from_str(r#"
      {
        "match": true,
        "comment": "Query strings are matched using basic string equality, these are not equal. (Not supported)",
        "expected" : {
          "method": "GET",
          "path": "/path",
          "query": "alligator=Mary&hippo=John",
          "headers": {}
      
        },
        "actual": {
          "method": "GET",
          "path": "/path",
          "query": "hippo=John&alligator=Mary",
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
