use pact_mock_server_matchers::model::Response;
use pact_mock_server_matchers::match_response;
use rustc_serialize::json;

#[test]
fn different_status() {
    let pact = json!(
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
    );

    let expected = Response::from_json(&pact.find("expected").unwrap());
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap());
    println!("{:?}", actual);
    let comment = "comment"; // pact.find("comment").unwrap().as_string().unwrap();
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       assert!(match_response(&expected, &actual).is_empty(), comment);
    } else {
       assert!(!match_response(&expected, &actual).is_empty(), comment);
    }
}

#[test]
fn matches() {
    let pact = json!(
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
    );

    let expected = Response::from_json(&pact.find("expected").unwrap());
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap());
    println!("{:?}", actual);
    let comment = "comment"; // pact.find("comment").unwrap().as_string().unwrap();
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       assert!(match_response(&expected, &actual).is_empty(), comment);
    } else {
       assert!(!match_response(&expected, &actual).is_empty(), comment);
    }
}
