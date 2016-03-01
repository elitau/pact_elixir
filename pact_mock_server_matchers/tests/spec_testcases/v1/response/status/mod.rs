use pact_mock_server_matchers::model::Response;
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
    let pact_match = pact.find("match").unwrap();
    assert!(pact_match.as_boolean().unwrap());
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
    let pact_match = pact.find("match").unwrap();
    assert!(pact_match.as_boolean().unwrap());
}
      
