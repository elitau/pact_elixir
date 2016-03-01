use pact_mock_server_matchers::model::Request;
use rustc_serialize::json;

#[test]
fn array_in_different_order() {
    let pact = json!(
      {
        "match": false,
        "comment": "Favourite colours in wrong order",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {},
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
          "headers": {},
          "body": {
            "alligator":{
              "favouriteColours": ["blue", "red"]
            }
          }
        }
      }
    );

    let expected = Request::from_json(&pact.find("expected").unwrap());
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.find("actual").unwrap());
    println!("{:?}", expected);
    let pact_match = pact.find("match").unwrap();
    assert!(pact_match.as_boolean().unwrap());
}
      

#[test]
fn different_value_found_at_index() {
    let pact = json!(
      {
        "match": false,
        "comment": "Incorrect favourite colour",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {},
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
          "headers": {},
          "body": {
            "alligator":{
              "favouriteColours": ["red","taupe"]
            }
          }
        }
      }
    );

    let expected = Request::from_json(&pact.find("expected").unwrap());
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.find("actual").unwrap());
    println!("{:?}", expected);
    let pact_match = pact.find("match").unwrap();
    assert!(pact_match.as_boolean().unwrap());
}
      

#[test]
fn different_value_found_at_key() {
    let pact = json!(
      {
        "match": false,
        "comment": "Incorrect value at alligator name",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {},
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
          "headers": {},
          "body": {
            "alligator":{
              "name": "Fred"
            }
          }
        }
      }
    );

    let expected = Request::from_json(&pact.find("expected").unwrap());
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.find("actual").unwrap());
    println!("{:?}", expected);
    let pact_match = pact.find("match").unwrap();
    assert!(pact_match.as_boolean().unwrap());
}
      

#[test]
fn matches() {
    let pact = json!(
      {
        "match": true,
        "comment": "Requests match",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {},
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
          "headers": {},
          "body": {
            "alligator":{
              "feet": 4,
              "name": "Mary",
              "favouriteColours": ["red","blue"]
            }
          }
        }
      }
    );

    let expected = Request::from_json(&pact.find("expected").unwrap());
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.find("actual").unwrap());
    println!("{:?}", expected);
    let pact_match = pact.find("match").unwrap();
    assert!(pact_match.as_boolean().unwrap());
}
      

#[test]
fn missing_index() {
    let pact = json!(
      {
        "match": false,
        "comment": "Missing favorite colour",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {},
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
          "headers": {},
          "body": {
            "alligator": {
              "favouriteColours": ["red"]
            }
          }
        }
      }
    );

    let expected = Request::from_json(&pact.find("expected").unwrap());
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.find("actual").unwrap());
    println!("{:?}", expected);
    let pact_match = pact.find("match").unwrap();
    assert!(pact_match.as_boolean().unwrap());
}
      

#[test]
fn missing_key() {
    let pact = json!(
      {
        "match": false,
        "comment": "Missing key alligator name",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {},
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
          "headers": {},
          "body": {
            "alligator": {
              "age": 3
            }
          }
        }
      }
    );

    let expected = Request::from_json(&pact.find("expected").unwrap());
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.find("actual").unwrap());
    println!("{:?}", expected);
    let pact_match = pact.find("match").unwrap();
    assert!(pact_match.as_boolean().unwrap());
}
      

#[test]
fn not_null_found_at_key_when_null_expected() {
    let pact = json!(
      {
        "match": false,
        "comment": "Name should be null",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {},
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
          "headers": {},
          "body": {
            "alligator":{
              "name": "Fred"
            }
          }
        }
      }
    );

    let expected = Request::from_json(&pact.find("expected").unwrap());
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.find("actual").unwrap());
    println!("{:?}", expected);
    let pact_match = pact.find("match").unwrap();
    assert!(pact_match.as_boolean().unwrap());
}
      

#[test]
fn not_null_found_in_array_when_null_expected() {
    let pact = json!(
      {
        "match": false,
        "comment": "Favourite colours expected to contain null, but not null found",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {},
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
          "headers": {},
          "body": {
            "alligator":{
              "favouriteNumbers": ["1","2","3"]
            }
          }
        }
      }
    );

    let expected = Request::from_json(&pact.find("expected").unwrap());
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.find("actual").unwrap());
    println!("{:?}", expected);
    let pact_match = pact.find("match").unwrap();
    assert!(pact_match.as_boolean().unwrap());
}
      

#[test]
fn null_found_at_key_where_not_null_expected() {
    let pact = json!(
      {
        "match": false,
        "comment": "Name should be null",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {},
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
          "headers": {},
          "body": {
            "alligator":{
              "name": null
            }
          }
        }
      }
    );

    let expected = Request::from_json(&pact.find("expected").unwrap());
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.find("actual").unwrap());
    println!("{:?}", expected);
    let pact_match = pact.find("match").unwrap();
    assert!(pact_match.as_boolean().unwrap());
}
      

#[test]
fn null_found_in_array_when_not_null_expected() {
    let pact = json!(
      {
        "match": false,
        "comment": "Favourite colours expected to be strings found a null",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {},
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
          "headers": {},
          "body": {
            "alligator":{
              "favouriteNumbers": ["1",null,"3"]
            }
          }
        }
      }
    );

    let expected = Request::from_json(&pact.find("expected").unwrap());
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.find("actual").unwrap());
    println!("{:?}", expected);
    let pact_match = pact.find("match").unwrap();
    assert!(pact_match.as_boolean().unwrap());
}
      

#[test]
fn number_found_at_key_when_string_expected() {
    let pact = json!(
      {
        "match": false,
        "comment": "Number of feet expected to be string but was number",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {},
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
          "headers": {},
          "body": {
            "alligator":{
              "feet": 4
            }
          }
        }
      }
    );

    let expected = Request::from_json(&pact.find("expected").unwrap());
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.find("actual").unwrap());
    println!("{:?}", expected);
    let pact_match = pact.find("match").unwrap();
    assert!(pact_match.as_boolean().unwrap());
}
      

#[test]
fn number_found_in_array_when_string_expected() {
    let pact = json!(
      {
        "match": false,
        "comment": "Favourite colours expected to be strings found a number",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {},
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
          "headers": {},
          "body": {
            "alligator":{
              "favouriteNumbers": ["1",2,"3"]
            }
          }
        }
      }
    );

    let expected = Request::from_json(&pact.find("expected").unwrap());
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.find("actual").unwrap());
    println!("{:?}", expected);
    let pact_match = pact.find("match").unwrap();
    assert!(pact_match.as_boolean().unwrap());
}
      

#[test]
fn plain_text_that_does_not_match() {
    let pact = json!(
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
    );

    let expected = Request::from_json(&pact.find("expected").unwrap());
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.find("actual").unwrap());
    println!("{:?}", expected);
    let pact_match = pact.find("match").unwrap();
    assert!(pact_match.as_boolean().unwrap());
}
      

#[test]
fn plain_text_that_matches() {
    let pact = json!(
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
    );

    let expected = Request::from_json(&pact.find("expected").unwrap());
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.find("actual").unwrap());
    println!("{:?}", expected);
    let pact_match = pact.find("match").unwrap();
    assert!(pact_match.as_boolean().unwrap());
}
      

#[test]
fn string_found_at_key_when_number_expected() {
    let pact = json!(
      {
        "match": false,
        "comment": "Number of feet expected to be number but was string",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {},
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
          "headers": {},
          "body": {
            "alligator":{
              "feet": "4"
            }
          }
        }
      }
    );

    let expected = Request::from_json(&pact.find("expected").unwrap());
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.find("actual").unwrap());
    println!("{:?}", expected);
    let pact_match = pact.find("match").unwrap();
    assert!(pact_match.as_boolean().unwrap());
}
      

#[test]
fn string_found_in_array_when_number_expected() {
    let pact = json!(
      {
        "match": false,
        "comment": "Favourite Numbers expected to be numbers, but 2 is a string",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {},
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
          "headers": {},
          "body": {
            "alligator":{
              "favouriteNumbers": [1,"2",3]
            }
          }
        }
      }
    );

    let expected = Request::from_json(&pact.find("expected").unwrap());
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.find("actual").unwrap());
    println!("{:?}", expected);
    let pact_match = pact.find("match").unwrap();
    assert!(pact_match.as_boolean().unwrap());
}
      

#[test]
fn unexpected_index_with_not_null_value() {
    let pact = json!(
      {
        "match": false,
        "comment": "Unexpected favourite colour",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {},
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
          "headers": {},
          "body": {
            "alligator":{
              "favouriteColours": ["red","blue","taupe"]
            }
          }
        }
      }
    );

    let expected = Request::from_json(&pact.find("expected").unwrap());
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.find("actual").unwrap());
    println!("{:?}", expected);
    let pact_match = pact.find("match").unwrap();
    assert!(pact_match.as_boolean().unwrap());
}
      

#[test]
fn unexpected_index_with_null_value() {
    let pact = json!(
      {
        "match": false,
        "comment": "Unexpected favourite colour with null value",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {},
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
          "headers": {},
          "body": {
            "alligator":{
              "favouriteColours": ["red","blue", null]
            }
          }
        }
      }
    );

    let expected = Request::from_json(&pact.find("expected").unwrap());
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.find("actual").unwrap());
    println!("{:?}", expected);
    let pact_match = pact.find("match").unwrap();
    assert!(pact_match.as_boolean().unwrap());
}
      

#[test]
fn unexpected_key_with_not_null_value() {
    let pact = json!(
      {
        "match": false,
        "comment": "Unexpected phone number",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {},
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
          "headers": {},
          "body": {
            "alligator":{
              "name": "Mary",
              "phoneNumber": "12345678"
            }
          }
        }
      }
    );

    let expected = Request::from_json(&pact.find("expected").unwrap());
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.find("actual").unwrap());
    println!("{:?}", expected);
    let pact_match = pact.find("match").unwrap();
    assert!(pact_match.as_boolean().unwrap());
}
      

#[test]
fn unexpected_key_with_null_value() {
    let pact = json!(
      {
        "match": false,
        "comment": "Unexpected phone number with null value",
        "expected" : {
          "method": "POST",
          "path": "/",
          "query": "",
          "headers": {},
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
          "headers": {},
          "body": {
            "alligator":{
              "name": "Mary",
              "phoneNumber": null
            }
          }
        }
      }
    );

    let expected = Request::from_json(&pact.find("expected").unwrap());
    println!("{:?}", expected);
    let actual = Request::from_json(&pact.find("actual").unwrap());
    println!("{:?}", expected);
    let pact_match = pact.find("match").unwrap();
    assert!(pact_match.as_boolean().unwrap());
}
      
