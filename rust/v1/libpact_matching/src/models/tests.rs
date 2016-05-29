use super::*;
use std::collections::HashMap;
use rustc_serialize::json::Json;
use expectest::prelude::*;

#[test]
fn request_from_json_defaults_to_get() {
    let request_json = Json::from_str(r#"
      {
          "path": "/",
          "query": "",
          "headers": {}
      }
    "#).unwrap();
    let request = Request::from_json(&request_json);
    assert_eq!(request.method, "GET".to_string());
}

#[test]
fn request_from_json_defaults_to_root_for_path() {
    let request_json = Json::from_str(r#"
      {
          "method": "PUT",
          "query": "",
          "headers": {}
      }
    "#).unwrap();
    println!("request_json: {}", request_json);
    let request = Request::from_json(&request_json);
    assert_eq!(request.path, "/".to_string());
}

#[test]
fn response_from_json_defaults_to_status_200() {
    let response_json = Json::from_str(r#"
      {
          "headers": {}
      }
    "#).unwrap();
    let response = Response::from_json(&response_json);
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
    expect!(request.mimetype()).to(be_equal_to("text/plain"));
    expect!(Request {
        headers: Some(hashmap!{ s!("Content-Type") => s!("text/html") }), .. request.clone() }.mimetype())
        .to(be_equal_to("text/html"));
    expect!(Request {
        headers: Some(hashmap!{ s!("Content-Type") => s!("application/json; charset=UTF-8") }), .. request.clone() }.mimetype())
        .to(be_equal_to("application/json"));
    expect!(Request {
        headers: Some(hashmap!{ s!("Content-Type") => s!("application/json") }), .. request.clone() }.mimetype())
        .to(be_equal_to("application/json"));
    expect!(Request {
        headers: Some(hashmap!{ s!("CONTENT-TYPE") => s!("application/json ; charset=UTF-8") }), .. request.clone() }.mimetype())
        .to(be_equal_to("application/json"));
    expect!(Request {
        body: OptionalBody::Present(s!("{\"json\": true}")), .. request.clone() }.mimetype())
        .to(be_equal_to("application/json"));
    expect!(Request {
        body: OptionalBody::Present(s!("{}")), .. request.clone() }.mimetype())
        .to(be_equal_to("application/json"));
    expect!(Request {
        body: OptionalBody::Present(s!("[]")), .. request.clone() }.mimetype())
        .to(be_equal_to("application/json"));
    expect!(Request {
        body: OptionalBody::Present(s!("[1,2,3]")), .. request.clone() }.mimetype())
        .to(be_equal_to("application/json"));
    expect!(Request {
        body: OptionalBody::Present(s!("\"string\"")), .. request.clone() }.mimetype())
        .to(be_equal_to("application/json"));
    expect!(Request {
        body: OptionalBody::Present(s!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<json>false</json>")), .. request.clone() }.mimetype())
        .to(be_equal_to("application/xml"));
    expect!(Request {
        body: OptionalBody::Present(s!("<json>false</json>")), .. request.clone() }.mimetype())
        .to(be_equal_to("application/xml"));
    expect!(Request {
        body: OptionalBody::Present(s!("this is not json")), .. request.clone() }.mimetype())
        .to(be_equal_to("text/plain"));
    expect!(Request {
        body: OptionalBody::Present(s!("<html><body>this is also not json</body></html>")), .. request.clone() }.mimetype())
        .to(be_equal_to("text/html"));
}

#[test]
fn loading_interaction_from_json() {
    let interaction_json = r#"{
        "description": "String",
        "providerState": "provider state"
    }"#;
    let interaction = Interaction::from_json(0, &Json::from_str(interaction_json).unwrap());
    expect!(interaction.description).to(be_equal_to("String"));
    expect!(interaction.provider_state).to(be_some().value("provider state"));
}

#[test]
fn defaults_to_number_if_no_description() {
    let interaction_json = r#"{
        "providerState": "provider state"
    }"#;
    let interaction = Interaction::from_json(0, &Json::from_str(interaction_json).unwrap());
    expect!(interaction.description).to(be_equal_to("Interaction 0"));
    expect!(interaction.provider_state).to(be_some().value("provider state"));
}

#[test]
fn defaults_to_none_if_no_provider_state() {
    let interaction_json = r#"{
    }"#;
    let interaction = Interaction::from_json(0, &Json::from_str(interaction_json).unwrap());
    expect!(interaction.provider_state).to(be_none());
}

#[test]
fn defaults_to_none_if_provider_state_null() {
    let interaction_json = r#"{
        "providerState": null
    }"#;
    let interaction = Interaction::from_json(0, &Json::from_str(interaction_json).unwrap());
    expect!(interaction.provider_state).to(be_none());
}

#[test]
fn load_empty_pact() {
    let pact_json = r#"{}"#;
    let pact = Pact::from_json(&Json::from_str(pact_json).unwrap());
    expect!(pact.provider.name).to(be_equal_to("provider"));
    expect!(pact.consumer.name).to(be_equal_to("consumer"));
    expect!(pact.interactions.iter()).to(have_count(0));
    expect!(pact.metadata.iter()).to(have_count(0));
}

#[test]
fn missing_metadata() {
    let pact_json = r#"{}"#;
    let pact = Pact::from_json(&Json::from_str(pact_json).unwrap());
    expect!(pact.specification_version()).to(be_equal_to(PactSpecification::Unknown));
}

#[test]
fn missing_spec_version() {
    let pact_json = r#"{
        "metadata" : {
        }
    }"#;
    let pact = Pact::from_json(&Json::from_str(pact_json).unwrap());
    expect!(pact.specification_version()).to(be_equal_to(PactSpecification::Unknown));
}

#[test]
fn missing_version_in_spec_version() {
    let pact_json = r#"{
        "metadata" : {
            "pact-specification": {

            }
        }
    }"#;
    let pact = Pact::from_json(&Json::from_str(pact_json).unwrap());
    expect!(pact.specification_version()).to(be_equal_to(PactSpecification::Unknown));
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
    let pact = Pact::from_json(&Json::from_str(pact_json).unwrap());
    expect!(pact.specification_version()).to(be_equal_to(PactSpecification::Unknown));
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
    let pact = Pact::from_json(&Json::from_str(pact_json).unwrap());
    expect!(pact.specification_version()).to(be_equal_to(PactSpecification::V1));
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
    let pact = Pact::from_json(&Json::from_str(pact_json).unwrap());
    expect!(pact.specification_version()).to(be_equal_to(PactSpecification::Unknown));
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
    let pact = Pact::from_json(&Json::from_str(pact_json).unwrap());
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
    expect!(pact.specification_version()).to(be_equal_to(PactSpecification::Unknown));
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
    let pact = Pact::from_json(&Json::from_str(pact_json).unwrap());
    expect!(&pact.provider.name).to(be_equal_to("test_provider"));
    expect!(&pact.consumer.name).to(be_equal_to("test_consumer"));
    expect!(pact.metadata.iter()).to(have_count(2));
    expect!(&pact.metadata["pact-specification"]["version"]).to(be_equal_to("1.0.0"));
    expect!(pact.specification_version()).to(be_equal_to(PactSpecification::V1));
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
    let pact = Pact::from_json(&Json::from_str(pact_json).unwrap());
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
    let pact = Pact::from_json(&Json::from_str(pact_json).unwrap());
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
