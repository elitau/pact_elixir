use pact_mock_server_matchers::model::Request;
use pact_mock_server_matchers::match_request;
use rustc_serialize::json;

#[test]
fn different_method() {
    let pact = json!(
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
    );

    let expected = Request::from_json(&pact.find("expected").unwrap());
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.find("actual").unwrap());
    println!("{:?}", expected);
    let comment = "comment"; //pact.find("comment").unwrap().as_string().unwrap();
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       assert!(match_request(expected, actual).is_empty(), comment);
    } else {
       assert!(!match_request(expected, actual).is_empty(), comment);
    }
}

#[test]
fn matches() {
    let pact = json!(
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
    );

    let expected = Request::from_json(&pact.find("expected").unwrap());
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.find("actual").unwrap());
    println!("{:?}", expected);
    let comment = "comment"; //pact.find("comment").unwrap().as_string().unwrap();
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       assert!(match_request(expected, actual).is_empty(), comment);
    } else {
       assert!(!match_request(expected, actual).is_empty(), comment);
    }
}

#[test]
fn method_is_different_case() {
    let pact = json!(
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
    );

    let expected = Request::from_json(&pact.find("expected").unwrap());
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.find("actual").unwrap());
    println!("{:?}", expected);
    let comment = "comment"; //pact.find("comment").unwrap().as_string().unwrap();
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       assert!(match_request(expected, actual).is_empty(), comment);
    } else {
       assert!(!match_request(expected, actual).is_empty(), comment);
    }
}
