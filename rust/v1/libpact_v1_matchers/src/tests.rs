use super::*;
use super::match_header_value;
use std::collections::HashMap;
use expectest::prelude::*;
use quickcheck::{TestResult, quickcheck};
use libpact_v1_models::*;
use libpact_v1_models::model::{HttpPart, Request, Response, OptionalBody};

#[test]
fn match_method_returns_nothing_if_the_method_matches() {
    let mut mismatches = vec![];
    match_method("GET".to_string(), "GET".to_string(), &mut mismatches);
    assert!(mismatches.is_empty());
}

#[test]
fn match_method_returns_a_mismatch_if_the_method_does_not_match() {
    let mut mismatches = vec![];
    match_method("GET".to_string(), "POST".to_string(), &mut mismatches);
    assert!(!mismatches.is_empty());
    assert_eq!(mismatches[0], Mismatch::MethodMismatch { expected: "GET".to_string(), actual: "POST".to_string() });
}

#[test]
fn match_method_returns_nothing_if_the_method_matches_with_differnt_case() {
    let mut mismatches = vec![];
    match_method("POST".to_string(), "post".to_string(), &mut mismatches);
    assert!(mismatches.is_empty());
}

#[test]
fn match_status_returns_nothing_if_the_status_matches() {
    let mut mismatches = vec![];
    match_status(200, 200, &mut mismatches);
    assert!(mismatches.is_empty());
}

#[test]
fn match_status_returns_a_mismatch_if_the_status_does_not_match() {
    let mut mismatches = vec![];
    match_status(200, 300, &mut mismatches);
    assert!(!mismatches.is_empty());
    assert_eq!(mismatches[0], Mismatch::StatusMismatch { expected: 200, actual: 300 });
}

#[test]
fn match_query_returns_nothing_if_there_are_no_query_strings() {
    let mut mismatches = vec![];
    let expected = None;
    let actual = None;
    match_query(expected, actual, &mut mismatches);
    assert!(mismatches.is_empty());
}

#[test]
fn match_query_returns_a_mismatch_if_there_is_no_expected_query_string() {
    let mut mismatches = vec![];
    let expected = None;
    let mut query_map = HashMap::new();
    query_map.insert("a".to_string(), vec!["b".to_string()]);
    let actual = Some(query_map);
    match_query(expected, actual, &mut mismatches);
    assert!(!mismatches.is_empty());
    assert_eq!(mismatches[0], Mismatch::QueryMismatch { parameter: "a".to_string(),
        expected: "".to_string(), actual: "[\"b\"]".to_string(),
        mismatch: "Unexpected query parameter 'a' received".to_string() });
}

#[test]
fn match_query_returns_a_mismatch_if_there_is_no_actual_query_string() {
    let mut mismatches = vec![];
    let mut query_map = HashMap::new();
    query_map.insert("a".to_string(), vec!["b".to_string()]);
    let expected = Some(query_map);
    let actual = None;
    match_query(expected, actual, &mut mismatches);
    assert!(!mismatches.is_empty());
    assert_eq!(mismatches[0], Mismatch::QueryMismatch { parameter: "a".to_string(),
        expected: "[\"b\"]".to_string(), actual: "".to_string(),
        mismatch: "Expected query parameter 'a' but was missing".to_string() });
}

#[test]
fn match_query_returns_a_mismatch_if_there_is_an_actual_query_parameter_that_is_not_expected() {
    let mut mismatches = vec![];
    let mut query_map = HashMap::new();
    query_map.insert("a".to_string(), vec!["b".to_string()]);
    let expected = Some(query_map);
    query_map = HashMap::new();
    query_map.insert("a".to_string(), vec!["b".to_string()]);
    query_map.insert("c".to_string(), vec!["d".to_string()]);
    let actual = Some(query_map);
    match_query(expected, actual, &mut mismatches);
    assert!(!mismatches.is_empty());
    assert_eq!(mismatches[0], Mismatch::QueryMismatch { parameter: "c".to_string(),
        expected: "".to_string(), actual: "[\"d\"]".to_string(),
        mismatch: "Unexpected query parameter 'c' received".to_string() });
}

#[test]
fn match_query_returns_a_mismatch_if_there_is_an_expected_query_parameter_that_is_not_received() {
    let mut mismatches = vec![];
    let mut query_map = HashMap::new();
    query_map.insert("a".to_string(), vec!["b".to_string()]);
    query_map.insert("c".to_string(), vec!["d".to_string()]);
    let expected = Some(query_map);
    query_map = HashMap::new();
    query_map.insert("a".to_string(), vec!["b".to_string()]);
    let actual = Some(query_map);
    match_query(expected, actual, &mut mismatches);
    assert!(!mismatches.is_empty());
    assert_eq!(mismatches[0], Mismatch::QueryMismatch { parameter: "c".to_string(),
        expected: "[\"d\"]".to_string(), actual: "".to_string(),
        mismatch: "Expected query parameter 'c' but was missing".to_string() });
}

#[test]
fn match_query_returns_a_mismatch_if_there_is_an_empty_expected_query_parameter_and_a_non_empty_actual() {
    let mut mismatches = vec![];
    let mut query_map = HashMap::new();
    query_map.insert("a".to_string(), vec!["b".to_string()]);
    query_map.insert("c".to_string(), vec![]);
    let expected = Some(query_map);
    query_map = HashMap::new();
    query_map.insert("a".to_string(), vec!["b".to_string()]);
    query_map.insert("c".to_string(), vec!["d".to_string()]);
    let actual = Some(query_map);
    match_query(expected, actual, &mut mismatches);
    assert!(!mismatches.is_empty());
    assert_eq!(mismatches[0], Mismatch::QueryMismatch { parameter: "c".to_string(),
        expected: "[]".to_string(), actual: "[\"d\"]".to_string(),
        mismatch: "Expected an empty parameter list for 'c' but received [\"d\"]".to_string() });
}

#[test]
fn match_query_returns_a_mismatch_if_the_query_values_have_different_lengths() {
    let mut mismatches = vec![];
    let mut query_map = HashMap::new();
    query_map.insert("a".to_string(), vec!["b".to_string()]);
    query_map.insert("c".to_string(), vec!["d".to_string(), "e".to_string()]);
    let expected = Some(query_map);
    query_map = HashMap::new();
    query_map.insert("a".to_string(), vec!["b".to_string()]);
    query_map.insert("c".to_string(), vec!["d".to_string()]);
    let actual = Some(query_map);
    match_query(expected, actual, &mut mismatches);
    assert_eq!(mismatches.len(), 2);
    assert_eq!(mismatches[0], Mismatch::QueryMismatch { parameter: "c".to_string(),
        expected: "[\"d\", \"e\"]".to_string(), actual: "[\"d\"]".to_string(),
        mismatch: "Expected query parameter 'c' with 2 value(s) but received 1 value(s)".to_string() });
    assert_eq!(mismatches[1], Mismatch::QueryMismatch { parameter: "c".to_string(),
        expected: "[\"d\", \"e\"]".to_string(), actual: "[\"d\"]".to_string(),
        mismatch: "Expected query parameter 'c' value 'e' but was missing".to_string() });
}

#[test]
fn match_query_returns_a_mismatch_if_the_values_are_not_the_same() {
    let mut mismatches = vec![];
    let mut query_map = HashMap::new();
    query_map.insert("a".to_string(), vec!["b".to_string()]);
    let expected = Some(query_map);
    query_map = HashMap::new();
    query_map.insert("a".to_string(), vec!["c".to_string()]);
    let actual = Some(query_map);
    match_query(expected, actual, &mut mismatches);
    assert!(!mismatches.is_empty());
    assert_eq!(mismatches[0], Mismatch::QueryMismatch { parameter: "a".to_string(),
        expected: "b".to_string(), actual: "c".to_string(),
        mismatch: "Expected 'b' but received 'c' for query parameter 'a'".to_string() });
}

#[test]
fn matching_headers_be_true_when_headers_are_equal() {
    let mut mismatches = vec![];
    match_header_value(&"HEADER".to_string(), &"HEADER".to_string(), &"HEADER".to_string(), &mut mismatches);
    assert!(mismatches.is_empty());
}

#[test]
fn matching_headers_be_false_when_headers_are_not_equal() {
    let mut mismatches = vec![];
    match_header_value(&"HEADER".to_string(), &"HEADER".to_string(), &"HEADER2".to_string(), &mut mismatches);
    assert!(!mismatches.is_empty());
    assert_eq!(mismatches[0], Mismatch::HeaderMismatch { key: "HEADER".to_string(),
        expected: "HEADER".to_string(), actual: "HEADER2".to_string(),
        mismatch: "Expected header 'HEADER' to have value 'HEADER' but was 'HEADER2'".to_string() });
}

#[test]
fn matching_headers_exclude_whitespaces() {
    let mut mismatches = vec![];
    match_header_value(&"HEADER".to_string(), &"HEADER1, HEADER2,   3".to_string(), &"HEADER1,HEADER2,3".to_string(), &mut mismatches);
    expect!(mismatches).to(be_empty());
}

#[test]
fn matching_headers_includes_whitespaces_within_a_value() {
    let mut mismatches = vec![];
    match_header_value(&"HEADER".to_string(), &"HEADER 1, \tHEADER 2,\n3".to_string(),
        &"HEADER 1,HEADER 2,3".to_string(), &mut mismatches);
    expect!(mismatches).to(be_empty());
}

#[test]
fn content_type_header_matches_when_headers_are_equal() {
    let mut mismatches = vec![];
    match_header_value(&"CONTENT-TYPE".to_string(), &"application/json;charset=UTF-8".to_string(),
        &"application/json; charset=UTF-8".to_string(), &mut mismatches);
    expect!(mismatches).to(be_empty());
}

#[test]
fn content_type_header_does_not_match_when_headers_are_not_equal() {
    let mut mismatches = vec![];
    match_header_value(&"CONTENT-TYPE".to_string(), &"application/pdf;charset=UTF-8".to_string(),
        &"application/json;charset=UTF-8".to_string(), &mut mismatches);
    expect!(mismatches).to_not(be_empty());
}

#[test]
fn content_type_header_does_not_match_when_expected_is_empty() {
    let mut mismatches = vec![];
    match_header_value(&"CONTENT-TYPE".to_string(), &"".to_string(),
        &"application/json;charset=UTF-8".to_string(), &mut mismatches);
    expect!(mismatches).to_not(be_empty());
}

#[test]
fn content_type_header_does_not_match_when_actual_is_empty() {
    let mut mismatches = vec![];
    match_header_value(&"CONTENT-TYPE".to_string(), &"application/pdf;charset=UTF-8".to_string(),
        &"".to_string(), &mut mismatches);
    expect!(mismatches).to_not(be_empty());
}

#[test]
fn content_type_header_does_not_match_when_charsets_are_not_equal() {
    let mut mismatches = vec![];
    match_header_value(&"CONTENT-TYPE".to_string(), &"application/json;charset=UTF-8".to_string(),
        &"application/json;charset=UTF-16".to_string(), &mut mismatches);
    expect!(mismatches).to_not(be_empty());
}

#[test]
fn content_type_header_does_not_match_when_charsets_other_parameters_not_equal() {
    let mut mismatches = vec![];
    match_header_value(&"CONTENT-TYPE".to_string(), &"application/json;declaration=\"<950118.AEB0@XIson.com>\"".to_string(),
        &"application/json;charset=UTF-8".to_string(), &mut mismatches);
    expect!(mismatches).to_not(be_empty());
}

#[test]
fn content_type_header_does_match_when_charsets_is_missing_from_expected_header() {
    let mut mismatches = vec![];
    match_header_value(&"CONTENT-TYPE".to_string(), &"application/json".to_string(),
        &"application/json;charset=UTF-8".to_string(), &mut mismatches);
    expect!(mismatches).to(be_empty());
}

#[test]
fn body_does_not_match_if_different_content_types() {
    let mut mismatches = vec![];
    let expected = Request { method: s!("GET"), path: s!("/"), query: None,
        headers: Some(hashmap!{ s!("Content-Type") => s!("application/json") }),
        body: OptionalBody::Present(s!("")), matching_rules: None };
    let actual = Request { method: "GET".to_string(), path: "/".to_string(), query: None,
        headers: Some(hashmap!{ "Content-Type".to_string() => "text/plain".to_string() }),
        body: OptionalBody::Missing, matching_rules: None };
    match_body(&expected, &actual, DiffConfig::NoUnexpectedKeys, &mut mismatches);
    expect!(mismatches.clone()).to_not(be_empty());
    expect!(mismatches[0].clone()).to(be_equal_to(Mismatch::BodyTypeMismatch { expected: "application/json".to_string(),
        actual: s!("text/plain") }));
}

#[test]
fn body_matches_if_expected_is_missing() {
    let mut mismatches = vec![];
    let expected = Request { method: s!("GET"), path: s!("/"), query: None,
        headers: Some(hashmap!{ s!("Content-Type") => s!("application/json") }),
        body: OptionalBody::Missing, matching_rules: None };
    let actual = Request { method: s!("GET"), path: s!("/"), query: None,
        headers: Some(hashmap!{ s!("Content-Type") => s!("application/json") }),
        body: OptionalBody::Present(s!("{}")), matching_rules: None };
    match_body(&expected, &actual, DiffConfig::NoUnexpectedKeys, &mut mismatches);
    expect!(mismatches.clone()).to(be_empty());
}

#[test]
fn partial_equal_for_method_mismatch() {
    let mismatch = Mismatch::MethodMismatch { expected: s!("get"), actual: s!("post") };
    let mismatch2 = Mismatch::MethodMismatch { expected: s!("get"), actual: s!("post") };
    let mismatch3 = Mismatch::MethodMismatch { expected: s!("get"), actual: s!("put") };
    let mismatch4 = Mismatch::MethodMismatch { expected: s!("post"), actual: s!("post") };
    expect!(&mismatch).to(be_equal_to(&mismatch));
    expect!(&mismatch).to(be_equal_to(&mismatch2));
    expect!(&mismatch).to_not(be_equal_to(&mismatch3));
    expect!(&mismatch).to_not(be_equal_to(&mismatch4));
    expect!(&mismatch).to_not(be_equal_to(&Mismatch::PathMismatch { expected: s!("get"), actual: s!("post") }));
    expect!(&mismatch).to_not(be_equal_to(&Mismatch::StatusMismatch { expected: 200, actual: 300 }));
    expect!(&mismatch).to_not(be_equal_to(&Mismatch::QueryMismatch { parameter: s!(""), expected: s!(""), actual: s!(""), mismatch: s!("") }));
    expect!(&mismatch).to_not(be_equal_to(&Mismatch::HeaderMismatch { key: s!(""), expected: s!(""), actual: s!(""), mismatch: s!("") }));
    expect!(&mismatch).to_not(be_equal_to(&Mismatch::BodyTypeMismatch { expected: s!(""), actual: s!("") }));
    expect!(&mismatch).to_not(be_equal_to(&Mismatch::BodyMismatch { expected: Some(s!("get")), actual: Some(s!("post")), mismatch: s!(""), path: s!("/") }));
}

#[test]
fn partial_equal_for_path_mismatch() {
    let mismatch = Mismatch::PathMismatch { expected: s!("get"), actual: s!("post") };
    let mismatch2 = Mismatch::PathMismatch { expected: s!("get"), actual: s!("post") };
    let mismatch3 = Mismatch::PathMismatch { expected: s!("get"), actual: s!("put") };
    let mismatch4 = Mismatch::PathMismatch { expected: s!("post"), actual: s!("post") };
    expect!(&mismatch).to(be_equal_to(&mismatch));
    expect!(&mismatch).to(be_equal_to(&mismatch2));
    expect!(&mismatch).to_not(be_equal_to(&mismatch3));
    expect!(&mismatch).to_not(be_equal_to(&mismatch4));
    expect!(&mismatch).to_not(be_equal_to(&Mismatch::MethodMismatch { expected: s!("get"), actual: s!("post") }));
    expect!(&mismatch).to_not(be_equal_to(&Mismatch::StatusMismatch { expected: 200, actual: 300 }));
    expect!(&mismatch).to_not(be_equal_to(&Mismatch::QueryMismatch { parameter: s!(""), expected: s!(""), actual: s!(""), mismatch: s!("") }));
    expect!(&mismatch).to_not(be_equal_to(&Mismatch::HeaderMismatch { key: s!(""), expected: s!(""), actual: s!(""), mismatch: s!("") }));
    expect!(&mismatch).to_not(be_equal_to(&Mismatch::BodyTypeMismatch { expected: s!(""), actual: s!("") }));
    expect!(&mismatch).to_not(be_equal_to(&Mismatch::BodyMismatch { expected: Some(s!("get")), actual: Some(s!("post")), mismatch: s!(""), path: s!("/") }));
}

#[test]
fn partial_equal_for_status_mismatch() {
    let mismatch = Mismatch::StatusMismatch { expected: 100, actual: 200 };
    let mismatch2 = Mismatch::StatusMismatch { expected: 100, actual: 200 };
    let mismatch3 = Mismatch::StatusMismatch { expected: 100, actual: 300 };
    let mismatch4 = Mismatch::StatusMismatch { expected: 200, actual: 100 };
    expect!(&mismatch).to(be_equal_to(&mismatch));
    expect!(&mismatch).to(be_equal_to(&mismatch2));
    expect!(&mismatch).to_not(be_equal_to(&mismatch3));
    expect!(&mismatch).to_not(be_equal_to(&mismatch4));
    expect!(&mismatch).to_not(be_equal_to(&Mismatch::MethodMismatch { expected: s!("get"), actual: s!("post") }));
    expect!(&mismatch).to_not(be_equal_to(&Mismatch::PathMismatch { expected: s!("200"), actual: s!("300") }));
    expect!(&mismatch).to_not(be_equal_to(&Mismatch::QueryMismatch { parameter: s!(""), expected: s!(""), actual: s!(""), mismatch: s!("") }));
    expect!(&mismatch).to_not(be_equal_to(&Mismatch::HeaderMismatch { key: s!(""), expected: s!(""), actual: s!(""), mismatch: s!("") }));
    expect!(&mismatch).to_not(be_equal_to(&Mismatch::BodyTypeMismatch { expected: s!(""), actual: s!("") }));
    expect!(&mismatch).to_not(be_equal_to(&Mismatch::BodyMismatch { expected: Some(s!("get")), actual: Some(s!("post")), mismatch: s!(""), path: s!("/") }));
}

#[test]
fn partial_equal_for_body_type_mismatch() {
    let mismatch = Mismatch::BodyTypeMismatch { expected: s!("get"), actual: s!("post") };
    let mismatch2 = Mismatch::BodyTypeMismatch { expected: s!("get"), actual: s!("post") };
    let mismatch3 = Mismatch::BodyTypeMismatch { expected: s!("get"), actual: s!("put") };
    let mismatch4 = Mismatch::BodyTypeMismatch { expected: s!("post"), actual: s!("post") };
    expect!(&mismatch).to(be_equal_to(&mismatch));
    expect!(&mismatch).to(be_equal_to(&mismatch2));
    expect!(&mismatch).to_not(be_equal_to(&mismatch3));
    expect!(&mismatch).to_not(be_equal_to(&mismatch4));
    expect!(&mismatch).to_not(be_equal_to(&Mismatch::MethodMismatch { expected: s!("get"), actual: s!("post") }));
    expect!(&mismatch).to_not(be_equal_to(&Mismatch::StatusMismatch { expected: 200, actual: 300 }));
    expect!(&mismatch).to_not(be_equal_to(&Mismatch::QueryMismatch { parameter: s!(""), expected: s!(""), actual: s!(""), mismatch: s!("") }));
    expect!(&mismatch).to_not(be_equal_to(&Mismatch::HeaderMismatch { key: s!(""), expected: s!(""), actual: s!(""), mismatch: s!("") }));
    expect!(&mismatch).to_not(be_equal_to(&Mismatch::PathMismatch { expected: s!(""), actual: s!("") }));
    expect!(&mismatch).to_not(be_equal_to(&Mismatch::BodyMismatch { expected: Some(s!("get")), actual: Some(s!("post")), mismatch: s!(""), path: s!("/") }));
}

#[test]
fn partial_equal_for_query_mismatch() {
    let mismatch = Mismatch::QueryMismatch { parameter: s!("key"), expected: s!("v1"), actual: s!("v2"), mismatch: s!("") };
    let mismatch2 = Mismatch::QueryMismatch { parameter: s!("key"), expected: s!("v1"), actual: s!("v2"), mismatch: s!("") };
    let mismatch3 = Mismatch::QueryMismatch { parameter: s!("key2"), expected: s!("v1"), actual: s!("v2"), mismatch: s!("") };
    let mismatch4 = Mismatch::QueryMismatch { parameter: s!("key"), expected: s!("v100"), actual: s!("v2"), mismatch: s!("") };
    let mismatch5 = Mismatch::QueryMismatch { parameter: s!("key"), expected: s!("v1"), actual: s!("v200"), mismatch: s!("") };
    let mismatch6 = Mismatch::QueryMismatch { parameter: s!("key"), expected: s!("v1"), actual: s!("v2"), mismatch: s!("did not match") };
    expect!(&mismatch).to(be_equal_to(&mismatch));
    expect!(&mismatch).to(be_equal_to(&mismatch2));
    expect!(&mismatch).to(be_equal_to(&mismatch6));
    expect!(&mismatch).to_not(be_equal_to(&mismatch3));
    expect!(&mismatch).to_not(be_equal_to(&mismatch4));
    expect!(&mismatch).to_not(be_equal_to(&mismatch5));
    expect!(&mismatch).to_not(be_equal_to(&Mismatch::MethodMismatch { expected: s!("get"), actual: s!("post") }));
    expect!(&mismatch).to_not(be_equal_to(&Mismatch::StatusMismatch { expected: 200, actual: 300 }));
    expect!(&mismatch).to_not(be_equal_to(&Mismatch::PathMismatch { expected: s!(""), actual: s!("")}));
    expect!(&mismatch).to_not(be_equal_to(&Mismatch::HeaderMismatch { key: s!(""), expected: s!(""), actual: s!(""), mismatch: s!("") }));
    expect!(&mismatch).to_not(be_equal_to(&Mismatch::BodyTypeMismatch { expected: s!(""), actual: s!("") }));
    expect!(&mismatch).to_not(be_equal_to(&Mismatch::BodyMismatch { expected: Some(s!("get")), actual: Some(s!("post")), mismatch: s!(""), path: s!("/") }));
}

#[test]
fn partial_equal_for_header_mismatch() {
    let mismatch = Mismatch::HeaderMismatch { key: s!("key"), expected: s!("v1"), actual: s!("v2"), mismatch: s!("") };
    let mismatch2 = Mismatch::HeaderMismatch { key: s!("key"), expected: s!("v1"), actual: s!("v2"), mismatch: s!("") };
    let mismatch3 = Mismatch::HeaderMismatch { key: s!("key2"), expected: s!("v1"), actual: s!("v2"), mismatch: s!("") };
    let mismatch4 = Mismatch::HeaderMismatch { key: s!("key"), expected: s!("v100"), actual: s!("v2"), mismatch: s!("") };
    let mismatch5 = Mismatch::HeaderMismatch { key: s!("key"), expected: s!("v1"), actual: s!("v200"), mismatch: s!("") };
    let mismatch6 = Mismatch::HeaderMismatch { key: s!("key"), expected: s!("v1"), actual: s!("v2"), mismatch: s!("did not match") };
    expect!(&mismatch).to(be_equal_to(&mismatch));
    expect!(&mismatch).to(be_equal_to(&mismatch2));
    expect!(&mismatch).to(be_equal_to(&mismatch6));
    expect!(&mismatch).to_not(be_equal_to(&mismatch3));
    expect!(&mismatch).to_not(be_equal_to(&mismatch4));
    expect!(&mismatch).to_not(be_equal_to(&mismatch5));
    expect!(&mismatch).to_not(be_equal_to(&Mismatch::MethodMismatch { expected: s!("get"), actual: s!("post") }));
    expect!(&mismatch).to_not(be_equal_to(&Mismatch::StatusMismatch { expected: 200, actual: 300 }));
    expect!(&mismatch).to_not(be_equal_to(&Mismatch::PathMismatch { expected: s!(""), actual: s!("")}));
    expect!(&mismatch).to_not(be_equal_to(&Mismatch::QueryMismatch { parameter: s!(""), expected: s!(""), actual: s!(""), mismatch: s!("") }));
    expect!(&mismatch).to_not(be_equal_to(&Mismatch::BodyTypeMismatch { expected: s!(""), actual: s!("") }));
    expect!(&mismatch).to_not(be_equal_to(&Mismatch::BodyMismatch { expected: Some(s!("get")), actual: Some(s!("post")), mismatch: s!(""), path: s!("/") }));
}

#[test]
fn partial_equal_for_body_mismatch() {
    let mismatch = Mismatch::BodyMismatch { path: s!("key"), expected: Some(s!("v1")), actual: Some(s!("v2")), mismatch: s!("") };
    let mismatch2 = Mismatch::BodyMismatch { path: s!("key"), expected: Some(s!("v1")), actual: Some(s!("v2")), mismatch: s!("") };
    let mismatch3 = Mismatch::BodyMismatch { path: s!("key2"), expected: Some(s!("v1")), actual: Some(s!("v2")), mismatch: s!("") };
    let mismatch4 = Mismatch::BodyMismatch { path: s!("key"), expected: None, actual: Some(s!("v2")), mismatch: s!("") };
    let mismatch5 = Mismatch::BodyMismatch { path: s!("key"), expected: Some(s!("v1")), actual: None, mismatch: s!("") };
    let mismatch6 = Mismatch::BodyMismatch { path: s!("key"), expected: Some(s!("v1")), actual: Some(s!("v2")), mismatch: s!("did not match") };
    expect!(&mismatch).to(be_equal_to(&mismatch));
    expect!(&mismatch).to(be_equal_to(&mismatch2));
    expect!(&mismatch).to(be_equal_to(&mismatch6));
    expect!(&mismatch).to_not(be_equal_to(&mismatch3));
    expect!(&mismatch).to_not(be_equal_to(&mismatch4));
    expect!(&mismatch).to_not(be_equal_to(&mismatch5));
    expect!(&mismatch).to_not(be_equal_to(&Mismatch::MethodMismatch { expected: s!("get"), actual: s!("post") }));
    expect!(&mismatch).to_not(be_equal_to(&Mismatch::StatusMismatch { expected: 200, actual: 300 }));
    expect!(&mismatch).to_not(be_equal_to(&Mismatch::PathMismatch { expected: s!(""), actual: s!("")}));
    expect!(&mismatch).to_not(be_equal_to(&Mismatch::HeaderMismatch { key: s!(""), expected: s!(""), actual: s!(""), mismatch: s!("") }));
    expect!(&mismatch).to_not(be_equal_to(&Mismatch::BodyTypeMismatch { expected: s!(""), actual: s!("") }));
    expect!(&mismatch).to_not(be_equal_to(&Mismatch::QueryMismatch { parameter: s!(""), expected: s!("get"), actual: s!("post"), mismatch: s!("") }));
}
