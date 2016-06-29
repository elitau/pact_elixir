#[allow(unused_imports)]
use pact_matching::models::*;
#[allow(unused_imports)]
use pact_matching::match_response;
#[allow(unused_imports)]
use rustc_serialize::json::Json;
#[allow(unused_imports)]
use expectest::prelude::*;

#[test]
#[ignore]
fn objects_in_array_type_matching() {
    let pact = Json::from_str(r#"
      {
        "match": true,
        "comment": "objects in array type matching",
        "expected": {
          "headers": {},
          "body": [{
            "name": "John Smith",
            "age": 50
          }],
          "matchingRules": {
            "$.body": {
              "match": "type"
            },
            "$.body[*]": {
              "match": "type"
            },
            "$.body[*].*": {
              "match": "type"
            }
          }
        },
        "actual": {
          "headers": {},
          "body": [{
            "name": "Peter Peterson",
            "age": 22,
            "gender": "Male"
          }, {
            "name": "John Johnston",
            "age": 64
          }]
        }
      }
    "#).unwrap();

    let expected = Response::from_json(&pact.find("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       expect!(match_response(expected, actual)).to(be_empty());
    } else {
       expect!(match_response(expected, actual)).to_not(be_empty());
    }
}

#[test]
#[ignore]
fn objects_in_array_with_type_mismatching() {
    let pact = Json::from_str(r#"
      {
        "match": false,
        "comment": "objects in array with type mismatching",
        "expected": {
          "headers": {},
          "body": [{
            "Name": "John Smith",
            "Age": 50
          }],
          "matchingRules": {
            "$.body[*]": {
              "match": "type"
            },
            "$.body[*].*": {
              "match": "type"
            }
          }
        },
        "actual": {
          "headers": {},
          "body": [{
            "name": "Peter Peterson",
            "age": 22,
            "gender": "Male"
          }, {}]
        }
      }
    "#).unwrap();

    let expected = Response::from_json(&pact.find("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       expect!(match_response(expected, actual)).to(be_empty());
    } else {
       expect!(match_response(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn property_name_is_different_case() {
    let pact = Json::from_str(r#"
      {
        "match": false,
        "comment": "Property names on objects are case sensitive",
        "expected" : {
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator":{
              "FavouriteColour": "red"
            }
          }
        },
        "actual": {
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator":{
              "favouritecolour": "red"
            }
          }
        }
      }
    "#).unwrap();

    let expected = Response::from_json(&pact.find("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       expect!(match_response(expected, actual)).to(be_empty());
    } else {
       expect!(match_response(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn string_found_at_key_when_number_expected() {
    let pact = Json::from_str(r#"
      {
        "match": false,
        "comment": "Number of feet expected to be number but was string",
        "expected" : {
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator":{
              "feet": 4
            }
          }
        },
        "actual": {
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator":{
              "feet": "4"
            }
          }
        }
      }
    "#).unwrap();

    let expected = Response::from_json(&pact.find("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       expect!(match_response(expected, actual)).to(be_empty());
    } else {
       expect!(match_response(expected, actual)).to_not(be_empty());
    }
}

#[test]
#[ignore]
fn array_with_type_matcher() {
    let pact = Json::from_str(r#"
      {
        "match": true,
        "comment": "array with type matcher",
        "expected": {
          "headers": {},
          "body" : {
            "myDates": [
              10
            ]
          },
          "matchingRules" : {
            "$.body.myDates" : {
              "match": "type"
            },
            "$.body.myDates[*]" : {
              "match": "type"
            }
          }
        },
        "actual": {
          "headers": {},
          "body": {
            "myDates": [
              20,
              5,
              1910
            ]
          }
        }
      }
    "#).unwrap();

    let expected = Response::from_json(&pact.find("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       expect!(match_response(expected, actual)).to(be_empty());
    } else {
       expect!(match_response(expected, actual)).to_not(be_empty());
    }
}

#[test]
#[ignore]
fn matches_with_regex() {
    let pact = Json::from_str(r#"
      {
        "match": true,
        "comment": "Requests match with regex",
        "expected" : {
          "headers": {"Content-Type": "application/json"},
          "matchingRules": {
            "$.body.alligator.name": {"match": "regex", "regex": "\\w+"}
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
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator":{
              "feet": 4,
              "name": "Harry",
              "favouriteColours": ["red","blue"]
            }
          }
        }
      }
    "#).unwrap();

    let expected = Response::from_json(&pact.find("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       expect!(match_response(expected, actual)).to(be_empty());
    } else {
       expect!(match_response(expected, actual)).to_not(be_empty());
    }
}

#[test]
#[ignore]
fn matches_with_type() {
    let pact = Json::from_str(r#"
      {
        "match": true,
        "comment": "Response match with same type",
        "expected" : {
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

    let expected = Response::from_json(&pact.find("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       expect!(match_response(expected, actual)).to(be_empty());
    } else {
       expect!(match_response(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn matches() {
    let pact = Json::from_str(r#"
      {
        "match": true,
        "comment": "Responses match",
        "expected" : {
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

    let expected = Response::from_json(&pact.find("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       expect!(match_response(expected, actual)).to(be_empty());
    } else {
       expect!(match_response(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn missing_key() {
    let pact = Json::from_str(r#"
      {
        "match": false,
        "comment": "Missing key alligator name",
        "expected" : {
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator":{
              "name": "Mary",
              "age": 3
            }
          }
        },
        "actual": {
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator": {
              "age": 3
            }
          }
        }
      }
    "#).unwrap();

    let expected = Response::from_json(&pact.find("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       expect!(match_response(expected, actual)).to(be_empty());
    } else {
       expect!(match_response(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn no_body_no_content_type() {
    let pact = Json::from_str(r#"
      {
        "match": true,
        "comment": "No body, no content-type",
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

    let expected = Response::from_json(&pact.find("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       expect!(match_response(expected, actual)).to(be_empty());
    } else {
       expect!(match_response(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn not_null_found_in_array_when_null_expected() {
    let pact = Json::from_str(r#"
      {
        "match": false,
        "comment": "Favourite numbers expected to contain null, but not null found",
        "expected" : {
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator":{
              "favouriteNumbers": ["1",null,"3"]
            }
          }
        },
        "actual": {
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator":{
              "favouriteNumbers": ["1","2","3"]
            }
          }
        }
      }
    "#).unwrap();

    let expected = Response::from_json(&pact.find("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       expect!(match_response(expected, actual)).to(be_empty());
    } else {
       expect!(match_response(expected, actual)).to_not(be_empty());
    }
}

#[test]
#[ignore]
fn array_with_type_matcher_mismatch() {
    let pact = Json::from_str(r#"
      {
        "match": false,
        "comment": "array with type matcher mismatch",
        "expected": {
          "headers": {},
          "body" : {
            "myDates": [
              10
            ]
          },
          "matchingRules" : {
            "$.body.myDates[*]" : {
              "match": "type"
            }
          }
        },
        "actual": {
          "headers": {},
          "body": {
            "myDates": [
              20,
              5,
              "100299"
            ]
          }
        }
      }
    "#).unwrap();

    let expected = Response::from_json(&pact.find("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       expect!(match_response(expected, actual)).to(be_empty());
    } else {
       expect!(match_response(expected, actual)).to_not(be_empty());
    }
}

#[test]
#[ignore]
fn additional_property_with_type_matcher() {
    let pact = Json::from_str(r#"
      {
        "match": true,
        "comment": "additional property with type matcher wildcards",
        "expected": {
          "headers": {},
          "body" : {
            "myPerson": {
              "name": "Any name",
              "age": 10
            }
          },
          "matchingRules" : {
            "$.body.myPerson.*" : {
              "match": "type"
            }
          }
        },
        "actual": {
          "headers": {},
          "body": {
            "myPerson": {
              "name": "Jon Peterson",
              "age": 39,
              "nationality": "Australian"
            }
          }
        }
      }
    "#).unwrap();

    let expected = Response::from_json(&pact.find("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       expect!(match_response(expected, actual)).to(be_empty());
    } else {
       expect!(match_response(expected, actual)).to_not(be_empty());
    }
}

#[test]
#[ignore]
fn array_at_top_level_with_matchers() {
    let pact = Json::from_str(r#"
      {
        "match": true,
        "comment": "top level array matches",
        "expected": {
          "headers": {"Content-Type": "application/json"},
          "body" : [ {
            "dob" : "06/11/2015",
            "name" : "Rogger the Dogger",
            "id" : 3380634027,
            "timestamp" : "2015-06-11T13:17:29"
          }, {
            "dob" : "06/11/2015",
            "name" : "Cat in the Hat",
            "id" : 1284270029,
            "timestamp" : "2015-06-11T13:17:29"
          } ],
          "matchingRules" : {
            "$.body[0].id" : {
              "match" : "type"
            },
            "$.body[1].id" : {
              "match" : "type"
            },
            "$.body[0].name" : {
              "match" : "type"
            },
            "$.body[1].name" : {
              "match" : "type"
            },
            "$.body[1].dob" : {
              "match": "regex", "regex" : "\\d{2}/\\d{2}/\\d{4}"
            },
            "$.body[1].timestamp" : {
              "match": "regex", "regex" : "\\d{4}-\\d{2}-\\d{2}T\\d{2}:\\d{2}:\\d{2}"
            },
            "$.body[0].timestamp" : {
              "match": "regex", "regex" : "\\d{4}-\\d{2}-\\d{2}T\\d{2}:\\d{2}:\\d{2}"
            },
            "$.body[0].dob" : {
              "match": "regex", "regex" : "\\d{2}/\\d{2}/\\d{4}"
            }
          }
        },
        "actual": {
          "headers": {"Content-Type": "application/json"},
          "body": [
            {
              "dob": "11/06/2015",
              "name": "Bob The Builder",
              "id": 1234567890,
              "timestamp": "2000-06-10T20:41:37"
            },
            {
              "dob": "12/10/2000",
              "name": "Slinky Malinky",
              "id": 6677889900,
              "timestamp": "2015-06-10T22:98:78"
            }
          ]
        }
      }
    "#).unwrap();

    let expected = Response::from_json(&pact.find("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       expect!(match_response(expected, actual)).to(be_empty());
    } else {
       expect!(match_response(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn null_found_at_key_where_not_null_expected() {
    let pact = Json::from_str(r#"
      {
        "match": false,
        "comment": "Name should not be null",
        "expected" : {
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator":{
              "name": "Mary"
            }
          }
        },
        "actual": {
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator":{
              "name": null
            }
          }
        }
      }
    "#).unwrap();

    let expected = Response::from_json(&pact.find("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       expect!(match_response(expected, actual)).to(be_empty());
    } else {
       expect!(match_response(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn different_value_found_at_key() {
    let pact = Json::from_str(r#"
      {
        "match": false,
        "comment": "Incorrect value at alligator name",
        "expected" : {
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator":{
              "name": "Mary"
            }
          }
        },
        "actual": {
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator":{
              "name": "Fred"
            }
          }
        }
      }
    "#).unwrap();

    let expected = Response::from_json(&pact.find("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       expect!(match_response(expected, actual)).to(be_empty());
    } else {
       expect!(match_response(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn keys_out_of_order_match() {
    let pact = Json::from_str(r#"
      {
        "match": true,
        "comment": "Favourite number and favourite colours out of order",
        "expected" : {
          "headers": {"Content-Type": "application/json"},
          "body": {
      		"favouriteNumber": 7,
              "favouriteColours": ["red","blue"]
          }
        },
        "actual": {
          "headers": {"Content-Type": "application/json"},
          "body": {
              "favouriteColours": ["red","blue"],
      		"favouriteNumber": 7
          }
        }
      }
    "#).unwrap();

    let expected = Response::from_json(&pact.find("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       expect!(match_response(expected, actual)).to(be_empty());
    } else {
       expect!(match_response(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn array_in_different_order() {
    let pact = Json::from_str(r#"
      {
        "match": false,
        "comment": "Favourite colours in wrong order",
        "expected" : {
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator":{
              "favouriteColours": ["red","blue"]
            }
          }
        },
        "actual": {
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator":{
              "favouriteColours": ["blue", "red"]
            }
          }
        }
      }
    "#).unwrap();

    let expected = Response::from_json(&pact.find("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       expect!(match_response(expected, actual)).to(be_empty());
    } else {
       expect!(match_response(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn missing_index() {
    let pact = Json::from_str(r#"
      {
        "match": false,
        "comment": "Missing favorite colour",
        "expected" : {
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator":{
              "favouriteColours": ["red","blue"]
            }
          }
        },
        "actual": {
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator": {
              "favouriteColours": ["red"]
            }
          }
        }
      }
    "#).unwrap();

    let expected = Response::from_json(&pact.find("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       expect!(match_response(expected, actual)).to(be_empty());
    } else {
       expect!(match_response(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn deeply_nested_objects() {
    let pact = Json::from_str(r#"
      {
      	"match": true,
      	"comment": "Comparisons should work even on nested objects",
      	"expected" : {
      		"headers": {"Content-Type": "application/json"},
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
      		"headers": {"Content-Type": "application/json"},
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

    let expected = Response::from_json(&pact.find("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       expect!(match_response(expected, actual)).to(be_empty());
    } else {
       expect!(match_response(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn different_value_found_at_index() {
    let pact = Json::from_str(r#"
      {
        "match": false,
        "comment": "Incorrect favourite colour",
        "expected" : {
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator":{
              "favouriteColours": ["red","blue"]
            }
          }
        },
        "actual": {
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator":{
              "favouriteColours": ["red","taupe"]
            }
          }
        }
      }
    "#).unwrap();

    let expected = Response::from_json(&pact.find("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       expect!(match_response(expected, actual)).to(be_empty());
    } else {
       expect!(match_response(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn not_null_found_at_key_when_null_expected() {
    let pact = Json::from_str(r#"
      {
        "match": false,
        "comment": "Name should be null",
        "expected" : {
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator":{
              "name": null
            }
          }
        },
        "actual": {
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator":{
              "name": "Fred"
            }
          }
        }
      }
    "#).unwrap();

    let expected = Response::from_json(&pact.find("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       expect!(match_response(expected, actual)).to(be_empty());
    } else {
       expect!(match_response(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn null_found_in_array_when_not_null_expected() {
    let pact = Json::from_str(r#"
      {
        "match": false,
        "comment": "Favourite numbers expected to be strings found a null",
        "expected" : {
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator":{
              "favouriteNumbers": ["1","2","3"]
            }
          }
        },
        "actual": {
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator":{
              "favouriteNumbers": ["1",null,"3"]
            }
          }
        }
      }
    "#).unwrap();

    let expected = Response::from_json(&pact.find("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       expect!(match_response(expected, actual)).to(be_empty());
    } else {
       expect!(match_response(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn number_found_at_key_when_string_expected() {
    let pact = Json::from_str(r#"
      {
        "match": false,
        "comment": "Number of feet expected to be string but was number",
        "expected" : {
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator":{
              "feet": "4"
            }
          }
        },
        "actual": {
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator":{
              "feet": 4
            }
          }
        }
      }
    "#).unwrap();

    let expected = Response::from_json(&pact.find("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       expect!(match_response(expected, actual)).to(be_empty());
    } else {
       expect!(match_response(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn number_found_in_array_when_string_expected() {
    let pact = Json::from_str(r#"
      {
        "match": false,
        "comment": "Favourite numbers expected to be strings found a number",
        "expected" : {
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator":{
              "favouriteNumbers": ["1","2","3"]
            }
          }
        },
        "actual": {
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator":{
              "favouriteNumbers": ["1",2,"3"]
            }
          }
        }
      }
    "#).unwrap();

    let expected = Response::from_json(&pact.find("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       expect!(match_response(expected, actual)).to(be_empty());
    } else {
       expect!(match_response(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn objects_in_array_first_matches() {
    let pact = Json::from_str(r#"
      {
        "match": false,
        "comment": "Properties match but unexpected element received",
        "expected": {
          "headers": {"Content-Type": "application/json"},
          "body": [
            {
              "favouriteColor": "red"
            }
          ]
        },
        "actual": {
          "headers": {"Content-Type": "application/json"},
          "body": [
            {
              "favouriteColor": "red",
              "favouriteNumber": 2
            },
            {
              "favouriteColor": "blue",
              "favouriteNumber": 2
            }
          ]
        }
      }
    "#).unwrap();

    let expected = Response::from_json(&pact.find("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       expect!(match_response(expected, actual)).to(be_empty());
    } else {
       expect!(match_response(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn objects_in_array_no_matches() {
    let pact = Json::from_str(r#"
      {
        "match": false,
        "comment": "Array of objects, properties match on incorrect objects",
        "expected" : {
          "headers": {"Content-Type": "application/json"},
          "body": [
      		{"favouriteColor": "red"},
      		{"favouriteNumber": 2}
      	]
        },
        "actual": {
          "headers": {"Content-Type": "application/json"},
          "body": [
      		{"favouriteColor": "blue",
      		"favouriteNumber": 4},
      		{"favouriteColor": "red",
      		"favouriteNumber": 2}
      	]
        }
      }
    "#).unwrap();

    let expected = Response::from_json(&pact.find("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       expect!(match_response(expected, actual)).to(be_empty());
    } else {
       expect!(match_response(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn objects_in_array_second_matches() {
    let pact = Json::from_str(r#"
      {
        "match": false,
        "comment": "Property of second object matches, but unexpected element recieved",
        "expected" : {
          "headers": {"Content-Type": "application/json"},
          "body": [
      		{"favouriteColor": "red"}
      	]
        },
        "actual": {
          "headers": {"Content-Type": "application/json"},
          "body": [
      		{"favouriteColor": "blue",
      		"favouriteNumber": 4},
      		{"favouriteColor": "red",
      		"favouriteNumber": 2}
      	]
        }
      }
    "#).unwrap();

    let expected = Response::from_json(&pact.find("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       expect!(match_response(expected, actual)).to(be_empty());
    } else {
       expect!(match_response(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn array_at_top_level() {
    let pact = Json::from_str(r#"
      {
        "match": true,
        "comment": "top level array matches",
        "expected": {
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

    let expected = Response::from_json(&pact.find("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       expect!(match_response(expected, actual)).to(be_empty());
    } else {
       expect!(match_response(expected, actual)).to_not(be_empty());
    }
}

#[test]
#[ignore]
fn array_with_regex_matcher() {
    let pact = Json::from_str(r#"
      {
        "match": true,
        "comment": "array with regex matcher",
        "expected": {
          "headers": {},
          "body" : {
            "myDates": [
              "29/10/2015"
            ]
          },
          "matchingRules" : {
            "$.body.myDates": {
              "match": "type"
            },
            "$.body.myDates[*]" : {
              "match": "regex", "regex" : "\\d{2}/\\d{2}/\\d{4}"
            }
          }
        },
        "actual": {
          "headers": {},
          "body": {
            "myDates": [
              "01/11/2010",
              "15/12/2014",
              "30/06/2015"
            ]
          }
        }
      }
    "#).unwrap();

    let expected = Response::from_json(&pact.find("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       expect!(match_response(expected, actual)).to(be_empty());
    } else {
       expect!(match_response(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn unexpected_index_with_not_null_value() {
    let pact = Json::from_str(r#"
      {
        "match": false,
        "comment": "Unexpected favourite colour",
        "expected" : {
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator":{
              "favouriteColours": ["red","blue"]
            }
          }
        },
        "actual": {
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator":{
              "favouriteColours": ["red","blue","taupe"]
            }
          }
        }
      }
    "#).unwrap();

    let expected = Response::from_json(&pact.find("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       expect!(match_response(expected, actual)).to(be_empty());
    } else {
       expect!(match_response(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn unexpected_index_with_null_value() {
    let pact = Json::from_str(r#"
      {
        "match": false,
        "comment": "Unexpected favourite colour with null value",
        "expected" : {
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator":{
              "favouriteColours": ["red","blue"]
            }
          }
        },
        "actual": {
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator":{
              "favouriteColours": ["red","blue", null]
            }
          }
        }
      }
    "#).unwrap();

    let expected = Response::from_json(&pact.find("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       expect!(match_response(expected, actual)).to(be_empty());
    } else {
       expect!(match_response(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn unexpected_key_with_not_null_value() {
    let pact = Json::from_str(r#"
      {
        "match": true,
        "comment": "Unexpected phone number",
        "expected" : {
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator":{
              "name": "Mary"
            }
          }
        },
        "actual": {
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

    let expected = Response::from_json(&pact.find("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       expect!(match_response(expected, actual)).to(be_empty());
    } else {
       expect!(match_response(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn unexpected_key_with_null_value() {
    let pact = Json::from_str(r#"
      {
        "match": true,
        "comment": "Unexpected phone number with null value",
        "expected" : {
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator":{
              "name": "Mary"
            }
          }
        },
        "actual": {
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

    let expected = Response::from_json(&pact.find("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       expect!(match_response(expected, actual)).to(be_empty());
    } else {
       expect!(match_response(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn string_found_in_array_when_number_expected() {
    let pact = Json::from_str(r#"
      {
        "match": false,
        "comment": "Favourite Numbers expected to be numbers, but 2 is a string",
        "expected" : {
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator":{
              "favouriteNumbers": [1,2,3]
            }
          }
        },
        "actual": {
          "headers": {"Content-Type": "application/json"},
          "body": {
            "alligator":{
              "favouriteNumbers": [1,"2",3]
            }
          }
        }
      }
    "#).unwrap();

    let expected = Response::from_json(&pact.find("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       expect!(match_response(expected, actual)).to(be_empty());
    } else {
       expect!(match_response(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn missing_body() {
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

    let expected = Response::from_json(&pact.find("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       expect!(match_response(expected, actual)).to(be_empty());
    } else {
       expect!(match_response(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn plain_text_that_does_not_match() {
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

    let expected = Response::from_json(&pact.find("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       expect!(match_response(expected, actual)).to(be_empty());
    } else {
       expect!(match_response(expected, actual)).to_not(be_empty());
    }
}

#[test]
fn plain_text_that_matches() {
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

    let expected = Response::from_json(&pact.find("expected").unwrap(), &PactSpecification::V2);
    println!("{:?}", expected);
    let actual = Response::from_json(&pact.find("actual").unwrap(), &PactSpecification::V2);
    println!("{:?}", actual);
    let pact_match = pact.find("match").unwrap();
    if pact_match.as_boolean().unwrap() {
       expect!(match_response(expected, actual)).to(be_empty());
    } else {
       expect!(match_response(expected, actual)).to_not(be_empty());
    }
}
