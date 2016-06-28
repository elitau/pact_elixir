#[allow(unused_imports)]
use pact_matching::models::*;
#[allow(unused_imports)]
use pact_matching::match_response;
#[allow(unused_imports)]
use rustc_serialize::json::Json;
#[allow(unused_imports)]
use expectest::prelude::*;

#[test]
fn different_status() {
    let pact = Json::from_str(r#"
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
    "#).unwrap();

    let expected = Response::from_json(&pact.find("expected").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       expect!(match_response(expected, actual)).to(be_empty());
    } else {
       expect!(match_response(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn matches() {
    let pact = Json::from_str(r#"
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
    "#).unwrap();

    let expected = Response::from_json(&pact.find("expected").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       expect!(match_response(expected, actual)).to(be_empty());
    } else {
       expect!(match_response(expected, actual)).to_not(be_empty());
    }
}
