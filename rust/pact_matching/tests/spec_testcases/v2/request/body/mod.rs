#[allow(unused_imports)]
use pact_matching::models::*;
#[allow(unused_imports)]
use env_logger;
#[allow(unused_imports)]
use pact_matching::match_request;
#[allow(unused_imports)]
use expectest::prelude::*;
use serde_json;

#[test]
fn array_at_top_level_xml() {
    env_logger::init().unwrap_or(());
    let pact : serde_json::Value = serde_json::from_str(r#"
      {
        "match": true,
        "comment": "XML top level array matches",
        "expected": {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/xml"},
          "body": "<?xml version=\"1.0\" encoding=\"UTF-8\"?><people><rogger dob=\"06/10/2015\" name=\"Rogger the Dogger\" id=\"1014753708\" timestamp=\"2015-06-10T20:41:37\"/><cat dob=\"06/10/2015\" name=\"Cat in the Hat\" id=\"8858030303\" timestamp=\"2015-06-10T20:41:37\"/></people>"
        },
        "actual": {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/xml"},
          "body": "<?xml version=\"1.0\" encoding=\"UTF-8\"?><people><rogger dob=\"06/10/2015\" name=\"Rogger the Dogger\" id=\"1014753708\" timestamp=\"2015-06-10T20:41:37\"/><cat dob=\"06/10/2015\" name=\"Cat in the Hat\" id=\"8858030303\" timestamp=\"2015-06-10T20:41:37\"/></people>"
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.get("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.get("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    if pact_match.as_bool().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn array_at_top_level() {
    env_logger::init().unwrap_or(());
    let pact : serde_json::Value = serde_json::from_str(r#"
      {
        "match": true,
        "comment": "top level array matches",
        "expected": {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/json"},
          "body": [
            {
              "dob": "06/10/2015",
              "name": "Rogger the Dogger",
              "id": 1014753708,
              "timestamp": "2015-06-10T20:41:37"
            },
            {
              "dob": "06/10/2015",
              "name": "Cat in the Hat",
              "id": 8858030303,
              "timestamp": "2015-06-10T20:41:37"
            }
          ]
        },
        "actual": {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/json"},
          "body": [
            {
              "dob": "06/10/2015",
              "name": "Rogger the Dogger",
              "id": 1014753708,
              "timestamp": "2015-06-10T20:41:37"
            },
            {
              "dob": "06/10/2015",
              "name": "Cat in the Hat",
              "id": 8858030303,
              "timestamp": "2015-06-10T20:41:37"
            }
          ]
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.get("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.get("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    if pact_match.as_bool().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn array_in_different_order_xml() {
    env_logger::init().unwrap_or(());
    let pact : serde_json::Value = serde_json::from_str(r#"
      {
        "match": false,
        "comment": "XML Favourite colours in wrong order",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/xml"},
          "body": "<?xml version=\"1.0\" encoding=\"UTF-8\"?><alligator><favouriteColours><favouriteColour>red</favouriteColour><favouriteColour>blue</favouriteColour></favouriteColours></alligator>"
        },
        "actual": {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/xml"},
          "body": "<?xml version=\"1.0\" encoding=\"UTF-8\"?><alligator><favouriteColours><favouriteColour>blue</favouriteColour><favouriteColour>red</favouriteColour></favouriteColours></alligator>"
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.get("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.get("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    if pact_match.as_bool().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn array_in_different_order() {
    env_logger::init().unwrap_or(());
    let pact : serde_json::Value = serde_json::from_str(r#"
      {
        "match": false,
        "comment": "Favourite colours in wrong order",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator":{
              "favouriteColours": ["red","blue"]
            }
          }
        },
        "actual": {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator":{
              "favouriteColours": ["blue", "red"]
            }
          }
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.get("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.get("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    if pact_match.as_bool().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn array_size_less_than_required_xml() {
    env_logger::init().unwrap_or(());
    let pact : serde_json::Value = serde_json::from_str(r#"
      {
        "match": false,
        "comment": "XML Array must have at least 2 elements",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/xml"},
          "matchingRules": {
            "$.body.animals": {"min": 2}
          },
          "body": "<?xml version=\"1.0\" encoding=\"UTF-8\"?><animals><alligator name=\"Mary\"/></animals>"
        },
        "actual": {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/xml"},
          "body": "<?xml version=\"1.0\" encoding=\"UTF-8\"?><animals><alligator name=\"Mary\"/></animals>"
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.get("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.get("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    if pact_match.as_bool().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn array_size_less_than_required() {
    env_logger::init().unwrap_or(());
    let pact : serde_json::Value = serde_json::from_str(r#"
      {
        "match": false,
        "comment": "Array must have at least 2 elements",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/json"},
          "matchingRules": {
            "$.body.animals": {"min": 2}
          },
          "body": {
            "animals": [
              {
                "name" : "Fred"
              }
            ]
          }
        },
        "actual": {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/json"},
          "body": {
            "animals": [
              {
                "name" : "Fred"
              }
            ]
          }
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.get("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.get("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    if pact_match.as_bool().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn array_with_at_least_one_element_matching_by_example_xml() {
    env_logger::init().unwrap_or(());
    let pact : serde_json::Value = serde_json::from_str(r#"
      {
        "match": true,
        "comment": "XML Tag with at least one element match",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/xml"},
          "matchingRules": {
            "$.body.animals": {"min": 1, "match": "type"},
            "$.body.animals[0]": {"match": "type"},
            "$.body.animals[1]": {"match": "type"}
          },
          "body": "<?xml version=\"1.0\" encoding=\"UTF-8\"?><animals><alligator name=\"Fred\"/></animals>"
        },
        "actual": {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/xml"},
          "body": "<?xml version=\"1.0\" encoding=\"UTF-8\"?><animals><alligator name=\"Mary\"/><alligator name=\"Susan\"/></animals>"
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.get("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.get("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    if pact_match.as_bool().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn array_with_at_least_one_element_matching_by_example() {
    env_logger::init().unwrap_or(());
    let pact : serde_json::Value = serde_json::from_str(r#"
      {
        "match": true,
        "comment": "Types and regular expressions match",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/json"},
          "matchingRules": {
            "$.body.animals": {"min": 1, "match": "type"},
            "$.body.animals[*].*": {"match": "type"}
          },
          "body": {
            "animals": [
              {
                "name" : "Fred"
              }
            ]
          }
        },
        "actual": {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/json"},
          "body": {
            "animals": [
              {
                "name" : "Mary"
              },{
                "name" : "Susan"
              }
            ]
          }
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.get("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.get("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    if pact_match.as_bool().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn array_with_at_least_one_element_not_matching_example_type() {
    env_logger::init().unwrap_or(());
    let pact : serde_json::Value = serde_json::from_str(r#"
      {
        "match": false,
        "comment": "Wrong type for name key",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/json"},
          "matchingRules": {
            "$.body.animals": {"min": 1, "match": "type"},
            "$.body.animals[*].*": {"match": "type"}
          },
          "body": {
            "animals": [
              {
                "name" : "Fred"
              }
            ]
          }
        },
        "actual": {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/json"},
          "body": {
            "animals": [
              {
                "name" : "Mary"
              },{
                "name" : 1
              }
            ]
          }
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.get("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.get("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    if pact_match.as_bool().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn array_with_nested_array_that_does_not_match() {
    env_logger::init().unwrap_or(());
    let pact : serde_json::Value = serde_json::from_str(r#"
      {
        "match": false,
        "comment": "Nested arrays do not match, age is wrong type",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/json"},
          "matchingRules": {
            "$.body.animals": {"min": 1, "match": "type"},
            "$.body.animals[*].*": {"match": "type"},
            "$.body.animals[*].children": {"min": 1},
            "$.body.animals[*].children[*].*": {"match": "type"}
          },
          "body": {
            "animals": [
              {
                "name" : "Fred",
                "children": [
                  {
                    "age": 9
                  }
                ]
              }
            ]
          }
        },
        "actual": {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/json"},
          "body": {
            "animals": [
              {
                "name" : "Mary",
                "children": [{"age": "9"}]
              }
            ]
          }
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.get("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.get("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    if pact_match.as_bool().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn array_with_nested_array_that_matches() {
    env_logger::init().unwrap_or(());
    let pact : serde_json::Value = serde_json::from_str(r#"
      {
        "match": true,
        "comment": "Nested arrays match",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/json"},
          "matchingRules": {
            "$.body.animals": {"min": 1, "match": "type"},
            "$.body.animals[*].*": {"match": "type"},
            "$.body.animals[*].children": {"min": 1, "match": "type"},
            "$.body.animals[*].children[*].*": {"match": "type"}
          },
          "body": {
            "animals": [
              {
                "name" : "Fred",
                "children": [
                  {
                    "age": 9
                  }
                ]
              }
            ]
          }
        },
        "actual": {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/json"},
          "body": {
            "animals": [
              {
                "name" : "Mary",
                "children": [
                  {"age": 3},
                  {"age": 5},
                  {"age": 5456}
                ]
              },{
                "name" : "Jo",
                "children": [
                  {"age": 0}
                ]
              }
            ]
          }
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.get("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.get("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    if pact_match.as_bool().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn array_with_regular_expression_in_element_xml() {
    env_logger::init().unwrap_or(());
    let pact : serde_json::Value = serde_json::from_str(r#"
      {
        "match": true,
        "comment": "XML Types and regular expressions match",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/xml"},
          "matchingRules": {
            "$.body.animals": {"min": 1, "match": "type"},
            "$.body.animals[0]": {"match": "type"},
            "$.body.animals[1]": {"match": "type"},
            "$.body.animals[*]['@phoneNumber']": {"match": "regex", "regex": "\\d+"}
          },
          "body": "<?xml version=\"1.0\" encoding=\"UTF-8\"?><animals><alligator phoneNumber=\"0415674567\"/></animals>"
        },
        "actual": {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/xml"},
          "body": "<?xml version=\"1.0\" encoding=\"UTF-8\"?><animals><alligator phoneNumber=\"333\"/><alligator phoneNumber=\"983479823479283478923\"/></animals>"
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.get("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.get("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    if pact_match.as_bool().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn array_with_regular_expression_in_element() {
    env_logger::init().unwrap_or(());
    let pact : serde_json::Value = serde_json::from_str(r#"
      {
        "match": true,
        "comment": "Types and regular expressions match",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/json"},
          "matchingRules": {
            "$.body.animals": {"min": 1, "match": "type"},
            "$.body.animals[*].*": {"match": "type"},
            "$.body.animals[*].phoneNumber": {"match": "regex", "regex": "\\d+"}
          },
          "body": {
            "animals": [
              {
                "phoneNumber": "0415674567"
              }
            ]
          }
        },
        "actual": {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/json"},
          "body": {
            "animals": [
              {
                "phoneNumber": "333"
              },{
                "phoneNumber": "983479823479283478923"
              }
            ]
          }
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.get("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.get("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    if pact_match.as_bool().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn array_with_regular_expression_that_does_not_match_in_element_xml() {
    env_logger::init().unwrap_or(());
    let pact : serde_json::Value = serde_json::from_str(r#"
      {
        "match": false,
        "comment": "Types and regular expressions match",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/xml"},
          "matchingRules": {
            "$.body.animals": {"min": 1, "match": "type"},
            "$.body.animals.0": {"match": "type"},
            "$.body.animals.1": {"match": "type"},
            "$.body.animals[*].alligator['@phoneNumber']": {"match": "regex", "regex": "\\d+"}
          },
          "body": "<?xml version=\"1.0\" encoding=\"UTF-8\"?><animals><alligator phoneNumber=\"0415674567\"/></animals>"
        },
        "actual": {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/xml"},
          "body": "<?xml version=\"1.0\" encoding=\"UTF-8\"?><animals><alligator phoneNumber=\"123\"/><alligator phoneNumber=\"abc\"/></animals>"
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.get("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.get("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    if pact_match.as_bool().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn array_with_regular_expression_that_does_not_match_in_element() {
    env_logger::init().unwrap_or(());
    let pact : serde_json::Value = serde_json::from_str(r#"
      {
        "match": false,
        "comment": "Types and regular expressions match",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/json"},
          "matchingRules": {
            "$.body.animals": {"min": 1, "match": "type"},
            "$.body.animals[*].*": {"match": "type"},
            "$.body.animals[*].phoneNumber": {"match": "regex", "regex": "\\d+"}
          },
          "body": {
            "animals": [
              {
                "phoneNumber": "0415674567"
              }
            ]
          }
        },
        "actual": {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/json"},
          "body": {
            "animals": [
              {
                "phoneNumber": "333"
              },{
                "phoneNumber": "abc"
              }
            ]
          }
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.get("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.get("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    if pact_match.as_bool().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn different_value_found_at_index_xml() {
    env_logger::init().unwrap_or(());
    let pact : serde_json::Value = serde_json::from_str(r#"
      {
        "match": false,
        "comment": "XML Incorrect favourite colour",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/xml"},
          "body": "<?xml version=\"1.0\" encoding=\"UTF-8\"?><alligator><favouriteColours><favouriteColour>red</favouriteColour><favouriteColour>blue</favouriteColour></favouriteColours></alligator>"
        },
        "actual": {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/xml"},
          "body": "<?xml version=\"1.0\" encoding=\"UTF-8\"?><alligator><favouriteColours><favouriteColour>red</favouriteColour><favouriteColour>taupe</favouriteColour></favouriteColours></alligator>"
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.get("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.get("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    if pact_match.as_bool().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn different_value_found_at_index() {
    env_logger::init().unwrap_or(());
    let pact : serde_json::Value = serde_json::from_str(r#"
      {
        "match": false,
        "comment": "Incorrect favourite colour",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator":{
              "favouriteColours": ["red","blue"]
            }
          }
        },
        "actual": {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator":{
              "favouriteColours": ["red","taupe"]
            }
          }
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.get("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.get("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    if pact_match.as_bool().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn different_value_found_at_key_xml() {
    env_logger::init().unwrap_or(());
    let pact : serde_json::Value = serde_json::from_str(r#"
      {
        "match": false,
        "comment": "XML Incorrect value at alligator name",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/xml"},
          "body": "<?xml version=\"1.0\" encoding=\"UTF-8\"?><alligator name=\"Mary\"/>"
        },
        "actual": {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/xml"},
          "body": "<?xml version=\"1.0\" encoding=\"UTF-8\"?><alligator name=\"Fred\"/>"
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.get("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.get("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    if pact_match.as_bool().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn different_value_found_at_key() {
    env_logger::init().unwrap_or(());
    let pact : serde_json::Value = serde_json::from_str(r#"
      {
        "match": false,
        "comment": "Incorrect value at alligator name",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator":{
              "name": "Mary"
            }
          }
        },
        "actual": {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator":{
              "name": "Fred"
            }
          }
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.get("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.get("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    if pact_match.as_bool().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn empty_body_no_content_type() {
    env_logger::init().unwrap_or(());
    let pact : serde_json::Value = serde_json::from_str(r#"
      {
        "match": true,
        "comment": "Empty body, no content-type",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "body": ""
        },
        "actual": {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/json"},
          "body": ""
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.get("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.get("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    if pact_match.as_bool().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn empty_body() {
    env_logger::init().unwrap_or(());
    let pact : serde_json::Value = serde_json::from_str(r#"
      {
        "match": true,
        "comment": "Empty body",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/json"},
          "body": ""
        },
        "actual": {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/json"},
          "body": ""
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.get("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.get("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    if pact_match.as_bool().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn empty_found_at_key_where_not_empty_expected_xml() {
    env_logger::init().unwrap_or(());
    let pact : serde_json::Value = serde_json::from_str(r#"
      {
        "match": false,
        "comment": "XML Name should not be empty",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/xml"},
          "body": "<?xml version=\"1.0\" encoding=\"UTF-8\"?><alligator name=\"Mary\"/>"
        },
        "actual": {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/xml"},
          "body": "<?xml version=\"1.0\" encoding=\"UTF-8\"?><alligator name=\"\"/>"
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.get("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.get("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    if pact_match.as_bool().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn matches_with_regex_with_bracket_notation_xml() {
    env_logger::init().unwrap_or(());
    let pact : serde_json::Value = serde_json::from_str(r#"
      {
        "match": true,
        "comment": "XML Requests match with regex",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/xml"},
          "matchingRules": {
            "$.body['two']['@str']": {"match": "regex", "regex": "\\w+"}
          },
          "body": "<?xml version=\"1.0\" encoding=\"UTF-8\"?><two str=\"jildrdmxddnVzcQZfjCA\"/>"
        },
        "actual": {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/xml"},
          "body": "<?xml version=\"1.0\" encoding=\"UTF-8\"?><two str=\"saldfhksajdhffdskkjh\"/>"
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.get("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.get("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    if pact_match.as_bool().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn matches_with_regex_with_bracket_notation() {
    env_logger::init().unwrap_or(());
    let pact : serde_json::Value = serde_json::from_str(r#"
      {
        "match": true,
        "comment": "Requests match with regex",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/json"},
          "matchingRules": {
            "$.body['2'].str": {"match": "regex", "regex": "\\w+"}
          },
          "body": {
            "2" : {
              "str" : "jildrdmxddnVzcQZfjCA"
            }
          }
        },
        "actual": {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/json"},
          "body": {
            "2" : {
              "str" : "saldfhksajdhffdskkjh"
            }
          }
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.get("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.get("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    if pact_match.as_bool().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn matches_with_regex_xml() {
    env_logger::init().unwrap_or(());
    let pact : serde_json::Value = serde_json::from_str(r#"
      {
        "match": true,
        "comment": "XML Requests match with regex",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/xml"},
          "matchingRules": {
            "$.body.alligator['@name']": {"match": "regex", "regex": "\\w+"},
            "$.body.alligator[0].favouriteColours[0].favouriteColour": {"match": "regex", "regex": "red|blue"},
            "$.body.alligator[0].favouriteColours[1].favouriteColour": {"match": "regex", "regex": "red|blue"}
          },
          "body": "<?xml version=\"1.0\" encoding=\"UTF-8\"?><alligator name=\"Mary\" feet=\"4\"><favouriteColours><favouriteColour>red</favouriteColour><favouriteColour>blue</favouriteColour></favouriteColours></alligator>"
        },
        "actual": {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/xml"},
          "body": "<?xml version=\"1.0\" encoding=\"UTF-8\"?><alligator name=\"Harry\" feet=\"4\"><favouriteColours><favouriteColour>blue</favouriteColour><favouriteColour>red</favouriteColour></favouriteColours></alligator>"
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.get("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.get("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    if pact_match.as_bool().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn matches_with_regex() {
    env_logger::init().unwrap_or(());
    let pact : serde_json::Value = serde_json::from_str(r#"
      {
        "match": true,
        "comment": "Requests match with regex",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/json"},
          "matchingRules": {
            "$.body.alligator.name": {"match": "regex", "regex": "\\w+"},
            "$.body.alligator.favouriteColours[0]": {"match": "regex", "regex": "red|blue"},
            "$.body.alligator.favouriteColours[1]": {"match": "regex", "regex": "red|blue"}
          },
          "body": {
            "alligator":{
              "name": "Mary",
              "feet": 4,
              "favouriteColours": ["red","blue"]
            }
          }
        },
        "actual": {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator":{
              "feet": 4,
              "name": "Harry",
              "favouriteColours": ["blue", "red"]
            }
          }
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.get("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.get("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    if pact_match.as_bool().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn matches_with_type() {
    env_logger::init().unwrap_or(());
    let pact : serde_json::Value = serde_json::from_str(r#"
      {
        "match": true,
        "comment": "Requests match with same type",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/json"},
          "matchingRules": {
            "$.body.alligator.name": {"match": "type"},
            "$.body.alligator.feet": {"match": "type"}
          },
          "body": {
            "alligator":{
              "name": "Mary",
              "feet": 4,
              "favouriteColours": ["red","blue"]
            }
          }
        },
        "actual": {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator":{
              "feet": 5,
              "name": "Harry the very hungry alligator with an extra foot",
              "favouriteColours": ["red","blue"]
            }
          }
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.get("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.get("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    if pact_match.as_bool().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn matches_xml() {
    env_logger::init().unwrap_or(());
    let pact : serde_json::Value = serde_json::from_str(r#"
      {
        "match": true,
        "comment": "XML Requests match",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/xml"},
          "body": "<?xml version=\"1.0\" encoding=\"UTF-8\"?><alligator name=\"Mary\" feet=\"4\"><favouriteColours><favouriteColour>red</favouriteColour><favouriteColour>blue</favouriteColour></favouriteColours></alligator>"
        },
        "actual": {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/xml"},
          "body": "<?xml version=\"1.0\" encoding=\"UTF-8\"?><alligator feet=\"4\" name=\"Mary\"><favouriteColours><favouriteColour>red</favouriteColour><favouriteColour>blue</favouriteColour></favouriteColours></alligator>"
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.get("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.get("actual").unwrap(), &PactSpecification::V2);
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
    let pact : serde_json::Value = serde_json::from_str(r#"
      {
        "match": true,
        "comment": "Requests match",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator":{
              "name": "Mary",
              "feet": 4,
              "favouriteColours": ["red","blue"]
            }
          }
        },
        "actual": {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator":{
              "feet": 4,
              "name": "Mary",
              "favouriteColours": ["red","blue"]
            }
          }
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.get("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.get("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    if pact_match.as_bool().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn missing_body_found_when_empty_expected() {
    env_logger::init().unwrap_or(());
    let pact : serde_json::Value = serde_json::from_str(r#"
      {
        "match": true,
        "comment": "Missing body found, when an empty body was expected",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "body": null
        },
        "actual": {
          "method": "POST",
          "path": "/",
          "query": ""
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.get("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.get("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    if pact_match.as_bool().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn missing_body_no_content_type() {
    env_logger::init().unwrap_or(());
    let pact : serde_json::Value = serde_json::from_str(r#"
      {
        "match": true,
        "comment": "Missing body, no content-type",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": ""
        },
        "actual": {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator": {
              "age": 3
            }
          }
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.get("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.get("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    if pact_match.as_bool().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn missing_body() {
    env_logger::init().unwrap_or(());
    let pact : serde_json::Value = serde_json::from_str(r#"
      {
        "match": true,
        "comment": "Missing body",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/json"}
        },
        "actual": {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator": {
              "age": 3
            }
          }
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.get("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.get("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    if pact_match.as_bool().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn missing_index_xml() {
    env_logger::init().unwrap_or(());
    let pact : serde_json::Value = serde_json::from_str(r#"
      {
        "match": false,
        "comment": "XML Missing favorite colour",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/xml"},
          "body": "<?xml version=\"1.0\" encoding=\"UTF-8\"?><alligator><favouriteColours><favouriteColour>red</favouriteColour><favouriteColour>blue</favouriteColour></favouriteColours></alligator>"
        },
        "actual": {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/xml"},
          "body": "<?xml version=\"1.0\" encoding=\"UTF-8\"?><alligator><favouriteColours><favouriteColour>red</favouriteColour></favouriteColours></alligator>"
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.get("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.get("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    if pact_match.as_bool().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn missing_index() {
    env_logger::init().unwrap_or(());
    let pact : serde_json::Value = serde_json::from_str(r#"
      {
        "match": false,
        "comment": "Missing favorite colour",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator":{
              "favouriteColours": ["red","blue"]
            }
          }
        },
        "actual": {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator": {
              "favouriteColours": ["red"]
            }
          }
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.get("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.get("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    if pact_match.as_bool().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn missing_key_xml() {
    env_logger::init().unwrap_or(());
    let pact : serde_json::Value = serde_json::from_str(r#"
      {
        "match": false,
        "comment": "XML Missing key alligator name",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/xml"},
          "body": "<?xml version=\"1.0\" encoding=\"UTF-8\"?><alligator name=\"Mary\" age=\"3\"></alligator>"
        },
        "actual": {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/xml"},
          "body": "<?xml version=\"1.0\" encoding=\"UTF-8\"?><alligator age=\"3\"></alligator>"
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.get("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.get("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    if pact_match.as_bool().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn missing_key() {
    env_logger::init().unwrap_or(());
    let pact : serde_json::Value = serde_json::from_str(r#"
      {
        "match": false,
        "comment": "Missing key alligator name",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator":{
              "name": "Mary",
              "age": 3
            }
          }
        },
        "actual": {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator": {
              "age": 3
            }
          }
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.get("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.get("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    if pact_match.as_bool().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn no_body_no_content_type_xml() {
    env_logger::init().unwrap_or(());
    let pact : serde_json::Value = serde_json::from_str(r#"
      {
        "match": true,
        "comment": "XML No body, no content-type",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": ""
        },
        "actual": {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/xml"},
          "body": "<?xml version=\"1.0\" encoding=\"UTF-8\"?><alligator name=\"Mary\"/>"
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.get("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.get("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    if pact_match.as_bool().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn no_body_no_content_type() {
    env_logger::init().unwrap_or(());
    let pact : serde_json::Value = serde_json::from_str(r#"
      {
        "match": true,
        "comment": "No body, no content-type",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": ""
        },
        "actual": {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator": {
              "age": 3
            }
          }
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.get("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.get("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    if pact_match.as_bool().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn no_body_xml() {
    env_logger::init().unwrap_or(());
    let pact : serde_json::Value = serde_json::from_str(r#"
      {
        "match": true,
        "comment": "XML Missing body",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/xml"}
        },
        "actual": {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/xml"},
          "body": "<?xml version=\"1.0\" encoding=\"UTF-8\"?><alligator name=\"Mary\"/>"
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.get("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.get("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    if pact_match.as_bool().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn no_body() {
    env_logger::init().unwrap_or(());
    let pact : serde_json::Value = serde_json::from_str(r#"
      {
        "match": true,
        "comment": "Missing body",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/json"}
        },
        "actual": {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator": {
              "age": 3
            }
          }
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.get("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.get("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    if pact_match.as_bool().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn non_empty_body_found_when_empty_expected() {
    env_logger::init().unwrap_or(());
    let pact : serde_json::Value = serde_json::from_str(r#"
      {
        "match": false,
        "comment": "Non empty body found, when an empty body was expected",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/json"},
          "body": null
        },
        "actual": {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator": {
              "age": 3
            }
          }
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.get("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.get("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    if pact_match.as_bool().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn not_empty_found_at_key_when_empty_expected_xml() {
    env_logger::init().unwrap_or(());
    let pact : serde_json::Value = serde_json::from_str(r#"
      {
        "match": false,
        "comment": "XML Name should be empty",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/json"},
          "body": "<?xml version=\"1.0\" encoding=\"UTF-8\"?><alligator name=\"\"/>"
        },
        "actual": {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/xml"},
          "body": "<?xml version=\"1.0\" encoding=\"UTF-8\"?><alligator name=\"Mary\"/>"
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.get("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.get("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    if pact_match.as_bool().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn not_empty_found_in_array_when_empty_expected_xml() {
    env_logger::init().unwrap_or(());
    let pact : serde_json::Value = serde_json::from_str(r#"
      {
        "match": false,
        "comment": "XML Favourite numbers expected to contain empty, but non-empty found",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/xml"},
          "body": "<?xml version=\"1.0\" encoding=\"UTF-8\"?><alligator><favouriteNumbers><favouriteNumber>1</favouriteNumber><favouriteNumber></favouriteNumber><favouriteNumber>3</favouriteNumber></favouriteNumbers></alligator>"
        },
        "actual": {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/xml"},
          "body": "<?xml version=\"1.0\" encoding=\"UTF-8\"?><alligator><favouriteNumbers><favouriteNumber>1</favouriteNumber><favouriteNumber>2</favouriteNumber><favouriteNumber>3</favouriteNumber></favouriteNumbers></alligator>"
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.get("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.get("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    if pact_match.as_bool().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn not_null_found_at_key_when_null_expected() {
    env_logger::init().unwrap_or(());
    let pact : serde_json::Value = serde_json::from_str(r#"
      {
        "match": false,
        "comment": "Name should be null",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator":{
              "name": null
            }
          }
        },
        "actual": {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator":{
              "name": "Fred"
            }
          }
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.get("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.get("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    if pact_match.as_bool().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn not_null_found_in_array_when_null_expected() {
    env_logger::init().unwrap_or(());
    let pact : serde_json::Value = serde_json::from_str(r#"
      {
        "match": false,
        "comment": "Favourite colours expected to contain null, but not null found",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator":{
              "favouriteNumbers": ["1",null,"3"]
            }
          }
        },
        "actual": {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator":{
              "favouriteNumbers": ["1","2","3"]
            }
          }
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.get("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.get("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    if pact_match.as_bool().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn null_body_no_content_type() {
    env_logger::init().unwrap_or(());
    let pact : serde_json::Value = serde_json::from_str(r#"
      {
        "match": true,
        "comment": "NULL body, no content-type",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "body": null
        },
        "actual": {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/json"},
          "body": null
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.get("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.get("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    if pact_match.as_bool().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn null_body() {
    env_logger::init().unwrap_or(());
    let pact : serde_json::Value = serde_json::from_str(r#"
      {
        "match": true,
        "comment": "NULL body",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/json"},
          "body": null
        },
        "actual": {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/json"},
          "body": null
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.get("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.get("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    if pact_match.as_bool().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn null_found_at_key_where_not_null_expected() {
    env_logger::init().unwrap_or(());
    let pact : serde_json::Value = serde_json::from_str(r#"
      {
        "match": false,
        "comment": "Name should be null",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator":{
              "name": "Mary"
            }
          }
        },
        "actual": {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator":{
              "name": null
            }
          }
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.get("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.get("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    if pact_match.as_bool().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn null_found_in_array_when_not_null_expected() {
    env_logger::init().unwrap_or(());
    let pact : serde_json::Value = serde_json::from_str(r#"
      {
        "match": false,
        "comment": "Favourite colours expected to be strings found a null",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator":{
              "favouriteNumbers": ["1","2","3"]
            }
          }
        },
        "actual": {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator":{
              "favouriteNumbers": ["1",null,"3"]
            }
          }
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.get("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.get("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    if pact_match.as_bool().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn number_found_at_key_when_string_expected() {
    env_logger::init().unwrap_or(());
    let pact : serde_json::Value = serde_json::from_str(r#"
      {
        "match": false,
        "comment": "Number of feet expected to be string but was number",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator":{
              "feet": "4"
            }
          }
        },
        "actual": {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator":{
              "feet": 4
            }
          }
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.get("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.get("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    if pact_match.as_bool().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn number_found_in_array_when_string_expected() {
    env_logger::init().unwrap_or(());
    let pact : serde_json::Value = serde_json::from_str(r#"
      {
        "match": false,
        "comment": "Favourite colours expected to be strings found a number",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator":{
              "favouriteNumbers": ["1","2","3"]
            }
          }
        },
        "actual": {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator":{
              "favouriteNumbers": ["1",2,"3"]
            }
          }
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.get("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.get("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    if pact_match.as_bool().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn plain_text_that_does_not_match() {
    env_logger::init().unwrap_or(());
    let pact : serde_json::Value = serde_json::from_str(r#"
      {
        "match": false,
        "comment": "Plain text that does not match",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": { "Content-Type": "text/plain" },
          "body": "alligator named mary"
        },
        "actual": {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": { "Content-Type": "text/plain" },
          "body": "alligator named fred"
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.get("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.get("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    if pact_match.as_bool().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn plain_text_that_matches() {
    env_logger::init().unwrap_or(());
    let pact : serde_json::Value = serde_json::from_str(r#"
      {
        "match": true,
        "comment": "Plain text that matches",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": { "Content-Type": "text/plain" },
          "body": "alligator named mary"
        },
        "actual": {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": { "Content-Type": "text/plain" },
          "body": "alligator named mary"
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.get("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.get("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    if pact_match.as_bool().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn string_found_at_key_when_number_expected() {
    env_logger::init().unwrap_or(());
    let pact : serde_json::Value = serde_json::from_str(r#"
      {
        "match": false,
        "comment": "Number of feet expected to be number but was string",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator":{
              "feet": 4
            }
          }
        },
        "actual": {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator":{
              "feet": "4"
            }
          }
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.get("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.get("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    if pact_match.as_bool().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn string_found_in_array_when_number_expected() {
    env_logger::init().unwrap_or(());
    let pact : serde_json::Value = serde_json::from_str(r#"
      {
        "match": false,
        "comment": "Favourite Numbers expected to be numbers, but 2 is a string",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator":{
              "favouriteNumbers": [1,2,3]
            }
          }
        },
        "actual": {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator":{
              "favouriteNumbers": [1,"2",3]
            }
          }
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.get("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.get("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    if pact_match.as_bool().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn unexpected_index_with_missing_value_xml() {
    env_logger::init().unwrap_or(());
    let pact : serde_json::Value = serde_json::from_str(r#"
      {
        "match": false,
        "comment": "XML Unexpected favourite colour with empty value",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/xml"},
          "body": "<?xml version=\"1.0\" encoding=\"UTF-8\"?><alligator><favouriteColours><favouriteColour>red</favouriteColour><favouriteColour>blue</favouriteColour></favouriteColours></alligator>"
        },
        "actual": {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/xml"},
          "body": "<?xml version=\"1.0\" encoding=\"UTF-8\"?><alligator><favouriteColours><favouriteColour>red</favouriteColour><favouriteColour>blue</favouriteColour><favouriteColour></favouriteColour></favouriteColours></alligator>"
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.get("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.get("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    if pact_match.as_bool().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn unexpected_index_with_non_empty_value_xml() {
    env_logger::init().unwrap_or(());
    let pact : serde_json::Value = serde_json::from_str(r#"
      {
        "match": false,
        "comment": "XML Unexpected favourite colour",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/xml"},
          "body": "<?xml version=\"1.0\" encoding=\"UTF-8\"?><alligator><favouriteColours><favouriteColour>red</favouriteColour><favouriteColour>blue</favouriteColour></favouriteColours></alligator>"
        },
        "actual": {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/xml"},
          "body": "<?xml version=\"1.0\" encoding=\"UTF-8\"?><alligator><favouriteColours><favouriteColour>red</favouriteColour><favouriteColour>blue</favouriteColour><favouriteColour>taupe</favouriteColour></favouriteColours></alligator>"
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.get("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.get("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    if pact_match.as_bool().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn unexpected_index_with_not_null_value() {
    env_logger::init().unwrap_or(());
    let pact : serde_json::Value = serde_json::from_str(r#"
      {
        "match": false,
        "comment": "Unexpected favourite colour",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator":{
              "favouriteColours": ["red","blue"]
            }
          }
        },
        "actual": {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator":{
              "favouriteColours": ["red","blue","taupe"]
            }
          }
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.get("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.get("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    if pact_match.as_bool().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn unexpected_index_with_null_value() {
    env_logger::init().unwrap_or(());
    let pact : serde_json::Value = serde_json::from_str(r#"
      {
        "match": false,
        "comment": "Unexpected favourite colour with null value",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator":{
              "favouriteColours": ["red","blue"]
            }
          }
        },
        "actual": {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator":{
              "favouriteColours": ["red","blue", null]
            }
          }
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.get("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.get("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    if pact_match.as_bool().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn unexpected_key_with_empty_value_xml() {
    env_logger::init().unwrap_or(());
    let pact : serde_json::Value = serde_json::from_str(r#"
      {
        "match": false,
        "comment": "XML Unexpected phone number with empty value",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/xml"},
          "body": "<?xml version=\"1.0\" encoding=\"UTF-8\"?><alligator name=\"Mary\"/>"
        },
        "actual": {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/xml"},
          "body": "<?xml version=\"1.0\" encoding=\"UTF-8\"?><alligator name=\"Mary\" phoneNumber=\"\"/>"
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.get("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.get("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    if pact_match.as_bool().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn unexpected_key_with_non_empty_value_xml() {
    env_logger::init().unwrap_or(());
    let pact : serde_json::Value = serde_json::from_str(r#"
      {
        "match": false,
        "comment": "XML Unexpected phone number",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/xml"},
          "body": "<?xml version=\"1.0\" encoding=\"UTF-8\"?><alligator name=\"Mary\"/>"
        },
        "actual": {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/xml"},
          "body": "<?xml version=\"1.0\" encoding=\"UTF-8\"?><alligator name=\"Mary\" phoneNumber=\"12345678\"/>"
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.get("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.get("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    if pact_match.as_bool().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn unexpected_key_with_not_null_value() {
    env_logger::init().unwrap_or(());
    let pact : serde_json::Value = serde_json::from_str(r#"
      {
        "match": false,
        "comment": "Unexpected phone number",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator":{
              "name": "Mary"
            }
          }
        },
        "actual": {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator":{
              "name": "Mary",
              "phoneNumber": "12345678"
            }
          }
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.get("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.get("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    if pact_match.as_bool().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn unexpected_key_with_null_value() {
    env_logger::init().unwrap_or(());
    let pact : serde_json::Value = serde_json::from_str(r#"
      {
        "match": false,
        "comment": "Unexpected phone number with null value",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator":{
              "name": "Mary"
            }
          }
        },
        "actual": {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator":{
              "name": "Mary",
              "phoneNumber": null
            }
          }
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.get("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.get("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    if pact_match.as_bool().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn value_found_in_array_when_empty_expected_xml() {
    env_logger::init().unwrap_or(());
    let pact : serde_json::Value = serde_json::from_str(r#"
      {
        "match": false,
        "comment": "XML Favourite numbers expected to be strings found an empty value",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/xml"},
          "body": "<?xml version=\"1.0\" encoding=\"UTF-8\"?><alligator><favouriteNumbers><favouriteNumber>1</favouriteNumber><favouriteNumber>2</favouriteNumber><favouriteNumber>3</favouriteNumber></favouriteNumbers></alligator>"
        },
        "actual": {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/xml"},
          "body": "<?xml version=\"1.0\" encoding=\"UTF-8\"?><alligator><favouriteNumbers><favouriteNumber>1</favouriteNumber><favouriteNumber></favouriteNumber><favouriteNumber>3</favouriteNumber></favouriteNumbers></alligator>"
        }
      }
    "#).unwrap();

    let expected = Request::from_json(&pact.get("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.get("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.get("match").unwrap();
    if pact_match.as_bool().unwrap() {
       expect!(match_request(expected, actual)).to(be_empty());
    } else {
       expect!(match_request(expected, actual)).to_not(be_empty());
    }
}
