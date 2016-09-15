#[allow(unused_imports)]
use pact_matching::models::*;
#[allow(unused_imports)]
use env_logger;
#[allow(unused_imports)]
use pact_matching::match_response;
#[allow(unused_imports)]
use rustc_serialize::json::Json;
#[allow(unused_imports)]
use expectest::prelude::*;

#[test]
fn array_in_different_order() {
    env_logger::init().unwrap_or(());
    let pact = Json::from_str(r#"
      {
        "match": false,
        "comment": "Favourite colours in wrong order",
        "expected" : {
          "headers": {},
          "body": {
            "alligator":{
              "favouriteColours": ["red","blue"]
            }
          }
        },
        "actual": {
          "headers": {},
          "body": {
            "alligator":{
              "favouriteColours": ["blue", "red"]
            }
          }
        }
      }
    "#).unwrap();

    let expected = Response::from_json(&pact.find("expected").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    let result = match_response(expected, actual);
    if pact_match.as_boolean().unwrap() {
       expect!(result).to(be_empty());
    } else {
       expect!(result).to_not(be_empty());
    }
}

#[test]
fn deeply_nested_objects() {
    env_logger::init().unwrap_or(());
    let pact = Json::from_str(r#"
      {
      	"match": true,
      	"comment": "Comparisons should work even on nested objects",
      	"expected" : {
      		"headers": {},
      		"body": {
      			"object1": {
      				"object2": { 
      					"object4": {
      						"object5": {
      							"name": "Mary",
      							"friends": ["Fred", "John"]
      						},
      						"object6": {
      							"phoneNumber": 1234567890
      						}
      					}
      				}
      			}
      		}
      	},
      	"actual": {
      		"headers": {},
      		"body": {
      			"object1":{
      				"object2": { 
      					"object4":{
      						"object5": {
      							"name": "Mary",
      							"friends": ["Fred", "John"],
      							"gender": "F"
      						},
      						"object6": {
      							"phoneNumber": 1234567890
      						}
      					}
      				},
      				"color": "red"
      			}
      		}
      	}
      }
    "#).unwrap();

    let expected = Response::from_json(&pact.find("expected").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    let result = match_response(expected, actual);
    if pact_match.as_boolean().unwrap() {
       expect!(result).to(be_empty());
    } else {
       expect!(result).to_not(be_empty());
    }
}

#[test]
fn different_value_found_at_index() {
    env_logger::init().unwrap_or(());
    let pact = Json::from_str(r#"
      {
        "match": false,
        "comment": "Incorrect favourite colour",
        "expected" : {
          "headers": {},
          "body": {
            "alligator":{
              "favouriteColours": ["red","blue"]
            }
          }
        },
        "actual": {
          "headers": {},
          "body": {
            "alligator":{
              "favouriteColours": ["red","taupe"]
            }
          }
        }
      }
    "#).unwrap();

    let expected = Response::from_json(&pact.find("expected").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    let result = match_response(expected, actual);
    if pact_match.as_boolean().unwrap() {
       expect!(result).to(be_empty());
    } else {
       expect!(result).to_not(be_empty());
    }
}

#[test]
fn different_value_found_at_key() {
    env_logger::init().unwrap_or(());
    let pact = Json::from_str(r#"
      {
        "match": false,
        "comment": "Incorrect value at alligator name",
        "expected" : {
          "headers": {},
          "body": {
            "alligator":{
              "name": "Mary"
            }
          }
        },
        "actual": {
          "headers": {},
          "body": {
            "alligator":{
              "name": "Fred"
            }
          }
        }
      }
    "#).unwrap();

    let expected = Response::from_json(&pact.find("expected").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    let result = match_response(expected, actual);
    if pact_match.as_boolean().unwrap() {
       expect!(result).to(be_empty());
    } else {
       expect!(result).to_not(be_empty());
    }
}

#[test]
fn empty_body_no_content_type() {
    env_logger::init().unwrap_or(());
    let pact = Json::from_str(r#"
      {
        "match": true,
        "comment": "Empty body, no content-type",
        "expected" : {
          "body": ""
        },
        "actual": {
          "headers": {"Content-Type": "application/json"},
          "body": ""
        }
      }
    "#).unwrap();

    let expected = Response::from_json(&pact.find("expected").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    let result = match_response(expected, actual);
    if pact_match.as_boolean().unwrap() {
       expect!(result).to(be_empty());
    } else {
       expect!(result).to_not(be_empty());
    }
}

#[test]
fn empty_body() {
    env_logger::init().unwrap_or(());
    let pact = Json::from_str(r#"
      {
        "match": true,
        "comment": "Empty body",
        "expected" : {
          "headers": {"Content-Type": "application/json"},
          "body": ""
        },
        "actual": {
          "headers": {"Content-Type": "application/json"},
          "body": ""
        }
      }
    "#).unwrap();

    let expected = Response::from_json(&pact.find("expected").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    let result = match_response(expected, actual);
    if pact_match.as_boolean().unwrap() {
       expect!(result).to(be_empty());
    } else {
       expect!(result).to_not(be_empty());
    }
}

#[test]
fn keys_out_of_order_match() {
    env_logger::init().unwrap_or(());
    let pact = Json::from_str(r#"
      {
        "match": true,
        "comment": "Favourite number and favourite colours out of order",
        "expected" : {
          "headers": {},
          "body": {
      		"favouriteNumber": 7,
              "favouriteColours": ["red","blue"]
          }
        },
        "actual": {
          "headers": {},
          "body": {
              "favouriteColours": ["red","blue"],
      		"favouriteNumber": 7
          }
        }
      }
    "#).unwrap();

    let expected = Response::from_json(&pact.find("expected").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    let result = match_response(expected, actual);
    if pact_match.as_boolean().unwrap() {
       expect!(result).to(be_empty());
    } else {
       expect!(result).to_not(be_empty());
    }
}

#[test]
fn matches() {
    env_logger::init().unwrap_or(());
    let pact = Json::from_str(r#"
      {
        "match": true,
        "comment": "Responses match",
        "expected" : {
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
    "#).unwrap();

    let expected = Response::from_json(&pact.find("expected").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    let result = match_response(expected, actual);
    if pact_match.as_boolean().unwrap() {
       expect!(result).to(be_empty());
    } else {
       expect!(result).to_not(be_empty());
    }
}

#[test]
fn missing_body_found_when_empty_expected() {
    env_logger::init().unwrap_or(());
    let pact = Json::from_str(r#"
      {
        "match": true,
        "comment": "Missing body found, when an empty body was expected",
        "expected" : {
          "body": null
        },
        "actual": {
        }
      }
    "#).unwrap();

    let expected = Response::from_json(&pact.find("expected").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    let result = match_response(expected, actual);
    if pact_match.as_boolean().unwrap() {
       expect!(result).to(be_empty());
    } else {
       expect!(result).to_not(be_empty());
    }
}

#[test]
fn missing_body_no_content_type() {
    env_logger::init().unwrap_or(());
    let pact = Json::from_str(r#"
      {
        "match": true,
        "comment": "Missing body, no content-type",
        "expected" : {
        },
        "actual": {
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

    let expected = Response::from_json(&pact.find("expected").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    let result = match_response(expected, actual);
    if pact_match.as_boolean().unwrap() {
       expect!(result).to(be_empty());
    } else {
       expect!(result).to_not(be_empty());
    }
}

#[test]
fn missing_body() {
    env_logger::init().unwrap_or(());
    let pact = Json::from_str(r#"
      {
        "match": true,
        "comment": "Missing body",
        "expected" : {
          "headers": {"Content-Type": "application/json"}
        },
        "actual": {
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

    let expected = Response::from_json(&pact.find("expected").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    let result = match_response(expected, actual);
    if pact_match.as_boolean().unwrap() {
       expect!(result).to(be_empty());
    } else {
       expect!(result).to_not(be_empty());
    }
}

#[test]
fn missing_index() {
    env_logger::init().unwrap_or(());
    let pact = Json::from_str(r#"
      {
        "match": false,
        "comment": "Missing favorite colour",
        "expected" : {
          "headers": {},
          "body": {
            "alligator":{
              "favouriteColours": ["red","blue"]
            }
          }
        },
        "actual": {
          "headers": {},
          "body": {
            "alligator": {
              "favouriteColours": ["red"]
            }
          }
        }
      }
    "#).unwrap();

    let expected = Response::from_json(&pact.find("expected").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    let result = match_response(expected, actual);
    if pact_match.as_boolean().unwrap() {
       expect!(result).to(be_empty());
    } else {
       expect!(result).to_not(be_empty());
    }
}

#[test]
fn missing_key() {
    env_logger::init().unwrap_or(());
    let pact = Json::from_str(r#"
      {
        "match": false,
        "comment": "Missing key alligator name",
        "expected" : {
          "headers": {},
          "body": {
            "alligator":{
              "name": "Mary",
              "age": 3
            }
          }
        },
        "actual": {
          "headers": {},
          "body": {
            "alligator": {
              "age": 3
            }
          }
        }
      }
    "#).unwrap();

    let expected = Response::from_json(&pact.find("expected").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    let result = match_response(expected, actual);
    if pact_match.as_boolean().unwrap() {
       expect!(result).to(be_empty());
    } else {
       expect!(result).to_not(be_empty());
    }
}

#[test]
fn non_empty_body_found_when_empty_expected() {
    env_logger::init().unwrap_or(());
    let pact = Json::from_str(r#"
      {
        "match": false,
        "comment": "Non empty body found, when an empty body was expected",
        "expected" : {
          "headers": {"Content-Type": "application/json"},
          "body": null
        },
        "actual": {
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

    let expected = Response::from_json(&pact.find("expected").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    let result = match_response(expected, actual);
    if pact_match.as_boolean().unwrap() {
       expect!(result).to(be_empty());
    } else {
       expect!(result).to_not(be_empty());
    }
}

#[test]
fn not_null_found_at_key_when_null_expected() {
    env_logger::init().unwrap_or(());
    let pact = Json::from_str(r#"
      {
        "match": false,
        "comment": "Name should be null",
        "expected" : {
          "headers": {},
          "body": {
            "alligator":{
              "name": null
            }
          }
        },
        "actual": {
          "headers": {},
          "body": {
            "alligator":{
              "name": "Fred"
            }
          }
        }
      }
    "#).unwrap();

    let expected = Response::from_json(&pact.find("expected").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    let result = match_response(expected, actual);
    if pact_match.as_boolean().unwrap() {
       expect!(result).to(be_empty());
    } else {
       expect!(result).to_not(be_empty());
    }
}

#[test]
fn not_null_found_in_array_when_null_expected() {
    env_logger::init().unwrap_or(());
    let pact = Json::from_str(r#"
      {
        "match": false,
        "comment": "Favourite numbers expected to contain null, but not null found",
        "expected" : {
          "headers": {},
          "body": {
            "alligator":{
              "favouriteNumbers": ["1",null,"3"]
            }
          }
        },
        "actual": {
          "headers": {},
          "body": {
            "alligator":{
              "favouriteNumbers": ["1","2","3"]
            }
          }
        }
      }
    "#).unwrap();

    let expected = Response::from_json(&pact.find("expected").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    let result = match_response(expected, actual);
    if pact_match.as_boolean().unwrap() {
       expect!(result).to(be_empty());
    } else {
       expect!(result).to_not(be_empty());
    }
}

#[test]
fn null_body_no_content_type() {
    env_logger::init().unwrap_or(());
    let pact = Json::from_str(r#"
      {
        "match": true,
        "comment": "NULL body, no content-type",
        "expected" : {
          "body": null
        },
        "actual": {
          "headers": {"Content-Type": "application/json"},
          "body": null
        }
      }
    "#).unwrap();

    let expected = Response::from_json(&pact.find("expected").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    let result = match_response(expected, actual);
    if pact_match.as_boolean().unwrap() {
       expect!(result).to(be_empty());
    } else {
       expect!(result).to_not(be_empty());
    }
}

#[test]
fn null_body() {
    env_logger::init().unwrap_or(());
    let pact = Json::from_str(r#"
      {
        "match": true,
        "comment": "NULL body",
        "expected" : {
          "headers": {"Content-Type": "application/json"},
          "body": null
        },
        "actual": {
          "headers": {"Content-Type": "application/json"},
          "body": null
        }
      }
    "#).unwrap();

    let expected = Response::from_json(&pact.find("expected").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    let result = match_response(expected, actual);
    if pact_match.as_boolean().unwrap() {
       expect!(result).to(be_empty());
    } else {
       expect!(result).to_not(be_empty());
    }
}

#[test]
fn null_found_at_key_where_not_null_expected() {
    env_logger::init().unwrap_or(());
    let pact = Json::from_str(r#"
      {
        "match": false,
        "comment": "Name should not be null",
        "expected" : {
          "headers": {},
          "body": {
            "alligator":{
              "name": "Mary"
            }
          }
        },
        "actual": {
          "headers": {},
          "body": {
            "alligator":{
              "name": null
            }
          }
        }
      }
    "#).unwrap();

    let expected = Response::from_json(&pact.find("expected").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    let result = match_response(expected, actual);
    if pact_match.as_boolean().unwrap() {
       expect!(result).to(be_empty());
    } else {
       expect!(result).to_not(be_empty());
    }
}

#[test]
fn null_found_in_array_when_not_null_expected() {
    env_logger::init().unwrap_or(());
    let pact = Json::from_str(r#"
      {
        "match": false,
        "comment": "Favourite numbers expected to be strings found a null",
        "expected" : {
          "headers": {},
          "body": {
            "alligator":{
              "favouriteNumbers": ["1","2","3"]
            }
          }
        },
        "actual": {
          "headers": {},
          "body": {
            "alligator":{
              "favouriteNumbers": ["1",null,"3"]
            }
          }
        }
      }
    "#).unwrap();

    let expected = Response::from_json(&pact.find("expected").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    let result = match_response(expected, actual);
    if pact_match.as_boolean().unwrap() {
       expect!(result).to(be_empty());
    } else {
       expect!(result).to_not(be_empty());
    }
}

#[test]
fn number_found_at_key_when_string_expected() {
    env_logger::init().unwrap_or(());
    let pact = Json::from_str(r#"
      {
        "match": false,
        "comment": "Number of feet expected to be string but was number",
        "expected" : {
          "headers": {},
          "body": {
            "alligator":{
              "feet": "4"
            }
          }
        },
        "actual": {
          "headers": {},
          "body": {
            "alligator":{
              "feet": 4
            }
          }
        }
      }
    "#).unwrap();

    let expected = Response::from_json(&pact.find("expected").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    let result = match_response(expected, actual);
    if pact_match.as_boolean().unwrap() {
       expect!(result).to(be_empty());
    } else {
       expect!(result).to_not(be_empty());
    }
}

#[test]
fn number_found_in_array_when_string_expected() {
    env_logger::init().unwrap_or(());
    let pact = Json::from_str(r#"
      {
        "match": false,
        "comment": "Favourite numbers expected to be strings found a number",
        "expected" : {
          "headers": {},
          "body": {
            "alligator":{
              "favouriteNumbers": ["1","2","3"]
            }
          }
        },
        "actual": {
          "headers": {},
          "body": {
            "alligator":{
              "favouriteNumbers": ["1",2,"3"]
            }
          }
        }
      }
    "#).unwrap();

    let expected = Response::from_json(&pact.find("expected").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    let result = match_response(expected, actual);
    if pact_match.as_boolean().unwrap() {
       expect!(result).to(be_empty());
    } else {
       expect!(result).to_not(be_empty());
    }
}

#[test]
fn objects_in_array_first_matches() {
    env_logger::init().unwrap_or(());
    let pact = Json::from_str(r#"
      {
        "match": false,
        "comment": "Properties match but unexpected element recieved",
        "expected" : {
          "headers": {},
          "body": [
      		{"favouriteColor": "red"}
      	]
        },
        "actual": {
          "headers": {},
          "body": [
      		{"favouriteColor": "red",
      		"favouriteNumber": 2},
      		{"favouriteColor": "blue",
      		"favouriteNumber": 2}
      	]
        }
      }
    "#).unwrap();

    let expected = Response::from_json(&pact.find("expected").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    let result = match_response(expected, actual);
    if pact_match.as_boolean().unwrap() {
       expect!(result).to(be_empty());
    } else {
       expect!(result).to_not(be_empty());
    }
}

#[test]
fn objects_in_array_no_matches() {
    env_logger::init().unwrap_or(());
    let pact = Json::from_str(r#"
      {
        "match": false,
        "comment": "Array of objects, properties match on incorrect objects",
        "expected" : {
          "headers": {},
          "body": [
      		{"favouriteColor": "red"},
      		{"favouriteNumber": 2}
      	]
        },
        "actual": {
          "headers": {},
          "body": [
      		{"favouriteColor": "blue",
      		"favouriteNumber": 4},
      		{"favouriteColor": "red",
      		"favouriteNumber": 2}
      	]
        }
      }
    "#).unwrap();

    let expected = Response::from_json(&pact.find("expected").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    let result = match_response(expected, actual);
    if pact_match.as_boolean().unwrap() {
       expect!(result).to(be_empty());
    } else {
       expect!(result).to_not(be_empty());
    }
}

#[test]
fn objects_in_array_second_matches() {
    env_logger::init().unwrap_or(());
    let pact = Json::from_str(r#"
      {
        "match": false,
        "comment": "Property of second object matches, but unexpected element recieved",
        "expected" : {
          "headers": {},
          "body": [
      		{"favouriteColor": "red"}
      	]
        },
        "actual": {
          "headers": {},
          "body": [
      		{"favouriteColor": "blue",
      		"favouriteNumber": 4},
      		{"favouriteColor": "red",
      		"favouriteNumber": 2}
      	]
        }
      }
    "#).unwrap();

    let expected = Response::from_json(&pact.find("expected").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    let result = match_response(expected, actual);
    if pact_match.as_boolean().unwrap() {
       expect!(result).to(be_empty());
    } else {
       expect!(result).to_not(be_empty());
    }
}

#[test]
fn plain_text_that_does_not_match() {
    env_logger::init().unwrap_or(());
    let pact = Json::from_str(r#"
      {
        "match": false,
        "comment": "Plain text that does not match",
        "expected" : {
          "headers": { "Content-Type": "text/plain" },
          "body": "alligator named mary"
        },
        "actual": {
          "headers": { "Content-Type": "text/plain" },
          "body": "alligator named fred"
        }
      }
    "#).unwrap();

    let expected = Response::from_json(&pact.find("expected").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    let result = match_response(expected, actual);
    if pact_match.as_boolean().unwrap() {
       expect!(result).to(be_empty());
    } else {
       expect!(result).to_not(be_empty());
    }
}

#[test]
fn plain_text_that_matches() {
    env_logger::init().unwrap_or(());
    let pact = Json::from_str(r#"
      {
        "match": true,
        "comment": "Plain text that matches",
        "expected" : {
          "headers": { "Content-Type": "text/plain" },
          "body": "alligator named mary"
        },
        "actual": {
          "headers": { "Content-Type": "text/plain" },
          "body": "alligator named mary"
        }
      }
    "#).unwrap();

    let expected = Response::from_json(&pact.find("expected").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    let result = match_response(expected, actual);
    if pact_match.as_boolean().unwrap() {
       expect!(result).to(be_empty());
    } else {
       expect!(result).to_not(be_empty());
    }
}

#[test]
fn property_name_is_different_case() {
    env_logger::init().unwrap_or(());
    let pact = Json::from_str(r#"
      {
        "match": false,
        "comment": "Property names on objects are case sensitive",
        "expected" : {
          "headers": {},
          "body": {
            "alligator":{
              "FavouriteColour": "red"
            }
          }
        },
        "actual": {
          "headers": {},
          "body": {
            "alligator":{
              "favouritecolour": "red"
            }
          }
        }
      }
    "#).unwrap();

    let expected = Response::from_json(&pact.find("expected").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    let result = match_response(expected, actual);
    if pact_match.as_boolean().unwrap() {
       expect!(result).to(be_empty());
    } else {
       expect!(result).to_not(be_empty());
    }
}

#[test]
fn string_found_at_key_when_number_expected() {
    env_logger::init().unwrap_or(());
    let pact = Json::from_str(r#"
      {
        "match": false,
        "comment": "Number of feet expected to be number but was string",
        "expected" : {
          "headers": {},
          "body": {
            "alligator":{
              "feet": 4
            }
          }
        },
        "actual": {
          "headers": {},
          "body": {
            "alligator":{
              "feet": "4"
            }
          }
        }
      }
    "#).unwrap();

    let expected = Response::from_json(&pact.find("expected").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    let result = match_response(expected, actual);
    if pact_match.as_boolean().unwrap() {
       expect!(result).to(be_empty());
    } else {
       expect!(result).to_not(be_empty());
    }
}

#[test]
fn string_found_in_array_when_number_expected() {
    env_logger::init().unwrap_or(());
    let pact = Json::from_str(r#"
      {
        "match": false,
        "comment": "Favourite Numbers expected to be numbers, but 2 is a string",
        "expected" : {
          "headers": {},
          "body": {
            "alligator":{
              "favouriteNumbers": [1,2,3]
            }
          }
        },
        "actual": {
          "headers": {},
          "body": {
            "alligator":{
              "favouriteNumbers": [1,"2",3]
            }
          }
        }
      }
    "#).unwrap();

    let expected = Response::from_json(&pact.find("expected").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    let result = match_response(expected, actual);
    if pact_match.as_boolean().unwrap() {
       expect!(result).to(be_empty());
    } else {
       expect!(result).to_not(be_empty());
    }
}

#[test]
fn unexpected_index_with_not_null_value() {
    env_logger::init().unwrap_or(());
    let pact = Json::from_str(r#"
      {
        "match": false,
        "comment": "Unexpected favourite colour",
        "expected" : {
          "headers": {},
          "body": {
            "alligator":{
              "favouriteColours": ["red","blue"]
            }
          }
        },
        "actual": {
          "headers": {},
          "body": {
            "alligator":{
              "favouriteColours": ["red","blue","taupe"]
            }
          }
        }
      }
    "#).unwrap();

    let expected = Response::from_json(&pact.find("expected").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    let result = match_response(expected, actual);
    if pact_match.as_boolean().unwrap() {
       expect!(result).to(be_empty());
    } else {
       expect!(result).to_not(be_empty());
    }
}

#[test]
fn unexpected_index_with_null_value() {
    env_logger::init().unwrap_or(());
    let pact = Json::from_str(r#"
      {
        "match": false,
        "comment": "Unexpected favourite colour with null value",
        "expected" : {
          "headers": {},
          "body": {
            "alligator":{
              "favouriteColours": ["red","blue"]
            }
          }
        },
        "actual": {
          "headers": {},
          "body": {
            "alligator":{
              "favouriteColours": ["red","blue", null]
            }
          }
        }
      }
    "#).unwrap();

    let expected = Response::from_json(&pact.find("expected").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    let result = match_response(expected, actual);
    if pact_match.as_boolean().unwrap() {
       expect!(result).to(be_empty());
    } else {
       expect!(result).to_not(be_empty());
    }
}

#[test]
fn unexpected_key_with_not_null_value() {
    env_logger::init().unwrap_or(());
    let pact = Json::from_str(r#"
      {
        "match": true,
        "comment": "Unexpected phone number",
        "expected" : {
          "headers": {},
          "body": {
            "alligator":{
              "name": "Mary"
            }
          }
        },
        "actual": {
          "headers": {},
          "body": {
            "alligator":{
              "name": "Mary",
              "phoneNumber": "12345678"
            }
          }
        }
      }
    "#).unwrap();

    let expected = Response::from_json(&pact.find("expected").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    let result = match_response(expected, actual);
    if pact_match.as_boolean().unwrap() {
       expect!(result).to(be_empty());
    } else {
       expect!(result).to_not(be_empty());
    }
}

#[test]
fn unexpected_key_with_null_value() {
    env_logger::init().unwrap_or(());
    let pact = Json::from_str(r#"
      {
        "match": true,
        "comment": "Unexpected phone number with null value",
        "expected" : {
          "headers": {},
          "body": {
            "alligator":{
              "name": "Mary"
            }
          }
        },
        "actual": {
          "headers": {},
          "body": {
            "alligator":{
              "name": "Mary",
              "phoneNumber": null
            }
          }
        }
      }
    "#).unwrap();

    let expected = Response::from_json(&pact.find("expected").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap(), &PactSpecification::V1_1);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    let result = match_response(expected, actual);
    if pact_match.as_boolean().unwrap() {
       expect!(result).to(be_empty());
    } else {
       expect!(result).to_not(be_empty());
    }
}
