#![feature(plugin)]
#![plugin(json_macros)]
extern crate rustc_serialize;
#[macro_use]
extern crate log;

pub mod model;

use model::{Request, Response};

#[derive(Debug, PartialEq)]
pub enum Mismatch {
    MethodMismatch { expected: String, actual: String },
    PathMismatch { expected: String, actual: String },
    StatusMismatch { expected: u16, actual: u16 },
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

pub fn match_request(expected: Request, actual: Request) -> Vec<Mismatch> {
    let mut mismatches = vec![];

    debug!("comparing to expected request: {:?}", expected);
    match_method(expected.method, actual.method, &mut mismatches);
    match_path(expected.path, actual.path, &mut mismatches);
    //   ++ matchQuery(expected, actual)
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
}
