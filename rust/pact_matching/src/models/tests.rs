use super::*;
use super::{matchers_from_json, body_from_json, headers_from_json};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io;
use std::io::prelude::*;
use std::env;
use expectest::prelude::*;
use rand;
use std::hash::{Hash, Hasher, SipHasher};

#[test]
fn request_from_json_defaults_to_get() {
    let request_json : serde_json::Value = serde_json::from_str(r#"
      {
          "path": "/",
          "query": "",
          "headers": {}
      }
     "#).unwrap();
    let request = Request::from_json(&request_json, &PactSpecification::V1_1);
    assert_eq!(request.method, "GET".to_string());
}

#[test]
fn request_from_json_defaults_to_root_for_path() {
    let request_json : serde_json::Value = serde_json::from_str(r#"
      {
          "method": "PUT",
          "query": "",
          "headers": {}
      }
     "#).unwrap();
    println!("request_json: {}", request_json);
    let request = Request::from_json(&request_json, &PactSpecification::V1_1);
    assert_eq!(request.path, "/".to_string());
}

#[test]
fn response_from_json_defaults_to_status_200() {
    let response_json : serde_json::Value = serde_json::from_str(r#"
      {
          "headers": {}
      }
     "#).unwrap();
    let response = Response::from_json(&response_json, &PactSpecification::V1_1);
    assert_eq!(response.status, 200);
}

#[test]
fn parse_query_string_test() {
    let query = "a=b&c=d".to_string();
    let mut expected = HashMap::new();
    expected.insert("a".to_string(), vec!["b".to_string()]);
    expected.insert("c".to_string(), vec!["d".to_string()]);
    let result = parse_query_string(&query);
    assert_eq!(result, Some(expected));
}

#[test]
fn parse_query_string_handles_empty_string() {
    let query = "".to_string();
    let expected = None;
    let result = parse_query_string(&query);
    assert_eq!(result, expected);
}

#[test]
fn parse_query_string_handles_missing_values() {
    let query = "a=&c=d".to_string();
    let mut expected = HashMap::new();
    expected.insert("a".to_string(), vec!["".to_string()]);
    expected.insert("c".to_string(), vec!["d".to_string()]);
    let result = parse_query_string(&query);
    assert_eq!(result, Some(expected));
}

#[test]
fn parse_query_string_handles_equals_in_values() {
    let query = "a=b&c=d=e=f".to_string();
    let mut expected = HashMap::new();
    expected.insert("a".to_string(), vec!["b".to_string()]);
    expected.insert("c".to_string(), vec!["d=e=f".to_string()]);
    let result = parse_query_string(&query);
    assert_eq!(result, Some(expected));
}

#[test]
fn parse_query_string_decodes_values() {
    let query = "a=a%20b%20c".to_string();
    let mut expected = HashMap::new();
    expected.insert("a".to_string(), vec!["a b c".to_string()]);
    let result = parse_query_string(&query);
    assert_eq!(result, Some(expected));
}

#[test]
#[ignore]
fn quickcheck_parse_query_string() {
    use quickcheck::{TestResult, quickcheck};
    use super::decode_query;
    use itertools::Itertools;
    fn prop(s: String) -> TestResult {
        if s.chars().all(|c| c.is_alphanumeric() || c == '+' || c == '&' || c == '%') {
            let result = match parse_query_string(&s) {
                Some(map) => {
                    if map.len() == 1 && !s.contains("=") {
                        *map.keys().next().unwrap() == decode_query(&s)
                    } else {
                        let reconstructed_query = map.iter().map(|(k, v)| {
                            v.iter().map(|qv| format!("{}={}", k, qv)).join("&")
                        }).join("&");
                        let r = decode_query(&s) == reconstructed_query;
                        if !r {
                            p!(reconstructed_query);
                            p!(decode_query(&s) == reconstructed_query);
                        }
                        r
                    }
                },
                None => s.is_empty()
            };

            if !result {
                p!(s);
                p!(decode_query(&s));
            }
            TestResult::from_bool(result)
        } else {
            TestResult::discard()
        }
    }
    quickcheck(prop as fn(_) -> _);
}

#[test]
fn request_mimetype_is_based_on_the_content_type_header() {
    let request = Request { method: s!("GET"), path: s!("/"), query: None, headers: None,
        body: OptionalBody::Missing, matching_rules: None };
    expect!(request.content_type()).to(be_equal_to("text/plain"));
    expect!(Request {
        headers: Some(hashmap!{ s!("Content-Type") => s!("text/html") }), .. request.clone() }.content_type())
        .to(be_equal_to("text/html"));
    expect!(Request {
        headers: Some(hashmap!{ s!("Content-Type") => s!("application/json; charset=UTF-8") }), .. request.clone() }.content_type())
        .to(be_equal_to("application/json"));
    expect!(Request {
        headers: Some(hashmap!{ s!("Content-Type") => s!("application/json") }), .. request.clone() }.content_type())
        .to(be_equal_to("application/json"));
    expect!(Request {
        headers: Some(hashmap!{ s!("CONTENT-TYPE") => s!("application/json ; charset=UTF-8") }), .. request.clone() }.content_type())
        .to(be_equal_to("application/json"));
    expect!(Request {
        body: OptionalBody::Present(s!("{\"json\": true}")), .. request.clone() }.content_type())
        .to(be_equal_to("application/json"));
    expect!(Request {
        body: OptionalBody::Present(s!("{}")), .. request.clone() }.content_type())
        .to(be_equal_to("application/json"));
    expect!(Request {
        body: OptionalBody::Present(s!("[]")), .. request.clone() }.content_type())
        .to(be_equal_to("application/json"));
    expect!(Request {
        body: OptionalBody::Present(s!("[1,2,3]")), .. request.clone() }.content_type())
        .to(be_equal_to("application/json"));
    expect!(Request {
        body: OptionalBody::Present(s!("\"string\"")), .. request.clone() }.content_type())
        .to(be_equal_to("application/json"));
    expect!(Request {
        body: OptionalBody::Present(s!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<json>false</json>")), .. request.clone() }.content_type())
        .to(be_equal_to("application/xml"));
    expect!(Request {
        body: OptionalBody::Present(s!("<json>false</json>")), .. request.clone() }.content_type())
        .to(be_equal_to("application/xml"));
    expect!(Request {
        body: OptionalBody::Present(s!("this is not json")), .. request.clone() }.content_type())
        .to(be_equal_to("text/plain"));
    expect!(Request {
        body: OptionalBody::Present(s!("<html><body>this is also not json</body></html>")), .. request.clone() }.content_type())
        .to(be_equal_to("text/html"));
}

#[test]
fn content_type_enum_test() {
    let request = Request { method: s!("GET"), path: s!("/"), query: None, headers: None,
        body: OptionalBody::Missing, matching_rules: None };
    expect!(request.content_type_enum()).to(be_equal_to(DetectedContentType::Text));
    expect!(Request {
        headers: Some(hashmap!{ s!("Content-Type") => s!("text/html") }), .. request.clone() }.content_type_enum())
        .to(be_equal_to(DetectedContentType::Text));
    expect!(Request {
        headers: Some(hashmap!{ s!("Content-Type") => s!("application/json") }), .. request.clone() }.content_type_enum())
        .to(be_equal_to(DetectedContentType::Json));
    expect!(Request {
        headers: Some(hashmap!{ s!("Content-Type") => s!("application/hal+json") }), .. request.clone() }.content_type_enum())
        .to(be_equal_to(DetectedContentType::Json));
    expect!(Request {
        headers: Some(hashmap!{ s!("CONTENT-TYPE") => s!("application/json-rpc") }), .. request.clone() }.content_type_enum())
        .to(be_equal_to(DetectedContentType::Json));
    expect!(Request {
        headers: Some(hashmap!{ s!("CONTENT-TYPE") => s!("application/xml") }), .. request.clone() }.content_type_enum())
        .to(be_equal_to(DetectedContentType::Xml));
    expect!(Request {
        headers: Some(hashmap!{ s!("CONTENT-TYPE") => s!("application/stuff+xml") }), .. request.clone() }.content_type_enum())
        .to(be_equal_to(DetectedContentType::Xml));
}

#[test]
fn loading_interaction_from_json() {
    let interaction_json = r#"{
        "description": "String",
        "providerState": "provider state"
    }"#;
    let interaction = Interaction::from_json(0, &serde_json::from_str({interaction_json}).unwrap(), &PactSpecification::V1_1);
    expect!(interaction.description).to(be_equal_to("String"));
    expect!(interaction.provider_state).to(be_some().value("provider state"));
}

#[test]
fn defaults_to_number_if_no_description() {
    let interaction_json = r#"{
        "providerState": "provider state"
    }"#;
    let interaction = Interaction::from_json(0, &serde_json::from_str({interaction_json}).unwrap(), &PactSpecification::V1_1);
    expect!(interaction.description).to(be_equal_to("Interaction 0"));
    expect!(interaction.provider_state).to(be_some().value("provider state"));
}

#[test]
fn defaults_to_none_if_no_provider_state() {
    let interaction_json = r#"{
    }"#;
    let interaction = Interaction::from_json(0, &serde_json::from_str({interaction_json}).unwrap(), &PactSpecification::V1_1);
    expect!(interaction.provider_state).to(be_none());
}

#[test]
fn defaults_to_none_if_provider_state_null() {
    let interaction_json = r#"{
        "providerState": null
    }"#;
    let interaction = Interaction::from_json(0, &serde_json::from_str({interaction_json}).unwrap(), &PactSpecification::V1_1);
    expect!(interaction.provider_state).to(be_none());
}

#[test]
fn load_empty_pact() {
    let pact_json = r#"{}"#;
    let pact = Pact::from_json(&s!(""), &serde_json::from_str(pact_json).unwrap());
    expect!(pact.provider.name).to(be_equal_to("provider"));
    expect!(pact.consumer.name).to(be_equal_to("consumer"));
    expect!(pact.interactions.iter()).to(have_count(0));
    expect!(pact.metadata.iter()).to(have_count(0));
    expect!(pact.specification_version).to(be_equal_to(PactSpecification::V2));
}

#[test]
fn missing_metadata() {
    let pact_json = r#"{}"#;
    let pact = Pact::from_json(&s!(""), &serde_json::from_str(pact_json).unwrap());
    expect!(pact.specification_version).to(be_equal_to(PactSpecification::V2));
}

#[test]
fn missing_spec_version() {
    let pact_json = r#"{
        "metadata" : {
        }
    }"#;
    let pact = Pact::from_json(&s!(""), &serde_json::from_str(pact_json).unwrap());
    expect!(pact.specification_version).to(be_equal_to(PactSpecification::V2));
}

#[test]
fn missing_version_in_spec_version() {
    let pact_json = r#"{
        "metadata" : {
            "pact-specification": {

            }
        }
    }"#;
    let pact = Pact::from_json(&s!(""), &serde_json::from_str(pact_json).unwrap());
    expect!(pact.specification_version).to(be_equal_to(PactSpecification::V2));
}

#[test]
fn empty_version_in_spec_version() {
    let pact_json = r#"{
        "metadata" : {
            "pact-specification": {
                "version": ""
            }
        }
    }"#;
    let pact = Pact::from_json(&s!(""), &serde_json::from_str(pact_json).unwrap());
    expect!(pact.specification_version).to(be_equal_to(PactSpecification::Unknown));
}

#[test]
fn correct_version_in_spec_version() {
    let pact_json = r#"{
        "metadata" : {
            "pact-specification": {
                "version": "1.0.0"
            }
        }
    }"#;
    let pact = Pact::from_json(&s!(""), &serde_json::from_str(pact_json).unwrap());
    expect!(pact.specification_version).to(be_equal_to(PactSpecification::V1));
}

#[test]
fn invalid_version_in_spec_version() {
    let pact_json = r#"{
        "metadata" : {
            "pact-specification": {
                "version": "znjclkazjs"
            }
        }
    }"#;
    let pact = Pact::from_json(&s!(""), &serde_json::from_str(pact_json).unwrap());
    expect!(pact.specification_version).to(be_equal_to(PactSpecification::Unknown));
}


#[test]
fn load_basic_pact() {
    let pact_json = r#"
    {
        "provider": {
            "name": "Alice Service"
        },
        "consumer": {
            "name": "Consumer"
        },
        "interactions": [
          {
              "description": "a retrieve Mallory request",
              "request": {
                "method": "GET",
                "path": "/mallory",
                "query": "name=ron&status=good"
              },
              "response": {
                "status": 200,
                "headers": {
                  "Content-Type": "text/html"
                },
                "body": "\"That is some good Mallory.\""
              }
          }
        ]
    }
    "#;
    let pact = Pact::from_json(&s!(""), &serde_json::from_str(pact_json).unwrap());
    expect!(&pact.provider.name).to(be_equal_to("Alice Service"));
    expect!(&pact.consumer.name).to(be_equal_to("Consumer"));
    expect!(pact.interactions.iter()).to(have_count(1));
    let interaction = pact.interactions[0].clone();
    expect!(interaction.description).to(be_equal_to("a retrieve Mallory request"));
    expect!(interaction.provider_state).to(be_none());
    expect!(interaction.request).to(be_equal_to(Request {
        method: s!("GET"),
        path: s!("/mallory"),
        query: Some(hashmap!{ s!("name") => vec![s!("ron")], s!("status") => vec![s!("good")] }),
        headers: None,
        body: OptionalBody::Missing,
        matching_rules: None
    }));
    expect!(interaction.response).to(be_equal_to(Response {
        status: 200,
        headers: Some(hashmap!{ s!("Content-Type") => s!("text/html") }),
        body: OptionalBody::Present(s!("\"That is some good Mallory.\"")),
        matching_rules: None
    }));
    expect!(pact.specification_version).to(be_equal_to(PactSpecification::V2));
    expect!(pact.metadata.iter()).to(have_count(0));
}

#[test]
fn load_pact() {
    let pact_json = r#"
    {
      "provider" : {
        "name" : "test_provider"
      },
      "consumer" : {
        "name" : "test_consumer"
      },
      "interactions" : [ {
        "providerState" : "test state",
        "description" : "test interaction",
        "request" : {
          "method" : "GET",
          "path" : "/",
          "headers" : {
            "testreqheader" : "testreqheadervalue"
          },
          "query" : "q=p&q=p2&r=s",
          "body" : {
            "test" : true
          }
        },
        "response" : {
          "status" : 200,
          "headers" : {
            "testreqheader" : "testreqheaderval"
          },
          "body" : {
            "responsetest" : true
          }
        }
      } ],
      "metadata" : {
        "pact-specification" : {
          "version" : "1.0.0"
        },
        "pact-jvm" : {
          "version" : ""
        }
      }
    }
    "#;
    let pact = Pact::from_json(&s!(""), &serde_json::from_str(pact_json).unwrap());
    expect!(&pact.provider.name).to(be_equal_to("test_provider"));
    expect!(&pact.consumer.name).to(be_equal_to("test_consumer"));
    expect!(pact.metadata.iter()).to(have_count(2));
    expect!(&pact.metadata["pact-specification"]["version"]).to(be_equal_to("1.0.0"));
    expect!(pact.specification_version).to(be_equal_to(PactSpecification::V1));
    expect!(pact.interactions.iter()).to(have_count(1));
    let interaction = pact.interactions[0].clone();
    expect!(interaction.description).to(be_equal_to("test interaction"));
    expect!(interaction.provider_state).to(be_some().value("test state"));
    expect!(interaction.request).to(be_equal_to(Request {
        method: s!("GET"),
        path: s!("/"),
        query: Some(hashmap!{ s!("q") => vec![s!("p"), s!("p2")], s!("r") => vec![s!("s")] }),
        headers: Some(hashmap!{ s!("testreqheader") => s!("testreqheadervalue") }),
        body: OptionalBody::Present(s!("{\"test\":true}")),
        matching_rules: None
    }));
    expect!(interaction.response).to(be_equal_to(Response {
        status: 200,
        headers: Some(hashmap!{ s!("testreqheader") => s!("testreqheaderval") }),
        body: OptionalBody::Present(s!("{\"responsetest\":true}")),
        matching_rules: None
    }));
}

#[test]
fn load_pact_encoded_query_string() {
    let pact_json = r#"
    {
      "provider" : {
        "name" : "test_provider"
      },
      "consumer" : {
        "name" : "test_consumer"
      },
      "interactions" : [ {
        "providerState" : "test state",
        "description" : "test interaction",
        "request" : {
          "method" : "GET",
          "path" : "/",
          "headers" : {
            "testreqheader" : "testreqheadervalue"
          },
          "query" : "datetime=2011-12-03T10%3A15%3A30%2B01%3A00&description=hello+world%21",
          "body" : {
            "test" : true
          }
        },
        "response" : {
          "status" : 200,
          "headers" : {
            "testreqheader" : "testreqheaderval"
          },
          "body" : {
            "responsetest" : true
          }
        }
      } ],
      "metadata" : {
        "pact-specification" : {
          "version" : "2.0.0"
        },
        "pact-jvm" : {
          "version" : ""
        }
      }
    }
    "#;
    let pact = Pact::from_json(&s!(""), &serde_json::from_str(pact_json).unwrap());
    expect!(pact.interactions.iter()).to(have_count(1));
    let interaction = pact.interactions[0].clone();
    expect!(interaction.request).to(be_equal_to(Request {
        method: s!("GET"),
        path: s!("/"),
        query: Some(hashmap!{ s!("datetime") => vec![s!("2011-12-03T10:15:30+01:00")],
            s!("description") => vec![s!("hello world!")] }),
        headers: Some(hashmap!{ s!("testreqheader") => s!("testreqheadervalue") }),
        body: OptionalBody::Present(s!("{\"test\":true}")),
        matching_rules: None
    }));
}

#[test]
fn load_pact_converts_methods_to_uppercase() {
    let pact_json = r#"
    {
      "interactions" : [ {
        "description" : "test interaction",
        "request" : {
          "method" : "get"
        },
        "response" : {
          "status" : 200
        }
      } ],
      "metadata" : {}
    }
    "#;
    let pact = Pact::from_json(&s!(""), &serde_json::from_str(pact_json).unwrap());
    expect!(pact.interactions.iter()).to(have_count(1));
    let interaction = pact.interactions[0].clone();
    expect!(interaction.request).to(be_equal_to(Request {
        method: s!("GET"),
        path: s!("/"),
        query: None,
        headers: None,
        body: OptionalBody::Missing,
        matching_rules: None
    }));
}

#[test]
fn request_to_json_with_defaults() {
    let request = Request::default_request();
    expect!(request.to_json().to_string()).to(be_equal_to("{\"method\":\"GET\",\"path\":\"/\"}"));
}

#[test]
fn request_to_json_converts_methods_to_upper_case() {
    let request = Request { method: s!("post"), .. Request::default_request() };
    expect!(request.to_json().to_string()).to(be_equal_to("{\"method\":\"POST\",\"path\":\"/\"}"));
}

#[test]
fn request_to_json_with_a_query() {
    let request = Request { query: Some(hashmap!{
        s!("a") => vec![s!("1"), s!("2")],
        s!("b") => vec![s!("3")]
    }), .. Request::default_request() };
    expect!(request.to_json().to_string()).to(
        be_equal_to(r#"{"method":"GET","path":"/","query":"a=1&a=2&b=3"}"#)
    );
}

#[test]
fn request_to_json_with_a_query_must_encode_the_query() {
    let request = Request { query: Some(hashmap!{
        s!("datetime") => vec![s!("2011-12-03T10:15:30+01:00")],
        s!("description") => vec![s!("hello world!")] }), .. Request::default_request() };
    expect!(request.to_json().to_string()).to(
        be_equal_to(r#"{"method":"GET","path":"/","query":"datetime=2011-12-03T10%3a15%3a30%2b01%3a00&description=hello+world%21"}"#)
    );
}

#[test]
fn request_to_json_with_a_query_must_encode_the_query_with_utf8_chars() {
    let request = Request { query: Some(hashmap!{
        s!("a") => vec![s!("b=c&d❤")]
    }), .. Request::default_request() };
    expect!(request.to_json().to_string()).to(
        be_equal_to(r#"{"method":"GET","path":"/","query":"a=b%3dc%26d%27%64"}"#)
    );
}

#[test]
fn request_to_json_with_headers() {
    let request = Request { headers: Some(hashmap!{
        s!("HEADERA") => s!("VALUEA"),
        s!("HEADERB") => s!("VALUEB1, VALUEB2")
    }), .. Request::default_request() };
    expect!(request.to_json().to_string()).to(
        be_equal_to(r#"{"headers":{"HEADERA":"VALUEA","HEADERB":"VALUEB1, VALUEB2"},"method":"GET","path":"/"}"#)
    );
}

#[test]
fn request_to_json_with_json_body() {
    let request = Request { headers: Some(hashmap!{
        s!("Content-Type") => s!("application/json")
    }), body: OptionalBody::Present(s!(r#"{"key": "value"}"#)), .. Request::default_request() };
    expect!(request.to_json().to_string()).to(
        be_equal_to(r#"{"body":{"key":"value"},"headers":{"Content-Type":"application/json"},"method":"GET","path":"/"}"#)
    );
}


#[test]
fn request_to_json_with_non_json_body() {
    let request = Request { headers: Some(hashmap!{ s!("Content-Type") => s!("text/plain") }),
        body: OptionalBody::Present(s!("This is some text")), .. Request::default_request() };
    expect!(request.to_json().to_string()).to(
        be_equal_to(r#"{"body":"This is some text","headers":{"Content-Type":"text/plain"},"method":"GET","path":"/"}"#)
    );
}

#[test]
fn request_to_json_with_empty_body() {
    let request = Request { body: OptionalBody::Empty, .. Request::default_request() };
    expect!(request.to_json().to_string()).to(
        be_equal_to(r#"{"body":"","method":"GET","path":"/"}"#)
    );
}

#[test]
fn request_to_json_with_null_body() {
    let request = Request { body: OptionalBody::Null, .. Request::default_request() };
    expect!(request.to_json().to_string()).to(
        be_equal_to(r#"{"body":null,"method":"GET","path":"/"}"#)
    );
}

#[test]
fn response_to_json_with_defaults() {
    let response = Response::default_response();
    expect!(response.to_json().to_string()).to(be_equal_to("{\"status\":200}"));
}

#[test]
fn response_to_json_with_headers() {
    let response = Response { headers: Some(hashmap!{
        s!("HEADERA") => s!("VALUEA"),
        s!("HEADERB") => s!("VALUEB1, VALUEB2")
    }), .. Response::default_response() };
    expect!(response.to_json().to_string()).to(
        be_equal_to(r#"{"headers":{"HEADERA":"VALUEA","HEADERB":"VALUEB1, VALUEB2"},"status":200}"#)
    );
}

#[test]
fn response_to_json_with_json_body() {
    let response = Response { headers: Some(hashmap!{
        s!("Content-Type") => s!("application/json")
    }), body: OptionalBody::Present(s!(r#"{"key": "value"}"#)), .. Response::default_response() };
    expect!(response.to_json().to_string()).to(
        be_equal_to(r#"{"body":{"key":"value"},"headers":{"Content-Type":"application/json"},"status":200}"#)
    );
}

#[test]
fn response_to_json_with_non_json_body() {
    let response = Response { headers: Some(hashmap!{ s!("Content-Type") => s!("text/plain") }),
        body: OptionalBody::Present(s!("This is some text")), .. Response::default_response() };
    expect!(response.to_json().to_string()).to(
        be_equal_to(r#"{"body":"This is some text","headers":{"Content-Type":"text/plain"},"status":200}"#)
    );
}

#[test]
fn response_to_json_with_empty_body() {
    let response = Response { body: OptionalBody::Empty, .. Response::default_response() };
    expect!(response.to_json().to_string()).to(
        be_equal_to(r#"{"body":"","status":200}"#)
    );
}

#[test]
fn response_to_json_with_null_body() {
    let response = Response { body: OptionalBody::Null, .. Response::default_response() };
    expect!(response.to_json().to_string()).to(
        be_equal_to(r#"{"body":null,"status":200}"#)
    );
}

#[test]
fn default_file_name_is_based_in_the_consumer_and_provider() {
    let pact = Pact { consumer: Consumer { name: s!("consumer") },
        provider: Provider { name: s!("provider") },
        interactions: vec![],
        metadata: btreemap!{},
        specification_version: PactSpecification::V1_1
    };
    expect!(pact.default_file_name()).to(be_equal_to("consumer-provider.json"));
}

fn read_pact_file(file: &str) -> io::Result<String> {
    let mut f = try!(File::open(file));
    let mut buffer = String::new();
    try!(f.read_to_string(&mut buffer));
    Ok(buffer)
}

#[test]
fn write_pact_test() {
    let pact = Pact { consumer: Consumer { name: s!("write_pact_test_consumer") },
        provider: Provider { name: s!("write_pact_test_provider") },
        interactions: vec![
            Interaction {
                description: s!("Test Interaction"),
                provider_state: Some(s!("Good state to be in")),
                request: Request::default_request(),
                response: Response::default_response()
            }
        ],
        .. Pact::default() };
    let mut dir = env::temp_dir();
    let x = rand::random::<u16>();
    dir.push(format!("pact_test_{}", x));
    dir.push(pact.default_file_name());

    let result = pact.write_pact(dir.as_path());

    let pact_file = read_pact_file(dir.as_path().to_str().unwrap()).unwrap_or(s!(""));
    fs::remove_dir_all(dir.parent().unwrap()).unwrap_or(());

    expect!(result).to(be_ok());
    expect(pact_file).to(be_equal_to(format!(r#"{{
  "consumer": {{
    "name": "write_pact_test_consumer"
  }},
  "interactions": [
    {{
      "description": "Test Interaction",
      "providerState": "Good state to be in",
      "request": {{
        "method": "GET",
        "path": "/"
      }},
      "response": {{
        "status": 200
      }}
    }}
  ],
  "metadata": {{
    "pact-rust": {{
      "version": "{}"
    }},
    "pact-specification": {{
      "version": "2.0.0"
    }}
  }},
  "provider": {{
    "name": "write_pact_test_provider"
  }}
}}"#, super::VERSION.unwrap())));
}

#[test]
fn write_pact_test_should_merge_pacts() {
    let pact = Pact { consumer: Consumer { name: s!("merge_consumer") },
        provider: Provider { name: s!("merge_provider") },
        interactions: vec![
            Interaction {
                description: s!("Test Interaction 2"),
                provider_state: Some(s!("Good state to be in")),
                request: Request::default_request(),
                response: Response::default_response()
            }
        ],
        metadata: btreemap!{},
        specification_version: PactSpecification::V1_1
    };
    let pact2 = Pact { consumer: Consumer { name: s!("merge_consumer") },
        provider: Provider { name: s!("merge_provider") },
        interactions: vec![
            Interaction {
                description: s!("Test Interaction"),
                provider_state: Some(s!("Good state to be in")),
                request: Request::default_request(),
                response: Response::default_response()
            }
        ],
        metadata: btreemap!{},
        specification_version: PactSpecification::V1_1
    };
    let mut dir = env::temp_dir();
    let x = rand::random::<u16>();
    dir.push(format!("pact_test_{}", x));
    dir.push(pact.default_file_name());

    let result = pact.write_pact(dir.as_path());
    let result2 = pact2.write_pact(dir.as_path());

    let pact_file = read_pact_file(dir.as_path().to_str().unwrap()).unwrap_or(s!(""));
    fs::remove_dir_all(dir.parent().unwrap()).unwrap_or(());

    expect!(result).to(be_ok());
    expect!(result2).to(be_ok());
    expect(pact_file).to(be_equal_to(format!(r#"{{
  "consumer": {{
    "name": "merge_consumer"
  }},
  "interactions": [
    {{
      "description": "Test Interaction",
      "providerState": "Good state to be in",
      "request": {{
        "method": "GET",
        "path": "/"
      }},
      "response": {{
        "status": 200
      }}
    }},
    {{
      "description": "Test Interaction 2",
      "providerState": "Good state to be in",
      "request": {{
        "method": "GET",
        "path": "/"
      }},
      "response": {{
        "status": 200
      }}
    }}
  ],
  "metadata": {{
    "pact-rust": {{
      "version": "{}"
    }},
    "pact-specification": {{
      "version": "2.0.0"
    }}
  }},
  "provider": {{
    "name": "merge_provider"
  }}
}}"#, super::VERSION.unwrap())));
}

#[test]
fn write_pact_test_should_not_merge_pacts_with_conflicts() {
    let pact = Pact { consumer: Consumer { name: s!("write_pact_test_consumer") },
        provider: Provider { name: s!("write_pact_test_provider") },
        interactions: vec![
            Interaction {
                description: s!("Test Interaction"),
                provider_state: Some(s!("Good state to be in")),
                request: Request::default_request(),
                response: Response::default_response()
            }
        ],
        metadata: btreemap!{},
        specification_version: PactSpecification::V1_1
    };
    let pact2 = Pact { consumer: Consumer { name: s!("write_pact_test_consumer") },
        provider: Provider { name: s!("write_pact_test_provider") },
        interactions: vec![
            Interaction {
                description: s!("Test Interaction"),
                provider_state: Some(s!("Good state to be in")),
                request: Request::default_request(),
                response: Response { status: 400, .. Response::default_response() }
            }
        ],
        metadata: btreemap!{},
        specification_version: PactSpecification::V1_1
    };
    let mut dir = env::temp_dir();
    let x = rand::random::<u16>();
    dir.push(format!("pact_test_{}", x));
    dir.push(pact.default_file_name());

    let result = pact.write_pact(dir.as_path());
    let result2 = pact2.write_pact(dir.as_path());

    let pact_file = read_pact_file(dir.as_path().to_str().unwrap()).unwrap_or(s!(""));
    fs::remove_dir_all(dir.parent().unwrap()).unwrap_or(());

    expect!(result).to(be_ok());
    expect!(result2).to(be_err());
    expect(pact_file).to(be_equal_to(format!(r#"{{
  "consumer": {{
    "name": "write_pact_test_consumer"
  }},
  "interactions": [
    {{
      "description": "Test Interaction",
      "providerState": "Good state to be in",
      "request": {{
        "method": "GET",
        "path": "/"
      }},
      "response": {{
        "status": 200
      }}
    }}
  ],
  "metadata": {{
    "pact-rust": {{
      "version": "{}"
    }},
    "pact-specification": {{
      "version": "2.0.0"
    }}
  }},
  "provider": {{
    "name": "write_pact_test_provider"
  }}
}}"#, super::VERSION.unwrap())));
}

#[test]
fn pact_merge_does_not_merge_different_consumers() {
    let pact = Pact { consumer: Consumer { name: s!("test_consumer") },
        provider: Provider { name: s!("test_provider") },
        interactions: vec![],
        metadata: btreemap!{},
        specification_version: PactSpecification::V1
    };
    let pact2 = Pact { consumer: Consumer { name: s!("test_consumer2") },
        provider: Provider { name: s!("test_provider") },
        interactions: vec![],
        metadata: btreemap!{},
        specification_version: PactSpecification::V1_1
    };
    expect!(pact.merge(&pact2)).to(be_err());
}

#[test]
fn pact_merge_does_not_merge_different_providers() {
    let pact = Pact { consumer: Consumer { name: s!("test_consumer") },
        provider: Provider { name: s!("test_provider") },
        interactions: vec![],
        metadata: btreemap!{},
        specification_version: PactSpecification::V1_1
    };
    let pact2 = Pact { consumer: Consumer { name: s!("test_consumer") },
        provider: Provider { name: s!("test_provider2") },
        interactions: vec![],
        metadata: btreemap!{},
        specification_version: PactSpecification::V1_1
    };
    expect!(pact.merge(&pact2)).to(be_err());
}

#[test]
fn pact_merge_does_not_merge_where_there_are_conflicting_interactions() {
    let pact = Pact { consumer: Consumer { name: s!("test_consumer") },
        provider: Provider { name: s!("test_provider") },
        interactions: vec![
            Interaction {
                description: s!("Test Interaction"),
                provider_state: Some(s!("Good state to be in")),
                request: Request::default_request(),
                response: Response::default_response()
            }
        ],
        metadata: btreemap!{},
        specification_version: PactSpecification::V1_1
    };
    let pact2 = Pact { consumer: Consumer { name: s!("test_consumer") },
        provider: Provider { name: s!("test_provider") },
        interactions: vec![
            Interaction {
                description: s!("Test Interaction"),
                provider_state: Some(s!("Good state to be in")),
                request: Request { path: s!("/other"), .. Request::default_request() },
                response: Response::default_response()
            }
        ],
        metadata: btreemap!{},
        specification_version: PactSpecification::V1_1
    };
    expect!(pact.merge(&pact2)).to(be_err());
}

#[test]
fn pact_merge_removes_duplicates() {
    let pact = Pact { consumer: Consumer { name: s!("test_consumer") },
        provider: Provider { name: s!("test_provider") },
        interactions: vec![
            Interaction {
                description: s!("Test Interaction"),
                provider_state: Some(s!("Good state to be in")),
                request: Request::default_request(),
                response: Response::default_response()
            }
        ],
        .. Pact::default()
    };
    let pact2 = Pact { consumer: Consumer { name: s!("test_consumer") },
        provider: Provider { name: s!("test_provider") },
        interactions: vec![
            Interaction {
                description: s!("Test Interaction"),
                provider_state: Some(s!("Good state to be in")),
                request: Request::default_request(),
                response: Response::default_response()
            },
            Interaction {
                description: s!("Test Interaction 2"),
                provider_state: Some(s!("Good state to be in")),
                request: Request::default_request(),
                response: Response::default_response()
            }
        ],
        .. Pact::default()
    };
    let merged_pact = pact.merge(&pact2);
    expect!(merged_pact.clone()).to(be_ok());
    expect!(merged_pact.clone().unwrap().interactions.len()).to(be_equal_to(2));
}

#[test]
fn interactions_do_not_conflict_if_they_have_different_descriptions() {
    let interaction1 = Interaction {
        description: s!("Test Interaction"),
        provider_state: Some(s!("Good state to be in")),
        request: Request::default_request(),
        response: Response::default_response()
    };
    let interaction2 =Interaction {
        description: s!("Test Interaction 2"),
        provider_state: Some(s!("Good state to be in")),
        request: Request::default_request(),
        response: Response::default_response()
    };
    expect(interaction1.conflicts_with(&interaction2)).to(be_empty());
}

#[test]
fn interactions_do_not_conflict_if_they_have_different_provider_states() {
    let interaction1 = Interaction {
        description: s!("Test Interaction"),
        provider_state: Some(s!("Good state to be in")),
        request: Request::default_request(),
        response: Response::default_response()
    };
    let interaction2 =Interaction {
        description: s!("Test Interaction"),
        provider_state: Some(s!("Bad state to be in")),
        request: Request::default_request(),
        response: Response::default_response()
    };
    expect(interaction1.conflicts_with(&interaction2)).to(be_empty());
}

#[test]
fn interactions_do_not_conflict_if_they_have_the_same_requests_and_responses() {
    let interaction1 = Interaction {
        description: s!("Test Interaction"),
        provider_state: Some(s!("Good state to be in")),
        request: Request::default_request(),
        response: Response::default_response()
    };
    let interaction2 =Interaction {
        description: s!("Test Interaction"),
        provider_state: Some(s!("Good state to be in")),
        request: Request::default_request(),
        response: Response::default_response()
    };
    expect(interaction1.conflicts_with(&interaction2)).to(be_empty());
}

#[test]
fn interactions_conflict_if_they_have_different_requests() {
    let interaction1 = Interaction {
        description: s!("Test Interaction"),
        provider_state: Some(s!("Good state to be in")),
        request: Request::default_request(),
        response: Response::default_response()
    };
    let interaction2 =Interaction {
        description: s!("Test Interaction"),
        provider_state: Some(s!("Good state to be in")),
        request: Request { method: s!("POST"), .. Request::default_request() },
        response: Response::default_response()
    };
    expect(interaction1.conflicts_with(&interaction2)).to_not(be_empty());
}

#[test]
fn interactions_conflict_if_they_have_different_responses() {
    let interaction1 = Interaction {
        description: s!("Test Interaction"),
        provider_state: Some(s!("Good state to be in")),
        request: Request::default_request(),
        response: Response::default_response()
    };
    let interaction2 =Interaction {
        description: s!("Test Interaction"),
        provider_state: Some(s!("Good state to be in")),
        request: Request::default_request(),
        response: Response { status: 400, .. Response::default_response() }
    };
    expect(interaction1.conflicts_with(&interaction2)).to_not(be_empty());
}

fn hash<T: Hash>(t: &T) -> u64 {
    let mut s = SipHasher::new();
    t.hash(&mut s);
    s.finish()
}

#[test]
fn hash_for_request() {
    let request1 = Request::default_request();
    let request2 = Request { method: s!("POST"), .. Request::default_request() };
    let request3 = Request { headers: Some(hashmap!{
        s!("H1") => s!("A")
    }), .. Request::default_request() };
    let request4 = Request { headers: Some(hashmap!{
        s!("H1") => s!("B")
    }), .. Request::default_request() };
    expect!(hash(&request1)).to(be_equal_to(hash(&request1)));
    expect!(hash(&request3)).to(be_equal_to(hash(&request3)));
    expect!(hash(&request1)).to_not(be_equal_to(hash(&request2)));
    expect!(hash(&request3)).to_not(be_equal_to(hash(&request4)));
}

#[test]
fn hash_for_response() {
    let response1 = Response::default_response();
    let response2 = Response { status: 400, .. Response::default_response() };
    let response3 = Response { headers: Some(hashmap!{
        s!("H1") => s!("A")
    }), .. Response::default_response() };
    let response4 = Response { headers: Some(hashmap!{
        s!("H1") => s!("B")
    }), .. Response::default_response() };
    expect!(hash(&response1)).to(be_equal_to(hash(&response1)));
    expect!(hash(&response3)).to(be_equal_to(hash(&response3)));
    expect!(hash(&response1)).to_not(be_equal_to(hash(&response2)));
    expect!(hash(&response3)).to_not(be_equal_to(hash(&response4)));
}

#[test]
fn matchers_from_json_handles_missing_matchers() {
    let json : serde_json::Value = serde_json::from_str(r#"
      {
          "path": "/",
          "query": "",
          "headers": {}
      }
     "#).unwrap();
    let matchers = matchers_from_json(&json, s!("deprecatedName"));
    expect!(matchers).to(be_none());
}

#[test]
fn matchers_from_json_handles_empty_matchers() {
    let json : serde_json::Value = serde_json::from_str(r#"
      {
          "path": "/",
          "query": "",
          "headers": {},
          "matchingRules": {}
      }
     "#).unwrap();
    let matchers = matchers_from_json(&json, s!("deprecatedName"));
    expect!(matchers).to(be_none());
}

#[test]
fn matchers_from_json_handles_matcher_with_no_matching_rules() {
    let json : serde_json::Value = serde_json::from_str(r#"
      {
          "path": "/",
          "query": "",
          "headers": {},
          "matchingRules": {
            "*.path": {}
          }
      }
     "#).unwrap();
    let matchers = matchers_from_json(&json, s!("deprecatedName"));
    expect!(matchers).to(be_some().value(hashmap!{
        s!("*.path") => hashmap!{}
    }));
}

#[test]
fn matchers_from_json_loads_matchers_correctly() {
    let json : serde_json::Value = serde_json::from_str(r#"
      {
          "path": "/",
          "query": "",
          "headers": {},
          "matchingRules": {
            "*.path": {
                "match": "regex",
                "regex": "\\d+"
            }
          }
      }
     "#).unwrap();
    let matchers = matchers_from_json(&json, s!("deprecatedName"));
    expect!(matchers).to(be_some().value(hashmap!{
        s!("*.path") => hashmap!{
            s!("match") => s!("regex"),
            s!("regex") => s!(r#"\d+"#)
        }
    }));
}

#[test]
fn matchers_from_json_loads_matchers_from_deprecated_name() {
    let json : serde_json::Value = serde_json::from_str(r#"
      {
          "path": "/",
          "query": "",
          "headers": {},
          "deprecatedName": {
            "*.path": {
                "match": "regex",
                "regex": "\\d+"
            }
          }
      }
     "#).unwrap();
    let matchers = matchers_from_json(&json, s!("deprecatedName"));
    expect!(matchers).to(be_some().value(hashmap!{
        s!("*.path") => hashmap!{
            s!("match") => s!("regex"),
            s!("regex") => s!(r#"\d+"#)
        }
    }));
}

#[test]
fn write_pact_test_with_matchers() {
    let pact = Pact { consumer: Consumer { name: s!("write_pact_test_consumer") },
        provider: Provider { name: s!("write_pact_test_provider") },
        interactions: vec![
            Interaction {
                description: s!("Test Interaction"),
                provider_state: Some(s!("Good state to be in")),
                request: Request {
                    matching_rules: Some(hashmap!{
                        s!("*.body") => hashmap!{ s!("match") => s!("type") }
                    }),
                    .. Request::default_request()
                },
                response: Response::default_response()
            }
        ],
        .. Pact::default() };
    let mut dir = env::temp_dir();
    let x = rand::random::<u16>();
    dir.push(format!("pact_test_{}", x));
    dir.push(pact.default_file_name());

    let result = pact.write_pact(dir.as_path());

    let pact_file = read_pact_file(dir.as_path().to_str().unwrap()).unwrap_or(s!(""));
    fs::remove_dir_all(dir.parent().unwrap()).unwrap_or(());

    expect!(result).to(be_ok());
    expect(pact_file).to(be_equal_to(format!(r#"{{
  "consumer": {{
    "name": "write_pact_test_consumer"
  }},
  "interactions": [
    {{
      "description": "Test Interaction",
      "providerState": "Good state to be in",
      "request": {{
        "matchingRules": {{
          "*.body": {{
            "match": "type"
          }}
        }},
        "method": "GET",
        "path": "/"
      }},
      "response": {{
        "status": 200
      }}
    }}
  ],
  "metadata": {{
    "pact-rust": {{
      "version": "{}"
    }},
    "pact-specification": {{
      "version": "2.0.0"
    }}
  }},
  "provider": {{
    "name": "write_pact_test_provider"
  }}
}}"#, super::VERSION.unwrap())));
}

#[test]
fn body_from_json_returns_missing_if_there_is_no_body() {
    let json : serde_json::Value = serde_json::from_str(r#"
      {
          "path": "/",
          "query": "",
          "headers": {},
          "matchingRules": {
            "*.path": {}
          }
      }
     "#).unwrap();
    let body = body_from_json(&json, &None);
    expect!(body).to(be_equal_to(OptionalBody::Missing));
}

#[test]
fn body_from_json_returns_null_if_the_body_is_null() {
    let json : serde_json::Value = serde_json::from_str(r#"
      {
          "path": "/",
          "query": "",
          "headers": {},
          "body": null
      }
     "#).unwrap();
    let body = body_from_json(&json, &None);
    expect!(body).to(be_equal_to(OptionalBody::Null));
}

#[test]
fn body_from_json_returns_json_string_if_the_body_is_json_but_not_a_string() {
    let json : serde_json::Value = serde_json::from_str(r#"
      {
          "path": "/",
          "query": "",
          "headers": {},
          "body": {
            "test": true
          }
      }
     "#).unwrap();
    let body = body_from_json(&json, &None);
    expect!(body).to(be_equal_to(OptionalBody::Present(s!("{\"test\":true}"))));
}

#[test]
fn body_from_json_returns_empty_if_the_body_is_an_empty_string() {
    let json : serde_json::Value = serde_json::from_str(r#"
      {
          "path": "/",
          "query": "",
          "headers": {},
          "body": ""
      }
     "#).unwrap();
    let body = body_from_json(&json, &None);
    expect!(body).to(be_equal_to(OptionalBody::Empty));
}

#[test]
fn body_from_json_returns_the_body_if_the_body_is_a_string() {
    let json : serde_json::Value = serde_json::from_str(r#"
      {
          "path": "/",
          "query": "",
          "headers": {},
          "body": "<?xml version=\"1.0\"?> <body></body>"
      }
     "#).unwrap();
    let body = body_from_json(&json, &None);
    expect!(body).to(be_equal_to(OptionalBody::Present(s!("<?xml version=\"1.0\"?> <body></body>"))));
}

#[test]
fn body_from_json_returns_the_a_json_formatted_body_if_the_body_is_a_string_and_the_content_type_is_json() {
    let json : serde_json::Value = serde_json::from_str(r#"
      {
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/json"},
          "body": "This is actually a JSON string"
      }
     "#).unwrap();
    let headers = headers_from_json(&json);
    let body = body_from_json(&json, &headers);
    expect!(body).to(be_equal_to(OptionalBody::Present(s!("\"This is actually a JSON string\""))));
}

#[test]
fn body_from_json_returns_the_body_if_the_content_type_is_json() {
    let json : serde_json::Value = serde_json::from_str(r#"
      {
          "path": "/",
          "query": "",
          "headers": {"Content-Type": "application/json"},
          "body": "{\"test\":true}"
      }
     "#).unwrap();
    let headers = headers_from_json(&json);
    let body = body_from_json(&json, &headers);
    expect!(body).to(be_equal_to(OptionalBody::Present(s!("{\"test\":true}"))));
}
