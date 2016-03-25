extern crate rustc_serialize;
#[macro_use] extern crate log;
#[macro_use] extern crate p_macro;
extern crate libpact_v1_models;

use libpact_v1_models::model::{Request, Response};
use std::collections::HashMap;
use std::iter::FromIterator;

mod text;
mod json;

static BODY_MATCHERS: [(&'static str, fn(mismatches: &mut Vec<Mismatch>)); 2] = [
    ("application/json", json::match_json),
    ("plain/text", text::match_text)
];

#[derive(Debug, PartialEq, Clone)]
pub enum Mismatch {
    MethodMismatch { expected: String, actual: String },
    PathMismatch { expected: String, actual: String },
    StatusMismatch { expected: u16, actual: u16 },
    QueryMismatch { parameter: String, expected: String, actual: String, mismatch: String },
    HeaderMismatch { key: String, expected: String, actual: String, mismatch: String },
}

pub fn match_method(expected: String, actual: String, mismatches: &mut Vec<Mismatch>) {
    if expected.to_lowercase() != actual.to_lowercase() {
        mismatches.push(Mismatch::MethodMismatch { expected: expected, actual: actual });
    }
}

pub fn match_path(expected: String, actual: String, mismatches: &mut Vec<Mismatch>) {
    if expected != actual {
        mismatches.push(Mismatch::PathMismatch { expected: expected, actual: actual });
    }
}

fn compare_query_parameter_value(key: &String, expected: &String, actual: &String,
    mismatches: &mut Vec<Mismatch>) {
    if expected != actual {
        mismatches.push(Mismatch::QueryMismatch { parameter: key.clone(),
            expected: expected.clone(),
            actual: actual.clone(),
            mismatch: format!("Expected '{}' but received '{}' for query parameter '{}'",
            expected, actual, key) });
    }
}

fn compare_query_parameter_values(key: &String, expected: &Vec<String>, actual: &Vec<String>,
    mismatches: &mut Vec<Mismatch>) {
    for (index, val) in expected.iter().enumerate() {
        if index < actual.len() {
            compare_query_parameter_value(key, val, &actual[index], mismatches);
        } else {
            mismatches.push(Mismatch::QueryMismatch { parameter: key.clone(),
                expected: format!("{:?}", expected),
                actual: format!("{:?}", actual),
                mismatch: format!("Expected query parameter '{}' value '{}' but was missing", key, val) });
        }
    }
}

fn match_query_values(key: &String, expected: &Vec<String>, actual: &Vec<String>,
    mismatches: &mut Vec<Mismatch>) {
    if expected.is_empty() && !actual.is_empty() {
        mismatches.push(Mismatch::QueryMismatch { parameter: key.clone(),
            expected: format!("{:?}", expected),
            actual: format!("{:?}", actual),
            mismatch: format!("Expected an empty parameter list for '{}' but received {:?}", key, actual) });
    } else {
        if expected.len() != actual.len() {
            mismatches.push(Mismatch::QueryMismatch { parameter: key.clone(),
                expected: format!("{:?}", expected),
                actual: format!("{:?}", actual),
                mismatch: format!(
                    "Expected query parameter '{}' with {} value(s) but received {} value(s)",
                    key, expected.len(), actual.len()) });
        }
        compare_query_parameter_values(key, expected, actual, mismatches);
    }
}

fn match_query_maps(expected: HashMap<String, Vec<String>>, actual: HashMap<String, Vec<String>>,
    mismatches: &mut Vec<Mismatch>) {
    for (key, value) in &expected {
        match actual.get(key) {
            Some(actual_value) => match_query_values(key, value, actual_value, mismatches),
            None => mismatches.push(Mismatch::QueryMismatch { parameter: key.clone(),
                expected: format!("{:?}", value),
                actual: "".to_string(),
                mismatch: format!("Expected query parameter '{}' but was missing", key) })
        }
    }
    for (key, value) in &actual {
        match expected.get(key) {
            Some(_) => (),
            None => mismatches.push(Mismatch::QueryMismatch { parameter: key.clone(),
                expected: "".to_string(),
                actual: format!("{:?}", value),
                mismatch: format!("Unexpected query parameter '{}' received", key) })
        }
    }
}

pub fn match_query(expected: Option<HashMap<String, Vec<String>>>,
    actual: Option<HashMap<String, Vec<String>>>, mismatches: &mut Vec<Mismatch>) {
    match (actual, expected) {
        (Some(aqm), Some(eqm)) => match_query_maps(eqm, aqm, mismatches),
        (Some(aqm), None) => for (key, value) in &aqm {
            mismatches.push(Mismatch::QueryMismatch { parameter: key.clone(),
                expected: "".to_string(),
                actual: format!("{:?}", value),
                mismatch: format!("Unexpected query parameter '{}' received", key) });
        },
        (None, Some(eqm)) => for (key, value) in &eqm {
            mismatches.push(Mismatch::QueryMismatch { parameter: key.clone(),
                expected: format!("{:?}", value),
                actual: "".to_string(),
                mismatch: format!("Expected query parameter '{}' but was missing", key) });
        },
        (None, None) => (),
    };
}

fn strip_whitespace<'a, T: FromIterator<&'a str>>(val: &'a String, split_by: &'a str) -> T {
    val.split(split_by).map(|v| v.trim().clone() ).collect()
}

fn parse_charset_parameters(parameters: &[&str]) -> HashMap<String, String> {
    parameters.iter().map(|v| v.split("=").map(|p| p.trim()).collect::<Vec<&str>>())
        .fold(HashMap::new(), |mut map, name_value| {
            map.insert(name_value[0].to_string(), name_value[1].to_string());
            map
        })
}

fn match_content_type(expected: &String, actual: &String, mismatches: &mut Vec<Mismatch>) {
    let expected_values: Vec<&str> = strip_whitespace(expected, ";");
    let actual_values: Vec<&str> = strip_whitespace(actual, ";");
    let expected_parameters = expected_values.as_slice().split_first().unwrap();
    let actual_parameters = actual_values.as_slice().split_first().unwrap();
    let header_mismatch = Mismatch::HeaderMismatch { key: "Content-Type".to_string(),
        expected: format!("{}", expected),
        actual: format!("{}", actual),
        mismatch: format!("Expected header 'Content-Type' to have value '{}' but was '{}'",
            expected, actual) };

    if expected_parameters.0 == actual_parameters.0 {
        let expected_parameter_map = parse_charset_parameters(expected_parameters.1);
        let actual_parameter_map = parse_charset_parameters(actual_parameters.1);
        for (k, v) in expected_parameter_map {
            if actual_parameter_map.contains_key(&k) {
                if v != *actual_parameter_map.get(&k).unwrap() {
                    mismatches.push(header_mismatch.clone());
                }
            } else {
                mismatches.push(header_mismatch.clone());
            }
        }
    } else {
        mismatches.push(header_mismatch.clone());
    }
}

fn match_header_value(key: &String, expected: &String, actual: &String, mismatches: &mut Vec<Mismatch>) {
    if key.to_lowercase() == "content-type" {
        match_content_type(expected, actual, mismatches);
    } else if strip_whitespace::<String>(expected, ",") != strip_whitespace::<String>(actual, ",") {
        mismatches.push(Mismatch::HeaderMismatch { key: key.clone(),
            expected: format!("{}", expected),
            actual: format!("{}", actual),
            mismatch: format!("Expected header '{}' to have value '{}' but was '{}'", key, expected, actual) });
    }
}

fn find_entry(map: &HashMap<String, String>, key: &String) -> Option<(String, String)> {
    match map.keys().find(|k| k.to_lowercase() == key.to_lowercase() ) {
        Some(k) => map.get(k).map(|v| (key.clone(), v.clone()) ),
        None => None
    }
}

fn match_header_maps(expected: HashMap<String, String>, actual: HashMap<String, String>,
    mismatches: &mut Vec<Mismatch>) {
    for (key, value) in &expected {
        match find_entry(&actual, key) {
            Some((_, actual_value)) => match_header_value(key, value, &actual_value, mismatches),
            None => mismatches.push(Mismatch::HeaderMismatch { key: key.clone(),
                expected: format!("{:?}", value),
                actual: "".to_string(),
                mismatch: format!("Expected header '{}' but was missing", key) })
        }
    }
}

pub fn match_headers(expected: Option<HashMap<String, String>>,
    actual: Option<HashMap<String, String>>, mismatches: &mut Vec<Mismatch>) {
    match (actual, expected) {
        (Some(aqm), Some(eqm)) => match_header_maps(eqm, aqm, mismatches),
        (Some(_), None) => (),
        (None, Some(eqm)) => for (key, value) in &eqm {
            mismatches.push(Mismatch::HeaderMismatch { key: key.clone(),
                expected: format!("{:?}", value),
                actual: "".to_string(),
                mismatch: format!("Expected header '{}' but was missing", key) });
        },
        (None, None) => (),
    };
}

pub fn match_request(expected: Request, actual: Request) -> Vec<Mismatch> {
    let mut mismatches = vec![];

    debug!("comparing to expected request: {:?}", expected);
    match_method(expected.method, actual.method, &mut mismatches);
    match_path(expected.path, actual.path, &mut mismatches);
    match_query(expected.query, actual.query, &mut mismatches);
    match_headers(expected.headers, actual.headers, &mut mismatches);
    //   ++ matchBody(expected, actual, diffConfig)).toSeq

    mismatches
}

pub fn match_status(expected: u16, actual: u16, mismatches: &mut Vec<Mismatch>) {
    if expected != actual {
        mismatches.push(Mismatch::StatusMismatch { expected: expected, actual: actual });
    }
}

pub fn match_response(expected: Response, actual: Response) -> Vec<Mismatch> {
    let mut mismatches = vec![];

    debug!("comparing to expected response: {:?}", expected);
    match_status(expected.status, actual.status, &mut mismatches);
    match_headers(expected.headers, actual.headers, &mut mismatches);
    //   ++ matchBody(expected, actual, providerDiffConfig)).toSeq

    mismatches
}

#[cfg(test)]
#[macro_use(expect)]
extern crate expectest;

#[cfg(test)]
extern crate quickcheck;

#[cfg(test)]
mod tests {
    use super::*;
    use super::match_header_value;
    use std::collections::HashMap;
    use expectest::prelude::*;
    use quickcheck::{TestResult, quickcheck};

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

}
