#[allow(unused_imports)]
use pact_matching::models::*;
#[allow(unused_imports)]
use env_logger;
#[allow(unused_imports)]
use pact_matching::match_request;
#[allow(unused_imports)]
use expectest::prelude::*;

#[test]
fn unexpected_param() {
    env_logger::init().unwrap_or(());
    let pact = json!(r#"
      {
        "match": false,
        "comment": "Queries are not the same - elephant is not expected",
        "expected" : {
          "method": "GET",
          "path": "/path",
          "query": "alligator=Mary&hippo=John",
          "headers": {}
      
        },
        "actual": {
          "method": "GET",
          "path": "/path",
          "query": "alligator=Mary&hippo=John&elephant=unexpected",
          "headers": {}
      
        }
      }
    "#);

    let expected = Request::from_json(&pact.get("expected").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.get("actual").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    if pact_match.as_bool().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn trailing_ampersand() {
    env_logger::init().unwrap_or(());
    let pact = json!(r#"
      {
        "match": true,
        "comment": "Trailing amperands can be ignored",
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
    "#);

    let expected = Request::from_json(&pact.get("expected").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.get("actual").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    if pact_match.as_bool().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn same_parameter_multiple_times() {
    env_logger::init().unwrap_or(());
    let pact = json!(r#"
      {
        "match": true,
        "comment": "Queries are the same - multiple values are in same order",
        "expected" : {
          "method": "GET",
          "path": "/path",
          "query": "animal=alligator&animal=hippo&animal=elephant&hippo=Fred",
          "headers": {}
      
        },
        "actual": {
          "method": "GET",
          "path": "/path",
          "query": "animal=alligator&hippo=Fred&animal=hippo&animal=elephant",
          "headers": {}
        }
      }
    "#);

    let expected = Request::from_json(&pact.get("expected").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.get("actual").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    if pact_match.as_bool().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn same_parameter_multiple_times_in_different_order() {
    env_logger::init().unwrap_or(());
    let pact = json!(r#"
      {
        "match": false,
        "comment": "Queries are not the same - values are in different order",
        "expected" : {
          "method": "GET",
          "path": "/path",
          "query": "animal=alligator&animal=hippo&animal=elephant",
          "headers": {}
      
        },
        "actual": {
          "method": "GET",
          "path": "/path",
          "query": "animal=hippo&animal=alligator&animal=elephant",
          "headers": {}
        }
      }
    "#);

    let expected = Request::from_json(&pact.get("expected").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.get("actual").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    if pact_match.as_bool().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn same_parameter_different_values() {
    env_logger::init().unwrap_or(());
    let pact = json!(r#"
      {
        "match": false,
        "comment": "Queries are not the same - animals are alligator, hippo versus alligator, elephant",
        "expected" : {
          "method": "GET",
          "path": "/path",
          "query": "animal=alligator&animal=hippo",
          "headers": {}
      
        },
        "actual": {
          "method": "GET",
          "path": "/path",
          "query": "animal=alligator&animal=elephant",
          "headers": {}
      
        }
      }
    "#);

    let expected = Request::from_json(&pact.get("expected").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.get("actual").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    if pact_match.as_bool().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn missing_params() {
    env_logger::init().unwrap_or(());
    let pact = json!(r#"
      {
        "match": false,
        "comment": "Queries are not the same - elephant is missing",
        "expected" : {
          "method": "GET",
          "path": "/path",
          "query": "alligator=Mary&hippo=Fred&elephant=missing",
          "headers": {}
      
        },
        "actual": {
          "method": "GET",
          "path": "/path",
          "query": "alligator=Mary&hippo=Fred",
          "headers": {}
      
        }
      }
    "#);

    let expected = Request::from_json(&pact.get("expected").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.get("actual").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    if pact_match.as_bool().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn matches() {
    env_logger::init().unwrap_or(());
    let pact = json!(r#"
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
    "#);

    let expected = Request::from_json(&pact.get("expected").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.get("actual").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    if pact_match.as_bool().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn matches_with_equals_in_the_query_value() {
    env_logger::init().unwrap_or(());
    let pact = json!(r#"
      {
        "match": true,
        "comment": "Queries are equivalent",
        "expected" : {
          "method": "GET",
          "path": "/path",
          "query": "options=delete.topic.enable=true&broker=1",
          "headers": {}
      
        },
        "actual": {
          "method": "GET",
          "path": "/path",
          "query": "options=delete.topic.enable%3Dtrue&broker=1",
          "headers": {}
      
        }
      }
    "#);

    let expected = Request::from_json(&pact.get("expected").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.get("actual").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    if pact_match.as_bool().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn different_params() {
    env_logger::init().unwrap_or(());
    let pact = json!(r#"
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
    "#);

    let expected = Request::from_json(&pact.get("expected").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.get("actual").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    if pact_match.as_bool().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn different_order() {
    env_logger::init().unwrap_or(());
    let pact = json!(r#"
      {
        "match": true,
        "comment": "Queries are the same but in different key order",
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
    "#);

    let expected = Request::from_json(&pact.get("expected").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.get("actual").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    if pact_match.as_bool().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}
