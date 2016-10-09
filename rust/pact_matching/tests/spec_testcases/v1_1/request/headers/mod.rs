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
fn empty_headers() {
    env_logger::init().unwrap_or(());
    let pact = Json::from_str(r#"
      {
        "match": true,
        "comment": "Empty headers match",
        "expected" : {
          "method": "POST",
          "path": "/path",
          "query": "",
          "headers": {}
      
        },
        "actual": {
          "method": "POST",
          "path": "/path",
          "query": "",
          "headers": {}
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.find("expected").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.find("actual").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn header_name_is_different_case() {
    env_logger::init().unwrap_or(());
    let pact = Json::from_str(r#"
      {
        "match": true,
        "comment": "Header name is case insensitive",
        "expected" : {
          "method": "POST",
          "path": "/path",
          "query": "",
          "headers": {
            "Accept": "alligators"
          }
        },
        "actual": {
          "method": "POST",
          "path": "/path",
          "query": "",
          "headers": {
            "ACCEPT": "alligators"
          }
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.find("expected").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.find("actual").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn header_value_is_different_case() {
    env_logger::init().unwrap_or(());
    let pact = Json::from_str(r#"
      {
        "match": false,
        "comment": "Headers values are case sensitive",
        "expected" : {
          "method": "POST",
          "path": "/path",
          "query": "",
          "headers": {
            "Accept": "alligators"
          }
        },
        "actual": {
          "method": "POST",
          "path": "/path",
          "query": "",
          "headers": {
            "Accept": "Alligators"
          }
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.find("expected").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.find("actual").unwrap(), &PactSpecification::V1_1);
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
        "comment": "Headers match",
        "expected" : {
          "method": "POST",
          "path": "/path",
          "query": "",
          "headers": {
            "Accept": "alligators",
            "Content-Type": "hippos"
          }
        },
        "actual": {
          "method": "POST",
          "path": "/path",
          "query": "",
          "headers": {
            "Content-Type": "hippos",
            "Accept": "alligators"
          }
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.find("expected").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.find("actual").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn order_of_comma_separated_header_values_different() {
    env_logger::init().unwrap_or(());
    let pact = Json::from_str(r#"
      {
        "match": false,
        "comment": "Comma separated headers out of order, order can matter http://tools.ietf.org/html/rfc2616",
        "expected" : {
          "method": "POST",
          "path": "/path",
          "query": "",
          "headers": {
            "Accept": "alligators, hippos"
          }
        },
        "actual": {
          "method": "POST",
          "path": "/path",
          "query": "",
          "headers": {
            "Accept": "hippos, alligators"
          }
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.find("expected").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.find("actual").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn unexpected_header_found() {
    env_logger::init().unwrap_or(());
    let pact = Json::from_str(r#"
      {
        "match": true,
        "comment": "Extra headers allowed",
        "expected" : {
          "method": "POST",
          "path": "/path",
          "query": "",
          "headers": {}
        },
        "actual": {
          "method": "POST",
          "path": "/path",
          "query": "",
          "headers": {
            "Accept": "alligators"
          }
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.find("expected").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.find("actual").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn whitespace_after_comma_different() {
    env_logger::init().unwrap_or(());
    let pact = Json::from_str(r#"
      {
        "match": true,
        "comment": "Whitespace between comma separated headers does not matter",
        "expected" : {
          "method": "POST",
          "path": "/path",
          "query": "",
          "headers": {
            "Accept": "alligators,hippos"
          }
        },
        "actual": {
          "method": "POST",
          "path": "/path",
          "query": "",
          "headers": {
            "Accept": "alligators, hippos"
          }
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.find("expected").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.find("actual").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}
