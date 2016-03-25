use libpact::model::Response;
use libpact_matchers::match_response;
use rustc_serialize::json::Json;

#[test]
fn different_status() {
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

    let expected = Response::from_json(&pact.find("expected").unwrap());
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap());
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       assert!(match_response(expected, actual).is_empty(), "Status is incorrect");
    } else {
       assert!(!match_response(expected, actual).is_empty(), "Status is incorrect");
    }
}

#[test]
fn matches() {
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

    let expected = Response::from_json(&pact.find("expected").unwrap());
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap());
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       assert!(match_response(expected, actual).is_empty(), "Status matches");
    } else {
       assert!(!match_response(expected, actual).is_empty(), "Status matches");
    }
}
