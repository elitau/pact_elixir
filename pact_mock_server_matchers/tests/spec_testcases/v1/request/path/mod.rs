use pact_mock_server_matchers::model::Request;
use pact_mock_server_matchers::match_request;
use rustc_serialize::json;

#[test]
fn empty_path_found_when_forward_slash_expected() {
    let pact = json!(
      {
        "match": false,
        "comment": "Empty path found when forward slash expected",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {}
      
        },
        "actual": {
          "method": "POST",
          "path": "",
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
       assert!(match_request(&expected, &actual).is_empty(), comment);
    } else {
       assert!(!match_request(&expected, &actual).is_empty(), comment);
    }
}

#[test]
fn forward_slash_found_when_empty_path_expected() {
    let pact = json!(
      {
        "match": false,
        "comment": "Foward slash found when empty path expected",
        "expected" : {
          "method": "POST",
          "path": "",
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
       assert!(match_request(&expected, &actual).is_empty(), comment);
    } else {
       assert!(!match_request(&expected, &actual).is_empty(), comment);
    }
}

#[test]
fn incorrect_path() {
    let pact = json!(
      {
        "match": false,
        "comment": "Paths do not match",
        "expected" : {
          "method": "POST",
          "path": "/path/to/something",
          "query": "",
          "headers": {}
      
        },
        "actual": {
          "method": "POST",
          "path": "/path/to/something/else",
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
        "comment": "Paths match",
        "expected" : {
          "method": "POST",
          "path": "/path/to/something",
          "query": "",
          "headers": {}
      
        },
        "actual": {
          "method": "POST",
          "path": "/path/to/something",
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
       assert!(match_request(&expected, &actual).is_empty(), comment);
    } else {
       assert!(!match_request(&expected, &actual).is_empty(), comment);
    }
}

#[test]
fn missing_trailing_slash_in_path() {
    let pact = json!(
      {
        "match": false,
        "comment": "Path is missing trailing slash, trailing slashes can matter",
        "expected" : {
          "method": "POST",
          "path": "/path/to/something/",
          "query": "",
          "headers": {}
      
        },
        "actual": {
          "method": "POST",
          "path": "/path/to/something",
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
       assert!(match_request(&expected, &actual).is_empty(), comment);
    } else {
       assert!(!match_request(&expected, &actual).is_empty(), comment);
    }
}

#[test]
fn unexpected_trailing_slash_in_path() {
    let pact = json!(
      {
        "match": false,
        "comment": "Path has unexpected trailing slash, trailing slashes can matter",
        "expected" : {
          "method": "POST",
          "path": "/path/to/something",
          "query": "",
          "headers": {}
      
        },
        "actual": {
          "method": "POST",
          "path": "/path/to/something/",
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
       assert!(match_request(&expected, &actual).is_empty(), comment);
    } else {
       assert!(!match_request(&expected, &actual).is_empty(), comment);
    }
}
