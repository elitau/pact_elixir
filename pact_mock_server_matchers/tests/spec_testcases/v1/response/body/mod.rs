use pact_mock_server_matchers::model::Response;
use pact_mock_server_matchers::match_response;
use rustc_serialize::json;

#[test]
fn array_in_different_order() {
    let pact = json!(
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
    );

    let expected = Response::from_json(&pact.find("expected").unwrap());
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap());
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       //assert!(match_response(expected, actual).is_empty(), "Favourite colours in wrong order");
    } else {
       //assert!(!match_response(expected, actual).is_empty(), "Favourite colours in wrong order");
    }
}

#[test]
fn deeply_nested_objects() {
    let pact = json!(
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
    );

    let expected = Response::from_json(&pact.find("expected").unwrap());
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap());
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       //assert!(match_response(expected, actual).is_empty(), "Comparisons should work even on nested objects");
    } else {
       //assert!(!match_response(expected, actual).is_empty(), "Comparisons should work even on nested objects");
    }
}

#[test]
fn different_value_found_at_index() {
    let pact = json!(
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
    );

    let expected = Response::from_json(&pact.find("expected").unwrap());
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap());
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       //assert!(match_response(expected, actual).is_empty(), "Incorrect favourite colour");
    } else {
       //assert!(!match_response(expected, actual).is_empty(), "Incorrect favourite colour");
    }
}

#[test]
fn different_value_found_at_key() {
    let pact = json!(
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
    );

    let expected = Response::from_json(&pact.find("expected").unwrap());
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap());
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       //assert!(match_response(expected, actual).is_empty(), "Incorrect value at alligator name");
    } else {
       //assert!(!match_response(expected, actual).is_empty(), "Incorrect value at alligator name");
    }
}

#[test]
fn keys_out_of_order_match() {
    let pact = json!(
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
    );

    let expected = Response::from_json(&pact.find("expected").unwrap());
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap());
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       //assert!(match_response(expected, actual).is_empty(), "Favourite number and favourite colours out of order");
    } else {
       //assert!(!match_response(expected, actual).is_empty(), "Favourite number and favourite colours out of order");
    }
}

#[test]
fn matches() {
    let pact = json!(
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
    );

    let expected = Response::from_json(&pact.find("expected").unwrap());
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap());
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       //assert!(match_response(expected, actual).is_empty(), "Responses match");
    } else {
       //assert!(!match_response(expected, actual).is_empty(), "Responses match");
    }
}

#[test]
fn missing_index() {
    let pact = json!(
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
    );

    let expected = Response::from_json(&pact.find("expected").unwrap());
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap());
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       //assert!(match_response(expected, actual).is_empty(), "Missing favorite colour");
    } else {
       //assert!(!match_response(expected, actual).is_empty(), "Missing favorite colour");
    }
}

#[test]
fn missing_key() {
    let pact = json!(
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
    );

    let expected = Response::from_json(&pact.find("expected").unwrap());
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap());
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       //assert!(match_response(expected, actual).is_empty(), "Missing key alligator name");
    } else {
       //assert!(!match_response(expected, actual).is_empty(), "Missing key alligator name");
    }
}

#[test]
fn not_null_found_at_key_when_null_expected() {
    let pact = json!(
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
    );

    let expected = Response::from_json(&pact.find("expected").unwrap());
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap());
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       //assert!(match_response(expected, actual).is_empty(), "Name should be null");
    } else {
       //assert!(!match_response(expected, actual).is_empty(), "Name should be null");
    }
}

#[test]
fn not_null_found_in_array_when_null_expected() {
    let pact = json!(
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
    );

    let expected = Response::from_json(&pact.find("expected").unwrap());
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap());
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       //assert!(match_response(expected, actual).is_empty(), "Favourite numbers expected to contain null, but not null found");
    } else {
       //assert!(!match_response(expected, actual).is_empty(), "Favourite numbers expected to contain null, but not null found");
    }
}

#[test]
fn null_found_at_key_where_not_null_expected() {
    let pact = json!(
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
    );

    let expected = Response::from_json(&pact.find("expected").unwrap());
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap());
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       //assert!(match_response(expected, actual).is_empty(), "Name should not be null");
    } else {
       //assert!(!match_response(expected, actual).is_empty(), "Name should not be null");
    }
}

#[test]
fn null_found_in_array_when_not_null_expected() {
    let pact = json!(
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
    );

    let expected = Response::from_json(&pact.find("expected").unwrap());
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap());
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       //assert!(match_response(expected, actual).is_empty(), "Favourite numbers expected to be strings found a null");
    } else {
       //assert!(!match_response(expected, actual).is_empty(), "Favourite numbers expected to be strings found a null");
    }
}

#[test]
fn number_found_at_key_when_string_expected() {
    let pact = json!(
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
    );

    let expected = Response::from_json(&pact.find("expected").unwrap());
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap());
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       //assert!(match_response(expected, actual).is_empty(), "Number of feet expected to be string but was number");
    } else {
       //assert!(!match_response(expected, actual).is_empty(), "Number of feet expected to be string but was number");
    }
}

#[test]
fn number_found_in_array_when_string_expected() {
    let pact = json!(
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
    );

    let expected = Response::from_json(&pact.find("expected").unwrap());
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap());
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       //assert!(match_response(expected, actual).is_empty(), "Favourite numbers expected to be strings found a number");
    } else {
       //assert!(!match_response(expected, actual).is_empty(), "Favourite numbers expected to be strings found a number");
    }
}

#[test]
fn objects_in_array_first_matches() {
    let pact = json!(
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
    );

    let expected = Response::from_json(&pact.find("expected").unwrap());
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap());
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       //assert!(match_response(expected, actual).is_empty(), "Properties match but unexpected element recieved");
    } else {
       //assert!(!match_response(expected, actual).is_empty(), "Properties match but unexpected element recieved");
    }
}

#[test]
fn objects_in_array_no_matches() {
    let pact = json!(
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
    );

    let expected = Response::from_json(&pact.find("expected").unwrap());
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap());
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       //assert!(match_response(expected, actual).is_empty(), "Array of objects, properties match on incorrect objects");
    } else {
       //assert!(!match_response(expected, actual).is_empty(), "Array of objects, properties match on incorrect objects");
    }
}

#[test]
fn objects_in_array_second_matches() {
    let pact = json!(
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
    );

    let expected = Response::from_json(&pact.find("expected").unwrap());
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap());
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       //assert!(match_response(expected, actual).is_empty(), "Property of second object matches, but unexpected element recieved");
    } else {
       //assert!(!match_response(expected, actual).is_empty(), "Property of second object matches, but unexpected element recieved");
    }
}

#[test]
fn plain_text_that_does_not_match() {
    let pact = json!(
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
    );

    let expected = Response::from_json(&pact.find("expected").unwrap());
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap());
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       //assert!(match_response(expected, actual).is_empty(), "Plain text that does not match");
    } else {
       //assert!(!match_response(expected, actual).is_empty(), "Plain text that does not match");
    }
}

#[test]
fn plain_text_that_matches() {
    let pact = json!(
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
    );

    let expected = Response::from_json(&pact.find("expected").unwrap());
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap());
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       //assert!(match_response(expected, actual).is_empty(), "Plain text that matches");
    } else {
       //assert!(!match_response(expected, actual).is_empty(), "Plain text that matches");
    }
}

#[test]
fn property_name_is_different_case() {
    let pact = json!(
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
    );

    let expected = Response::from_json(&pact.find("expected").unwrap());
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap());
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       //assert!(match_response(expected, actual).is_empty(), "Property names on objects are case sensitive");
    } else {
       //assert!(!match_response(expected, actual).is_empty(), "Property names on objects are case sensitive");
    }
}

#[test]
fn string_found_at_key_when_number_expected() {
    let pact = json!(
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
    );

    let expected = Response::from_json(&pact.find("expected").unwrap());
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap());
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       //assert!(match_response(expected, actual).is_empty(), "Number of feet expected to be number but was string");
    } else {
       //assert!(!match_response(expected, actual).is_empty(), "Number of feet expected to be number but was string");
    }
}

#[test]
fn string_found_in_array_when_number_expected() {
    let pact = json!(
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
    );

    let expected = Response::from_json(&pact.find("expected").unwrap());
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap());
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       //assert!(match_response(expected, actual).is_empty(), "Favourite Numbers expected to be numbers, but 2 is a string");
    } else {
       //assert!(!match_response(expected, actual).is_empty(), "Favourite Numbers expected to be numbers, but 2 is a string");
    }
}

#[test]
fn unexpected_index_with_not_null_value() {
    let pact = json!(
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
    );

    let expected = Response::from_json(&pact.find("expected").unwrap());
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap());
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       //assert!(match_response(expected, actual).is_empty(), "Unexpected favourite colour");
    } else {
       //assert!(!match_response(expected, actual).is_empty(), "Unexpected favourite colour");
    }
}

#[test]
fn unexpected_index_with_null_value() {
    let pact = json!(
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
    );

    let expected = Response::from_json(&pact.find("expected").unwrap());
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap());
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       //assert!(match_response(expected, actual).is_empty(), "Unexpected favourite colour with null value");
    } else {
       //assert!(!match_response(expected, actual).is_empty(), "Unexpected favourite colour with null value");
    }
}

#[test]
fn unexpected_key_with_not_null_value() {
    let pact = json!(
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
    );

    let expected = Response::from_json(&pact.find("expected").unwrap());
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap());
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       //assert!(match_response(expected, actual).is_empty(), "Unexpected phone number");
    } else {
       //assert!(!match_response(expected, actual).is_empty(), "Unexpected phone number");
    }
}

#[test]
fn unexpected_key_with_null_value() {
    let pact = json!(
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
    );

    let expected = Response::from_json(&pact.find("expected").unwrap());
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap());
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       //assert!(match_response(expected, actual).is_empty(), "Unexpected phone number with null value");
    } else {
       //assert!(!match_response(expected, actual).is_empty(), "Unexpected phone number with null value");
    }
}
