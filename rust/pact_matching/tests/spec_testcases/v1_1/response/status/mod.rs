#[allow(unused_imports)]
use pact_matching::models::*;
#[allow(unused_imports)]
use env_logger;
#[allow(unused_imports)]
use pact_matching::match_response;
#[allow(unused_imports)]
use expectest::prelude::*;

#[test]
fn matches() {
    env_logger::init().unwrap_or(());
    let pact = json!(r#"
      {
          "match": true,
          "comment": "Status matches",
          "expected": {
              "status": 202
          },
          "actual": {
              "status": 202
          }
      }
    "#);

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
fn different_status() {
    env_logger::init().unwrap_or(());
    let pact = json!(r#"
      {
          "match": false,
          "comment": "Status is incorrect",
          "expected": {
              "status": 202
          },
          "actual": {
              "status": 400
          }
      }
    "#);

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
