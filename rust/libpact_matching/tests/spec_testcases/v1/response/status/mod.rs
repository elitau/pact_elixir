#[allow(unused_imports)]
use pact_matching::models::*;
#[allow(unused_imports)]
use env_logger;
#[allow(unused_imports)]
use pact_matching::match_response;
#[allow(unused_imports)]
use rustc_serialize::json::Json;
#[allow(unused_imports)]
use expectest::prelude::*;

#[test]
fn different_status() {
    env_logger::init().unwrap_or(());
    let pact = Json::from_str(r#"
      {
      	"match": false,
      	"comment": "Status is incorrect",
      	"expected" : {
      		"status" : 202
      	},
      	"actual" : {
      		"status" : 400
      	}
      }
    "#).unwrap();

    let expected = Response::from_json(&pact.find("expected").unwrap(), &PactSpecification::V1);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap(), &PactSpecification::V1);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    let result = match_response(expected, actual);
    if pact_match.as_boolean().unwrap() {
       expect!(result).to(be_empty());
    } else {
       expect!(result).to_not(be_empty());
    }
}

#[test]
fn matches() {
    env_logger::init().unwrap_or(());
    let pact = Json::from_str(r#"
      {
      	"match": true,
      	"comment": "Status matches",
      	"expected" : {
      		"status" : 202
      	},
      	"actual" : {
      		"status" : 202
      	}
      }
    "#).unwrap();

    let expected = Response::from_json(&pact.find("expected").unwrap(), &PactSpecification::V1);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap(), &PactSpecification::V1);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    let result = match_response(expected, actual);
    if pact_match.as_boolean().unwrap() {
       expect!(result).to(be_empty());
    } else {
       expect!(result).to_not(be_empty());
    }
}
