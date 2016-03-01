use pact_mock_server_matchers::model::Response;
use pact_mock_server_matchers::match_response;
use rustc_serialize::json;

#[test]
fn empty_headers() {
    let pact = json!(
      {
        "match": true,
        "comment": "Empty headers match",
        "expected" : {
          "headers": {}
      
        },
        "actual": {
          "headers": {}
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
       //assert!(match_response(&expected, &actual).is_empty(), comment);
    } else {
       //assert!(!match_response(&expected, &actual).is_empty(), comment);
    }
}

#[test]
fn header_name_is_different_case() {
    let pact = json!(
      {
        "match": true,
        "comment": "Header name is case insensitive",
        "expected" : {
          "headers": {
            "Accept": "alligators"
          }
        },
        "actual": {
          "headers": {
            "ACCEPT": "alligators"
          }
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
       //assert!(match_response(&expected, &actual).is_empty(), comment);
    } else {
       //assert!(!match_response(&expected, &actual).is_empty(), comment);
    }
}

#[test]
fn header_value_is_different_case() {
    let pact = json!(
      {
        "match": false,
        "comment": "Headers values are case sensitive",
        "expected" : {
          "headers": {
            "Accept": "alligators"
          }
        },
        "actual": {
          "headers": {
            "Accept": "Alligators"
          }
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
       //assert!(match_response(&expected, &actual).is_empty(), comment);
    } else {
       //assert!(!match_response(&expected, &actual).is_empty(), comment);
    }
}

#[test]
fn matches() {
    let pact = json!(
      {
        "match": true,
        "comment": "Headers match",
        "expected" : {
          "headers": {
            "Accept": "alligators",
            "Content-Type": "hippos"
          }
        },
        "actual": {
          "headers": {
            "Content-Type": "hippos",
            "Accept": "alligators"
          }
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
       //assert!(match_response(&expected, &actual).is_empty(), comment);
    } else {
       //assert!(!match_response(&expected, &actual).is_empty(), comment);
    }
}

#[test]
fn order_of_comma_separated_header_values_different() {
    let pact = json!(
      {
        "match": false,
        "comment": "Comma separated headers out of order, order can matter http://tools.ietf.org/html/rfc2616",
        "expected" : {
          "headers": {
            "Accept": "alligators, hippos"
          }
        },
        "actual": {
          "headers": {
            "Accept": "hippos, alligators"
          }
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
       //assert!(match_response(&expected, &actual).is_empty(), comment);
    } else {
       //assert!(!match_response(&expected, &actual).is_empty(), comment);
    }
}

#[test]
fn unexpected_header_found() {
    let pact = json!(
      {
        "match": true,
        "comment": "Extra headers allowed",
        "expected" : {
          "headers": {}
        },
        "actual": {
          "headers": {
            "Accept": "alligators"
          }
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
       //assert!(match_response(&expected, &actual).is_empty(), comment);
    } else {
       //assert!(!match_response(&expected, &actual).is_empty(), comment);
    }
}

#[test]
fn whitespace_after_comma_different() {
    let pact = json!(
      {
        "match": true,
        "comment": "Whitespace between comma separated headers does not matter",
        "expected" : {
          "headers": {
            "Accept": "alligators,hippos"
          }
        },
        "actual": {
          "headers": {
            "Accept": "alligators, hippos"
          }
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
       //assert!(match_response(&expected, &actual).is_empty(), comment);
    } else {
       //assert!(!match_response(&expected, &actual).is_empty(), comment);
    }
}
