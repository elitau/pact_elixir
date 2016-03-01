use pact_mock_server_matchers::model::Request;
use pact_mock_server_matchers::match_request;
use rustc_serialize::json;

#[test]
fn different_param_order() {
    let pact = json!(
      {
        "match": false,
        "comment": "Query strings are matched using basic string equality, these are not equal.",
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
    );

    let expected = Request::from_json(&pact.find("expected").unwrap());
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.find("actual").unwrap());
    println!("{:?}", expected);
    let comment = "comment"; //pact.find("comment").unwrap().as_string().unwrap();
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       assert!(match_request(&expected, &actual).is_empty(), comment);
    } else {
       assert!(!match_request(&expected, &actual).is_empty(), comment);
    }
}

#[test]
fn different_param_values() {
    let pact = json!(
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
    );

    let expected = Request::from_json(&pact.find("expected").unwrap());
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.find("actual").unwrap());
    println!("{:?}", expected);
    let comment = "comment"; //pact.find("comment").unwrap().as_string().unwrap();
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       assert!(match_request(&expected, &actual).is_empty(), comment);
    } else {
       assert!(!match_request(&expected, &actual).is_empty(), comment);
    }
}

#[test]
fn matches() {
    let pact = json!(
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
    );

    let expected = Request::from_json(&pact.find("expected").unwrap());
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.find("actual").unwrap());
    println!("{:?}", expected);
    let comment = "comment"; //pact.find("comment").unwrap().as_string().unwrap();
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       assert!(match_request(&expected, &actual).is_empty(), comment);
    } else {
       assert!(!match_request(&expected, &actual).is_empty(), comment);
    }
}

#[test]
fn trailing_amperand() {
    let pact = json!(
      {
        "match": false,
        "comment": "Query strings are matched using basic string equality, these are not equal.",
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
    );

    let expected = Request::from_json(&pact.find("expected").unwrap());
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.find("actual").unwrap());
    println!("{:?}", expected);
    let comment = "comment"; //pact.find("comment").unwrap().as_string().unwrap();
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       assert!(match_request(&expected, &actual).is_empty(), comment);
    } else {
       assert!(!match_request(&expected, &actual).is_empty(), comment);
    }
}
