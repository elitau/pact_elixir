extern crate rustc_serialize;
#[macro_use]
extern crate log;

pub mod model;

use model::{Request, Response};
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Mismatch {
    MethodMismatch { expected: String, actual: String },
    PathMismatch { expected: String, actual: String },
    StatusMismatch { expected: u16, actual: u16 },
    QueryMismatch { parameter: String, expected: String, actual: String, mismatch: String },
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

pub fn match_request(expected: Request, actual: Request) -> Vec<Mismatch> {
    let mut mismatches = vec![];

    debug!("comparing to expected request: {:?}", expected);
    match_method(expected.method, actual.method, &mut mismatches);
    match_path(expected.path, actual.path, &mut mismatches);
    match_query(expected.query, actual.query, &mut mismatches);
    //   ++ matchCookie(CollectionUtils.toOptionalList(expected.cookie), CollectionUtils.toOptionalList(actual.cookie))
    //   ++ matchRequestHeaders(expected, actual)
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
    //   ++ matchHeaders(expected, actual)
    //   ++ matchBody(expected, actual, providerDiffConfig)).toSeq

    mismatches
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

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
}
