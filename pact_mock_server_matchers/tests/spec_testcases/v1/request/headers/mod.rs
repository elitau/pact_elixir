use pact_mock_server_matchers::model::Request;
use pact_mock_server_matchers::match_request;
use rustc_serialize::json;

#[test]
fn empty_headers() {
    let pact = json!(
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
    );

    let expected = Request::from_json(&pact.find("expected").unwrap());
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.find("actual").unwrap());
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       //assert!(match_request(expected, actual).is_empty(), "Empty headers match");
    } else {
       //assert!(!match_request(expected, actual).is_empty(), "Empty headers match");
    }
}

#[test]
fn header_name_is_different_case() {
    let pact = json!(
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
    );

    let expected = Request::from_json(&pact.find("expected").unwrap());
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.find("actual").unwrap());
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       //assert!(match_request(expected, actual).is_empty(), "Header name is case insensitive");
    } else {
       //assert!(!match_request(expected, actual).is_empty(), "Header name is case insensitive");
    }
}

#[test]
fn header_value_is_different_case() {
    let pact = json!(
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
    );

    let expected = Request::from_json(&pact.find("expected").unwrap());
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.find("actual").unwrap());
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       //assert!(match_request(expected, actual).is_empty(), "Headers values are case sensitive");
    } else {
       //assert!(!match_request(expected, actual).is_empty(), "Headers values are case sensitive");
    }
}

#[test]
fn matches() {
    let pact = json!(
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
    );

    let expected = Request::from_json(&pact.find("expected").unwrap());
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.find("actual").unwrap());
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       //assert!(match_request(expected, actual).is_empty(), "Headers match");
    } else {
       //assert!(!match_request(expected, actual).is_empty(), "Headers match");
    }
}

#[test]
fn order_of_comma_separated_header_values_different() {
    let pact = json!(
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
    );

    let expected = Request::from_json(&pact.find("expected").unwrap());
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.find("actual").unwrap());
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       //assert!(match_request(expected, actual).is_empty(), "Comma separated headers out of order, order can matter http://tools.ietf.org/html/rfc2616");
    } else {
       //assert!(!match_request(expected, actual).is_empty(), "Comma separated headers out of order, order can matter http://tools.ietf.org/html/rfc2616");
    }
}

#[test]
fn unexpected_header_found() {
    let pact = json!(
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
    );

    let expected = Request::from_json(&pact.find("expected").unwrap());
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.find("actual").unwrap());
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       //assert!(match_request(expected, actual).is_empty(), "Extra headers allowed");
    } else {
       //assert!(!match_request(expected, actual).is_empty(), "Extra headers allowed");
    }
}

#[test]
fn whitespace_after_comma_different() {
    let pact = json!(
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
    );

    let expected = Request::from_json(&pact.find("expected").unwrap());
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.find("actual").unwrap());
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       //assert!(match_request(expected, actual).is_empty(), "Whitespace between comma separated headers does not matter");
    } else {
       //assert!(!match_request(expected, actual).is_empty(), "Whitespace between comma separated headers does not matter");
    }
}
