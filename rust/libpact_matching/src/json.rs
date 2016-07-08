use rustc_serialize::json::Json;
use rustc_serialize::json::ToJson;
use std::collections::btree_map::*;
use super::Mismatch;
use super::DiffConfig;
use models::Matchers;
use matchers::*;

fn type_of(json: &Json) -> String {
    match json {
        &Json::Object(_) => s!("Map"),
        &Json::Array(_) => s!("List"),
        _ => s!("")
    }
}

fn value_of(json: &Json) -> String {
    match json {
        &Json::String(ref s) => s.clone(),
        _ => format!("{}", json)
    }
}

impl Matches<Json> for Json {
    fn matches(&self, actual: &Json, matcher: &Matcher) -> Result<(), String> {
        debug!("comparing '{}' to '{}' using {:?}", self, actual, matcher);
        match *matcher {
           Matcher::RegexMatcher(ref regex) => {
               if regex.is_match(&actual.to_string()) {
                   Ok(())
               } else {
                   Err(format!("Expected '{}' to match '{}'", value_of(actual), regex))
               }
           },
           Matcher::TypeMatcher => Ok(()),
           Matcher::EqualityMatcher => {
               if self == actual {
                   Ok(())
               } else {
                   Err(format!("Expected '{}' to be equal to '{}'", value_of(self), value_of(actual)))
               }
           }
       }
    }
}

impl Matches<Vec<Json>> for Vec<Json> {
    fn matches(&self, actual: &Vec<Json>, matcher: &Matcher) -> Result<(), String> {
        debug!("comparing '{:?}' to '{:?}' using {:?}", self, actual, matcher);
        match *matcher {
           Matcher::RegexMatcher(ref regex) => {
               if regex.is_match(&Json::Array(actual.clone()).to_string()) {
                   Ok(())
               } else {
                   Err(format!("Expected '{:?}' to match '{}'", value_of(&Json::Array(actual.clone())), regex))
               }
           },
           Matcher::TypeMatcher => Ok(()),
           Matcher::EqualityMatcher => {
               if self == actual {
                   Ok(())
               } else {
                   Err(format!("Expected '{}' to be equal to '{}'", value_of(&Json::Array(self.clone())),
                    value_of(&&Json::Array(actual.clone()))))
               }
           }
       }
    }
}

pub fn match_json(expected: &String, actual: &String, config: DiffConfig,
    mismatches: &mut Vec<super::Mismatch>, matchers: &Option<Matchers>) {
    let expected_json = Json::from_str(expected);
    let actual_json = Json::from_str(actual);

    if expected_json.is_err() || actual_json.is_err() {
        match expected_json {
            Err(e) => {
                mismatches.push(Mismatch::BodyMismatch { path: s!("$.body"), expected: Some(expected.clone()),
                    actual: Some(actual.clone()),
                    mismatch: format!("Failed to parse the expected body: '{}'", e)});
            },
            _ => ()
        }
        match actual_json {
            Err(e) => {
                mismatches.push(Mismatch::BodyMismatch { path: s!("$.body"), expected: Some(expected.clone()),
                    actual: Some(actual.clone()),
                    mismatch: format!("Failed to parse the actual body: '{}'", e)});
            },
            _ => ()
        }
    } else {
        compare(&vec![s!("$"), s!("body")], &expected_json.unwrap(), &actual_json.unwrap(), &config,
            mismatches, matchers);
    }
}

fn compare(path: &Vec<String>, expected: &Json, actual: &Json, config: &DiffConfig,
    mismatches: &mut Vec<super::Mismatch>, matchers: &Option<Matchers>) {
    match (expected, actual) {
        (&Json::Object(ref emap), &Json::Object(ref amap)) => compare_maps(path, emap, amap, config, mismatches, matchers),
        (&Json::Object(_), _) => {
            mismatches.push(Mismatch::BodyMismatch { path: path.join("."),
                expected: Some(value_of(expected)),
                actual: Some(value_of(actual)),
                mismatch: format!("Type mismatch: Expected {} {} but received {} {}",
                    type_of(expected), expected, type_of(actual), actual)});
        },
        (&Json::Array(ref elist), &Json::Array(ref alist)) => compare_lists(path, elist, alist, config, mismatches, matchers),
        (&Json::Array(_), _) => {
            mismatches.push(Mismatch::BodyMismatch { path: path.join("."),
                expected: Some(value_of(expected)),
                actual: Some(value_of(actual)),
                mismatch: format!("Type mismatch: Expected {} {} but received {} {}",
                    type_of(expected), value_of(expected), type_of(actual), value_of(actual))});
        },
        (_, _) => compare_values(path, expected, actual, mismatches, matchers)
    }
}

fn compare_maps(path: &Vec<String>, expected: &BTreeMap<String, Json>, actual: &BTreeMap<String, Json>,
    config: &DiffConfig, mismatches: &mut Vec<super::Mismatch>, matchers: &Option<Matchers>) {
    if expected.is_empty() && !actual.is_empty() {
      mismatches.push(Mismatch::BodyMismatch { path: path.join("."),
          expected: Some(value_of(&expected.to_json())),
          actual: Some(value_of(&actual.to_json())),
          mismatch: format!("Expected an empty Map but received {}", value_of(&actual.to_json()))});
    } else {
        match config {
            &DiffConfig::AllowUnexpectedKeys if expected.len() > actual.len() => {
                mismatches.push(Mismatch::BodyMismatch { path: path.join("."),
                    expected: Some(value_of(&expected.to_json())),
                    actual: Some(value_of(&actual.to_json())),
                    mismatch: format!("Expected a Map with at least {} elements but received {} elements",
                    expected.len(), actual.len())});
            },
            &DiffConfig::NoUnexpectedKeys if expected.len() != actual.len() => {
                mismatches.push(Mismatch::BodyMismatch { path: path.join("."),
                    expected: Some(value_of(&expected.to_json())),
                    actual: Some(value_of(&actual.to_json())),
                    mismatch: format!("Expected a Map with {} elements but received {} elements",
                    expected.len(), actual.len())});
            },
            _ => ()
        }

        for (key, value) in expected.iter() {
            if actual.contains_key(key) {
                let mut p = path.to_vec();
                p.push(key.clone());
                compare(&p, value, &actual[key], config, mismatches, matchers);
            } else {
                mismatches.push(Mismatch::BodyMismatch { path: path.join("."),
                    expected: Some(value_of(&expected.to_json())),
                    actual: Some(value_of(&actual.to_json())),
                    mismatch: format!("Expected entry {}={} but was missing", key, value_of(value))});
            }
        }
    }
}

fn compare_lists(path: &Vec<String>, expected: &Vec<Json>, actual: &Vec<Json>, config: &DiffConfig,
    mismatches: &mut Vec<super::Mismatch>, matchers: &Option<Matchers>) {
    let spath = path.join(".");
    if matcher_is_defined(&path, matchers) {
        debug!("compare_lists: matcher defined for path '{}'", spath);
        let expected_json = Json::Array(expected.clone());
        let actual_json = Json::Array(actual.clone());
        match match_values(path, matchers.clone().unwrap(), &expected_json, &actual_json) {
            Err(message) => mismatches.push(Mismatch::BodyMismatch { path: path.join("."),
                expected: Some(expected_json.to_string()),
                actual: Some(actual_json.to_string()),
                mismatch: message.clone() }),
            Ok(_) => ()
        }
        compare_list_content(path, expected, actual, config, mismatches, matchers);
    } else {
        if expected.is_empty() && !actual.is_empty() {
            mismatches.push(Mismatch::BodyMismatch { path: spath,
                expected: Some(value_of(&expected.to_json())),
                actual: Some(value_of(&actual.to_json())),
                mismatch: format!("Expected an empty List but received {}", value_of(&actual.to_json()))});
        } else {
            compare_list_content(path, expected, actual, config, mismatches, matchers);
            if expected.len() != actual.len() {
                mismatches.push(Mismatch::BodyMismatch { path: spath,
                    expected: Some(value_of(&expected.to_json())),
                    actual: Some(value_of(&actual.to_json())),
                    mismatch: format!("Expected a List with {} elements but received {} elements",
                        expected.len(), actual.len())});
            }
        }
    }
}

fn compare_list_content(path: &Vec<String>, expected: &Vec<Json>, actual: &Vec<Json>, config: &DiffConfig,
    mismatches: &mut Vec<super::Mismatch>, matchers: &Option<Matchers>) {
    for (index, value) in expected.iter().enumerate() {
      let ps = index.to_string();
      if index < actual.len() {
          let mut p = path.to_vec();
          p.push(ps);
          compare(&p, value, &actual[index], config, mismatches, matchers);
      } else if !matcher_is_defined(&path, matchers) {
          mismatches.push(Mismatch::BodyMismatch { path: path.join("."),
              expected: Some(value_of(&expected.to_json())),
              actual: Some(value_of(&actual.to_json())),
              mismatch: format!("Expected {} but was missing", value_of(value))});
      }
    }
}

fn compare_values(path: &Vec<String>, expected: &Json, actual: &Json, mismatches: &mut Vec<super::Mismatch>,
    matchers: &Option<Matchers>) {
    let matcher_result = if matcher_is_defined(&path, matchers) {
        match_values(path, matchers.clone().unwrap(), expected, actual)
    } else {
        expected.matches(actual, &Matcher::EqualityMatcher)
    };
    match matcher_result {
        Err(message) => mismatches.push(Mismatch::BodyMismatch { path: path.join("."),
            expected: Some(format!("{}", expected)),
            actual: Some(format!("{}", actual)),
            mismatch: message.clone() }),
        Ok(_) => ()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use expectest::prelude::*;
    use Mismatch;
    use DiffConfig;

    #[test]
    fn match_json_handles_invalid_expected_json() {
        let mut mismatches = vec![];
        let expected = s!(r#"{"json": "is bad"#);
        let actual = s!("{}");
        match_json(&expected, &actual, DiffConfig::AllowUnexpectedKeys, &mut mismatches, &None);
        expect!(mismatches.iter()).to(have_count(1));
        let mismatch = mismatches[0].clone();
        expect!(&mismatch).to(be_equal_to(&Mismatch::BodyMismatch { path: s!("$.body"), expected: Some(expected),
            actual: Some(actual), mismatch: s!("")}));
    }

    #[test]
    fn match_json_handles_invalid_actual_json() {
        let mut mismatches = vec![];
        let expected = s!("{}");
        let actual = s!(r#"{json: "is bad"}"#);
        match_json(&expected, &actual, DiffConfig::AllowUnexpectedKeys, &mut mismatches, &None);
        expect!(mismatches.iter()).to(have_count(1));
        let mismatch = mismatches[0].clone();
        expect!(&mismatch).to(be_equal_to(&Mismatch::BodyMismatch { path: s!("$.body"), expected: Some(expected),
            actual: Some(actual), mismatch: s!("")}));
    }

    fn mismatch_message(mismatch: &Mismatch) -> String {
        match mismatch {
            &Mismatch::BodyMismatch{ path: _, expected: _, actual: _, mismatch: ref m } => m.clone(),
            _ => s!("")
        }
    }

    #[test]
    fn match_json_handles_expecting_a_map_but_getting_a_list() {
        let mut mismatches = vec![];
        let expected = s!(r#"{}"#);
        let actual = s!(r#"[]"#);
        match_json(&expected, &actual, DiffConfig::AllowUnexpectedKeys, &mut mismatches, &None);
        expect!(mismatches.iter()).to(have_count(1));
        let mismatch = mismatches[0].clone();
        expect!(&mismatch).to(be_equal_to(&Mismatch::BodyMismatch { path: s!("$.body"), expected: Some(expected),
            actual: Some(actual), mismatch: s!("")}));
        expect!(mismatch_message(&mismatch)).to(be_equal_to(s!("Type mismatch: Expected Map {} but received List []")));
    }

    #[test]
    fn match_json_handles_expecting_a_list_but_getting_a_map() {
        let mut mismatches = vec![];
        let expected = s!(r#"[{}]"#);
        let actual = s!(r#"{}"#);
        match_json(&expected, &actual, DiffConfig::AllowUnexpectedKeys, &mut mismatches, &None);
        expect!(mismatches.iter()).to(have_count(1));
        let mismatch = mismatches[0].clone();
        expect!(&mismatch).to(be_equal_to(&Mismatch::BodyMismatch { path: s!("$.body"), expected: Some(expected),
            actual: Some(actual), mismatch: s!("")}));
        expect!(mismatch_message(&mismatch)).to(be_equal_to(s!("Type mismatch: Expected List [{}] but received Map {}")));
    }

    #[test]
    fn match_json_handles_comparing_strings() {
        let mut mismatches = vec![];
        let val1 = s!(r#""string value""#);
        let val2 = s!(r#""other value""#);
        match_json(&val1, &val1, DiffConfig::AllowUnexpectedKeys, &mut mismatches, &None);
        expect!(mismatches.clone()).to(be_empty());
        match_json(&val1, &val2, DiffConfig::AllowUnexpectedKeys, &mut mismatches, &None);
        expect!(mismatches.iter()).to(have_count(1));
        let mismatch = mismatches[0].clone();
        expect!(&mismatch).to(be_equal_to(&Mismatch::BodyMismatch { path: s!("$.body"), expected: Some(val1),
            actual: Some(val2), mismatch: s!("")}));
        expect!(mismatch_message(&mismatch)).to(be_equal_to(s!("Expected 'string value' to be equal to 'other value'")));
    }

    #[test]
    fn match_json_handles_comparing_integers() {
        let mut mismatches = vec![];
        let val1 = s!(r#"100"#);
        let val2 = s!(r#"200"#);
        match_json(&val1, &val1, DiffConfig::AllowUnexpectedKeys, &mut mismatches, &None);
        expect!(mismatches.clone()).to(be_empty());
        match_json(&val1, &val2, DiffConfig::AllowUnexpectedKeys, &mut mismatches, &None);
        expect!(mismatches.iter()).to(have_count(1));
        let mismatch = mismatches[0].clone();
        expect!(&mismatch).to(be_equal_to(&Mismatch::BodyMismatch { path: s!("$.body"), expected: Some(val1),
            actual: Some(val2), mismatch: s!("")}));
        expect!(mismatch_message(&mismatch)).to(be_equal_to(s!("Expected '100' to be equal to '200'")));
    }

    #[test]
    fn match_json_handles_comparing_floats() {
        let mut mismatches = vec![];
        let val1 = s!(r#"100.01"#);
        let val2 = s!(r#"100.02"#);
        match_json(&val1, &val1, DiffConfig::AllowUnexpectedKeys, &mut mismatches, &None);
        expect!(mismatches.clone()).to(be_empty());
        match_json(&val1, &val2, DiffConfig::AllowUnexpectedKeys, &mut mismatches, &None);
        expect!(mismatches.iter()).to(have_count(1));
        let mismatch = mismatches[0].clone();
        expect!(&mismatch).to(be_equal_to(&Mismatch::BodyMismatch { path: s!("$.body"), expected: Some(val1),
            actual: Some(val2), mismatch: s!("")}));
        expect!(mismatch_message(&mismatch)).to(be_equal_to(s!("Expected '100.01' to be equal to '100.02'")));
    }

    #[test]
    fn match_json_handles_comparing_booleans() {
        let mut mismatches = vec![];
        let val1 = s!(r#"true"#);
        let val2 = s!(r#"false"#);
        match_json(&val1, &val1, DiffConfig::AllowUnexpectedKeys, &mut mismatches, &None);
        expect!(mismatches.clone()).to(be_empty());
        match_json(&val1, &val2, DiffConfig::AllowUnexpectedKeys, &mut mismatches, &None);
        expect!(mismatches.iter()).to(have_count(1));
        let mismatch = mismatches[0].clone();
        expect!(&mismatch).to(be_equal_to(&Mismatch::BodyMismatch { path: s!("$.body"), expected: Some(val1),
            actual: Some(val2), mismatch: s!("")}));
        expect!(mismatch_message(&mismatch)).to(be_equal_to(s!("Expected 'true' to be equal to 'false'")));
    }

    #[test]
    fn match_json_handles_comparing_nulls() {
        let mut mismatches = vec![];
        let val1 = s!(r#"null"#);
        let val2 = s!(r#"33"#);
        match_json(&val1, &val1, DiffConfig::AllowUnexpectedKeys, &mut mismatches, &None);
        expect!(mismatches.clone()).to(be_empty());
        match_json(&val1, &val2, DiffConfig::AllowUnexpectedKeys, &mut mismatches, &None);
        expect!(mismatches.iter()).to(have_count(1));
        let mismatch = mismatches[0].clone();
        expect!(&mismatch).to(be_equal_to(&Mismatch::BodyMismatch { path: s!("$.body"), expected: Some(val1),
            actual: Some(val2), mismatch: s!("")}));
        expect!(mismatch_message(&mismatch)).to(be_equal_to(s!("Expected 'null' to be equal to '33'")));
    }

    #[test]
    fn match_json_handles_comparing_lists() {
        let mut mismatches = vec![];
        let val1 = s!(r#"[]"#);
        let val2 = s!(r#"[11,22,33]"#);
        let val3 = s!(r#"[11,44,33]"#);
        let val4 = s!(r#"[11,44,33, 66]"#);

        match_json(&val1, &val1, DiffConfig::AllowUnexpectedKeys, &mut mismatches, &None);
        expect!(mismatches.clone()).to(be_empty());
        mismatches.clear();

        match_json(&val2, &val2, DiffConfig::AllowUnexpectedKeys, &mut mismatches, &None);
        expect!(mismatches.clone()).to(be_empty());
        mismatches.clear();

        match_json(&val3, &val3, DiffConfig::AllowUnexpectedKeys, &mut mismatches, &None);
        expect!(mismatches.clone()).to(be_empty());
        mismatches.clear();

        match_json(&val1, &val2, DiffConfig::AllowUnexpectedKeys, &mut mismatches, &None);
        expect!(mismatches.iter()).to(have_count(1));
        let mismatch = mismatches[0].clone();
        expect!(mismatch_message(&mismatch)).to(be_equal_to(s!("Expected an empty List but received [11,22,33]")));
        mismatches.clear();

        match_json(&val2, &val3, DiffConfig::AllowUnexpectedKeys, &mut mismatches, &None);
        expect!(mismatches.iter()).to(have_count(1));
        let mismatch = mismatches[0].clone();
        expect!(&mismatch).to(be_equal_to(&Mismatch::BodyMismatch { path: s!("$.body.1"),
            expected: Some(s!("22")), actual: Some(s!("44")), mismatch: s!("")}));
        expect!(mismatch_message(&mismatch)).to(be_equal_to(s!("Expected '22' to be equal to '44'")));
        mismatches.clear();

        match_json(&val3, &val4, DiffConfig::AllowUnexpectedKeys, &mut mismatches, &None);
        expect!(mismatches.iter()).to(have_count(1));
        let mismatch = mismatches[0].clone();
        expect!(&mismatch).to(be_equal_to(&Mismatch::BodyMismatch { path: s!("$.body"),
            expected: Some(s!("[11,44,33]")),
            actual: Some(s!("[11,44,33,66]")), mismatch: s!("")}));
        expect!(mismatch_message(&mismatch)).to(be_equal_to(s!("Expected a List with 3 elements but received 4 elements")));
        mismatches.clear();

        match_json(&val2, &val4, DiffConfig::AllowUnexpectedKeys, &mut mismatches, &None);
        expect!(mismatches.iter()).to(have_count(2));
        let mismatch = mismatches[0].clone();
        expect!(&mismatch).to(be_equal_to(&Mismatch::BodyMismatch { path: s!("$.body.1"),
            expected: Some(s!("22")),
            actual: Some(s!("44")), mismatch: s!("")}));
        expect!(mismatch_message(&mismatch)).to(be_equal_to(s!("Expected '22' to be equal to '44'")));
        let mismatch = mismatches[1].clone();
        expect!(&mismatch).to(be_equal_to(&Mismatch::BodyMismatch { path: s!("$.body"),
            expected: Some(s!("[11,22,33]")),
            actual: Some(s!("[11,44,33,66]")), mismatch: s!("")}));
        expect!(mismatch_message(&mismatch)).to(be_equal_to(s!("Expected a List with 3 elements but received 4 elements")));
        mismatches.clear();

        match_json(&val2, &val4, DiffConfig::AllowUnexpectedKeys, &mut mismatches, &Some(hashmap!{
            s!("$.body") => hashmap!{ s!("match") => s!("type") }
        }));
        expect!(mismatches.clone()).to(be_empty());
        match_json(&val4, &val2, DiffConfig::AllowUnexpectedKeys, &mut mismatches, &Some(hashmap!{
            s!("$.body") => hashmap!{ s!("match") => s!("type") }
        }));
        expect!(mismatches.clone()).to(be_empty());
    }

    #[test]
    fn match_json_handles_comparing_maps() {
        let mut mismatches = vec![];
        let val1 = s!(r#"{}"#);
        let val2 = s!(r#"{"a": 1, "b": 2}"#);
        let val3 = s!(r#"{"a": 1, "b": 3}"#);
        let val4 = s!(r#"{"a": 1, "b": 2, "c": 3}"#);

        match_json(&val1, &val1, DiffConfig::AllowUnexpectedKeys, &mut mismatches, &None);
        expect!(mismatches.clone()).to(be_empty());
        mismatches.clear();

        match_json(&val2, &val2, DiffConfig::AllowUnexpectedKeys, &mut mismatches, &None);
        expect!(mismatches.clone()).to(be_empty());
        mismatches.clear();

        match_json(&val4, &val4, DiffConfig::AllowUnexpectedKeys, &mut mismatches, &None);
        expect!(mismatches.clone()).to(be_empty());
        mismatches.clear();

        match_json(&val1, &val2, DiffConfig::AllowUnexpectedKeys, &mut mismatches, &None);
        expect!(mismatches.iter()).to(have_count(1));
        let mismatch = mismatches[0].clone();
        expect!(mismatch_message(&mismatch)).to(be_equal_to(s!("Expected an empty Map but received {\"a\":1,\"b\":2}")));
        mismatches.clear();

        match_json(&val2, &val3, DiffConfig::AllowUnexpectedKeys, &mut mismatches, &None);
        expect!(mismatches.iter()).to(have_count(1));
        let mismatch = mismatches[0].clone();
        expect!(&mismatch).to(be_equal_to(&Mismatch::BodyMismatch { path: s!("$.body.b"),
            expected: Some(s!("2")), actual: Some(s!("3")), mismatch: s!("")}));
        expect!(mismatch_message(&mismatch)).to(be_equal_to(s!("Expected '2' to be equal to '3'")));
        mismatches.clear();

        match_json(&val2, &val4, DiffConfig::AllowUnexpectedKeys, &mut mismatches, &None);
        expect!(mismatches.iter()).to(have_count(0));
        match_json(&val2, &val4, DiffConfig::NoUnexpectedKeys, &mut mismatches, &None);
        expect!(mismatches.iter()).to(have_count(1));
        let mismatch = mismatches[0].clone();
        expect!(&mismatch).to(be_equal_to(&Mismatch::BodyMismatch { path: s!("$.body"),
            expected: Some(s!("{\"a\":1,\"b\":2}")),
            actual: Some(s!("{\"a\":1,\"b\":2,\"c\":3}")), mismatch: s!("")}));
        expect!(mismatch_message(&mismatch)).to(be_equal_to(s!("Expected a Map with 2 elements but received 3 elements")));
        mismatches.clear();

        match_json(&val3, &val4, DiffConfig::AllowUnexpectedKeys, &mut mismatches, &None);
        expect!(mismatches.iter()).to(have_count(1));
        let mismatch = mismatches[0].clone();
        expect!(&mismatch).to(be_equal_to(&Mismatch::BodyMismatch { path: s!("$.body.b"),
            expected: Some(s!("3")),
            actual: Some(s!("2")), mismatch: s!("")}));
        expect!(mismatch_message(&mismatch)).to(be_equal_to(s!("Expected '3' to be equal to '2'")));
        mismatches.clear();

        match_json(&val3, &val4, DiffConfig::NoUnexpectedKeys, &mut mismatches, &None);
        expect!(mismatches.iter()).to(have_count(2));
        let mismatch = mismatches[0].clone();
        expect!(&mismatch).to(be_equal_to(&Mismatch::BodyMismatch { path: s!("$.body"),
            expected: Some(s!("{\"a\":1,\"b\":3}")),
            actual: Some(s!("{\"a\":1,\"b\":2,\"c\":3}")), mismatch: s!("")}));
        expect!(mismatch_message(&mismatch)).to(be_equal_to(s!("Expected a Map with 2 elements but received 3 elements")));
        let mismatch = mismatches[1].clone();
        expect!(&mismatch).to(be_equal_to(&Mismatch::BodyMismatch { path: s!("$.body.b"),
            expected: Some(s!("3")),
            actual: Some(s!("2")), mismatch: s!("")}));
        expect!(mismatch_message(&mismatch)).to(be_equal_to(s!("Expected '3' to be equal to '2'")));
        mismatches.clear();

        match_json(&val4, &val2, DiffConfig::AllowUnexpectedKeys, &mut mismatches, &None);
        expect!(mismatches.iter()).to(have_count(2));
        let mismatch = mismatches[0].clone();
        expect!(&mismatch).to(be_equal_to(&Mismatch::BodyMismatch { path: s!("$.body"),
            expected: Some(s!("{\"a\":1,\"b\":2,\"c\":3}")),
            actual: Some(s!("{\"a\":1,\"b\":2}")), mismatch: s!("")}));
        expect!(mismatch_message(&mismatch)).to(be_equal_to(s!("Expected a Map with at least 3 elements but received 2 elements")));
        let mismatch = mismatches[1].clone();
        expect!(&mismatch).to(be_equal_to(&Mismatch::BodyMismatch { path: s!("$.body"),
            expected: Some(s!("{\"a\":1,\"b\":2,\"c\":3}")),
            actual: Some(s!("{\"a\":1,\"b\":2}")), mismatch: s!("")}));
        expect!(mismatch_message(&mismatch)).to(be_equal_to(s!("Expected entry c=3 but was missing")));
        mismatches.clear();

        match_json(&val3, &val2, DiffConfig::AllowUnexpectedKeys, &mut mismatches, &Some(hashmap!{
            s!("$.body.*") => hashmap!{ s!("match") => s!("type") }
        }));
        expect!(mismatches.clone()).to(be_empty());
    }

}
