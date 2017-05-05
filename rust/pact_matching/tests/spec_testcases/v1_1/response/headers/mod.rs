#[allow(unused_imports)]
use pact_matching::models::*;
#[allow(unused_imports)]
use env_logger;
#[allow(unused_imports)]
use pact_matching::match_response;
#[allow(unused_imports)]
use expectest::prelude::*;
use serde_json;

#[test]
fn whitespace_after_comma_different() {
    env_logger::init().unwrap_or(());
    let pact : serde_json::Value = serde_json::from_str(r#"
      {
        "match": true,
        "comment": "Whitespace between comma separated headers does not matter",
        "expected" : {
          "headers": {
            "Accept": "alligators,hippos"
          }
        },
        "actual": {
          "headers": {
            "Accept": "alligators, hippos"
          }
        }
      }
    "#).unwrap();

    let expected = Response::from_json(&pact.get("expected").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.get("actual").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    let result = match_response(expected, actual);
    if pact_match.as_bool().unwrap() {
       expect!(result).to(be_empty());
    } else {
       expect!(result).to_not(be_empty());
    }
}

#[test]
fn unexpected_header_found() {
    env_logger::init().unwrap_or(());
    let pact : serde_json::Value = serde_json::from_str(r#"
      {
        "match": true,
        "comment": "Extra headers allowed",
        "expected" : {
          "headers": {}
        },
        "actual": {
          "headers": {
            "Accept": "alligators"
          }
        }
      }
    "#).unwrap();

    let expected = Response::from_json(&pact.get("expected").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.get("actual").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    let result = match_response(expected, actual);
    if pact_match.as_bool().unwrap() {
       expect!(result).to(be_empty());
    } else {
       expect!(result).to_not(be_empty());
    }
}

#[test]
fn order_of_comma_separated_header_values_different() {
    env_logger::init().unwrap_or(());
    let pact : serde_json::Value = serde_json::from_str(r#"
      {
        "match": false,
        "comment": "Comma separated headers out of order, order can matter http://tools.ietf.org/html/rfc2616",
        "expected" : {
          "headers": {
            "Accept": "alligators, hippos"
          }
        },
        "actual": {
          "headers": {
            "Accept": "hippos, alligators"
          }
        }
      }
    "#).unwrap();

    let expected = Response::from_json(&pact.get("expected").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.get("actual").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    let result = match_response(expected, actual);
    if pact_match.as_bool().unwrap() {
       expect!(result).to(be_empty());
    } else {
       expect!(result).to_not(be_empty());
    }
}

#[test]
fn matches() {
    env_logger::init().unwrap_or(());
    let pact : serde_json::Value = serde_json::from_str(r#"
      {
        "match": true,
        "comment": "Headers match",
        "expected" : {
          "headers": {
            "Accept": "alligators",
            "Content-Type": "hippos"
          }
        },
        "actual": {
          "headers": {
            "Content-Type": "hippos",
            "Accept": "alligators"
          }
        }
      }
    "#).unwrap();

    let expected = Response::from_json(&pact.get("expected").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.get("actual").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    let result = match_response(expected, actual);
    if pact_match.as_bool().unwrap() {
       expect!(result).to(be_empty());
    } else {
       expect!(result).to_not(be_empty());
    }
}

#[test]
fn header_value_is_different_case() {
    env_logger::init().unwrap_or(());
    let pact : serde_json::Value = serde_json::from_str(r#"
      {
        "match": false,
        "comment": "Headers values are case sensitive",
        "expected" : {
          "headers": {
            "Accept": "alligators"
          }
        },
        "actual": {
          "headers": {
            "Accept": "Alligators"
          }
        }
      }
    "#).unwrap();

    let expected = Response::from_json(&pact.get("expected").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.get("actual").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    let result = match_response(expected, actual);
    if pact_match.as_bool().unwrap() {
       expect!(result).to(be_empty());
    } else {
       expect!(result).to_not(be_empty());
    }
}

#[test]
fn header_name_is_different_case() {
    env_logger::init().unwrap_or(());
    let pact : serde_json::Value = serde_json::from_str(r#"
      {
        "match": true,
        "comment": "Header name is case insensitive",
        "expected" : {
          "headers": {
            "Accept": "alligators"
          }
        },
        "actual": {
          "headers": {
            "ACCEPT": "alligators"
          }
        }
      }
    "#).unwrap();

    let expected = Response::from_json(&pact.get("expected").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.get("actual").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    let result = match_response(expected, actual);
    if pact_match.as_bool().unwrap() {
       expect!(result).to(be_empty());
    } else {
       expect!(result).to_not(be_empty());
    }
}

#[test]
fn empty_headers() {
    env_logger::init().unwrap_or(());
    let pact : serde_json::Value = serde_json::from_str(r#"
      {
        "match": true,
        "comment": "Empty headers match",
        "expected" : {
          "headers": {}
      
        },
        "actual": {
          "headers": {}
        }
      }
    "#).unwrap();

    let expected = Response::from_json(&pact.get("expected").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.get("actual").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    let result = match_response(expected, actual);
    if pact_match.as_bool().unwrap() {
       expect!(result).to(be_empty());
    } else {
       expect!(result).to_not(be_empty());
    }
}
