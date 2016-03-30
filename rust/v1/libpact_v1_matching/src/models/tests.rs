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
}
