extern crate rustc_serialize;
#[macro_use]
extern crate log;

pub mod model;

use model::{Request, Response};

#[derive(Debug, PartialEq)]
pub enum Mismatch {
    MethodMismatch { expected: String, actual: String }
}

pub fn match_method(expected: String, actual: String, mismatches: &mut Vec<Mismatch>) {
    if expected.to_lowercase() != actual.to_lowercase() {
        mismatches.push(Mismatch::MethodMismatch { expected: expected, actual: actual });
    }
}

pub fn match_request(expected: Request, actual: Request) -> Vec<Mismatch> {
    let mut mismatches = vec![];

    debug!("comparing to expected request: {:?}", expected);
    match_method(expected.method, actual.method, &mut mismatches);
    // (matchMethod(expected.getMethod, actual.getMethod)
    //   ++ matchPath(expected, actual)
    //   ++ matchQuery(expected, actual)
    //   ++ matchCookie(CollectionUtils.toOptionalList(expected.cookie), CollectionUtils.toOptionalList(actual.cookie))
    //   ++ matchRequestHeaders(expected, actual)
    //   ++ matchBody(expected, actual, diffConfig)).toSeq

    mismatches
}

pub fn match_response(expected: &Response, actual: &Response) -> Vec<Mismatch> {
    Vec::new()
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
}
