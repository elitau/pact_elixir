use models::matchingrules::*;
use itertools::Itertools;
use regex::Regex;

pub trait Matches<A> {
    fn matches(&self, actual: &A, matcher: &MatchingRule) -> Result<(), String>;
}

impl Matches<String> for String {
    fn matches(&self, actual: &String, matcher: &MatchingRule) -> Result<(), String> {
        debug!("comparing '{}' to '{}' using {:?}", self, actual, matcher);
        match *matcher {
          MatchingRule::Regex(ref regex) => {
            match Regex::new(regex) {
              Ok(re) => {
                if re.is_match(actual) {
                  Ok(())
                } else {
                  Err(format!("Expected '{}' to match '{}'", actual, regex))
                }
              },
              Err(err) => Err(format!("'{}' is not a valid regular expression - {}", regex, err))
            }
          },
          MatchingRule::Equality => {
            if self == actual {
              Ok(())
            } else {
              Err(format!("Expected '{}' to be equal to '{}'", self, actual))
            }
          },
          MatchingRule::Type |
          MatchingRule::MinType(_) |
          MatchingRule::MaxType(_)|
          MatchingRule::MinMaxType(_, _) => Ok(()),
          _ => Err(format!("Unable to match '{}' using {:?}", self, matcher))
       }
    }
}

impl Matches<u64> for String {
    fn matches(&self, actual: &u64, matcher: &MatchingRule) -> Result<(), String> {
        debug!("comparing '{}' to {} using {:?}", self, actual, matcher);
        match *matcher {
          MatchingRule::Regex(ref regex) => {
            match Regex::new(regex) {
              Ok(re) => {
                if re.is_match(&actual.to_string()) {
                  Ok(())
                } else {
                  Err(format!("Expected '{}' to match '{}'", actual, regex))
                }
              },
              Err(err) => Err(format!("'{}' is not a valid regular expression - {}", regex, err))
            }
           },
          MatchingRule::Type |
          MatchingRule::MinType(_) |
          MatchingRule::MaxType(_) |
          MatchingRule::MinMaxType(_, _) =>
            Err(format!("Expected '{}' (String) to be the same type as '{}' (Number)", self, actual)),
          MatchingRule::Equality => Err(format!("Expected '{}' (String) to be equal to '{}' (Number)", self, actual)),
          _ => Err(format!("Unable to match '{}' using {:?}", self, matcher))
       }
    }
}

impl Matches<u64> for u64 {
    fn matches(&self, actual: &u64, matcher: &MatchingRule) -> Result<(), String> {
        debug!("comparing '{}' to {} using {:?}", self, actual, matcher);
        match *matcher {
          MatchingRule::Regex(ref regex) => {
            match Regex::new(regex) {
              Ok(re) => {
                if re.is_match(&actual.to_string()) {
                  Ok(())
                } else {
                  Err(format!("Expected '{}' to match '{}'", actual, regex))
                }
              },
              Err(err) => Err(format!("'{}' is not a valid regular expression - {}", regex, err))
            }
          },
          MatchingRule::Type |
          MatchingRule::MinType(_) |
          MatchingRule::MaxType(_) |
          MatchingRule::MinMaxType(_, _) => Ok(()),
          MatchingRule::Equality => {
             if self == actual {
                 Ok(())
             } else {
                 Err(format!("Expected '{}' to be equal to '{}'", self, actual))
             }
          },
          _ => Err(format!("Unable to match '{}' using {:?}", self, matcher))
       }
    }
}

impl Matches<f64> for u64 {
    fn matches(&self, actual: &f64, matcher: &MatchingRule) -> Result<(), String> {
        debug!("comparing '{}' to {} using {:?}", self, actual, matcher);
        match *matcher {
          MatchingRule::Regex(ref regex) => {
            match Regex::new(regex) {
              Ok(re) => {
                if re.is_match(&actual.to_string()) {
                  Ok(())
                } else {
                  Err(format!("Expected '{}' to match '{}'", actual, regex))
                }
              },
              Err(err) => Err(format!("'{}' is not a valid regular expression - {}", regex, err))
            }
          },
          MatchingRule::Type |
          MatchingRule::MinType(_) |
          MatchingRule::MaxType(_) |
          MatchingRule::MinMaxType(_, _) =>
            Err(format!("Expected '{}' (Integer) to be the same type as '{}' (Decimal)", self, actual)),
          MatchingRule::Equality => Err(format!("Expected '{}' (Integer) to be equal to '{}' (Decimal)", self, actual)),
          _ => Err(format!("Unable to match '{}' using {:?}", self, matcher))
       }
    }
}

impl Matches<f64> for f64 {
    fn matches(&self, actual: &f64, matcher: &MatchingRule) -> Result<(), String> {
        debug!("comparing '{}' to {} using {:?}", self, actual, matcher);
        match *matcher {
          MatchingRule::Regex(ref regex) => {
            match Regex::new(regex) {
              Ok(re) => {
                if re.is_match(&actual.to_string()) {
                  Ok(())
                } else {
                  Err(format!("Expected '{}' to match '{}'", actual, regex))
                }
              },
              Err(err) => Err(format!("'{}' is not a valid regular expression - {}", regex, err))
            }
          },
          MatchingRule::Type |
          MatchingRule::MinType(_) |
          MatchingRule::MaxType(_) |
          MatchingRule::MinMaxType(_, _) => Ok(()),
          MatchingRule::Equality => {
             if self == actual {
                 Ok(())
             } else {
                 Err(format!("Expected '{}' to be equal to '{}'", self, actual))
             }
          },
          _ => Err(format!("Unable to match '{}' using {:?}", self, matcher))
       }
    }
}

impl Matches<u64> for f64 {
    fn matches(&self, actual: &u64, matcher: &MatchingRule) -> Result<(), String> {
        debug!("comparing '{}' to {} using {:?}", self, actual, matcher);
        match *matcher {
          MatchingRule::Regex(ref regex) => {
            match Regex::new(regex) {
              Ok(re) => {
                if re.is_match(&actual.to_string()) {
                  Ok(())
                } else {
                  Err(format!("Expected '{}' to match '{}'", actual, regex))
                }
              },
              Err(err) => Err(format!("'{}' is not a valid regular expression - {}", regex, err))
            }
          },
          MatchingRule::Type |
          MatchingRule::MinType(_) |
          MatchingRule::MaxType(_) |
          MatchingRule::MinMaxType(_, _) =>
            Err(format!("Expected '{}' (Decimal) to be the same type as '{}' (Integer)", self, actual)),
          MatchingRule::Equality => Err(format!("Expected '{}' (Decimal) to be equal to '{}' (Integer)", self, actual)),
          _ => Err(format!("Unable to match '{}' using {:?}", self, matcher))
       }
    }
}

fn select_best_matcher(category: &str, path: &Vec<String>, matchers: &MatchingRules) -> Option<RuleList> {
  if category == "body" {
    matchers.resolve_body_matchers_by_path(path)
  } else {
    match matchers.resolve_matchers(category, path) {
      Some(category) => category.rules.values().next().cloned(),
      None => None
    }
  }
}

pub fn match_values<E, A>(category: &str, path: &Vec<String>, matchers: MatchingRules, expected: &E, actual: &A) -> Result<(), Vec<String>>
    where E: Matches<A> {
    let matching_rules = select_best_matcher(category, path, &matchers);
    match matching_rules {
        None => Err(vec![format!("No matcher found for category '{}' and path '{}'", category,
                            path.iter().join("."))]),
        Some(ref rulelist) => {
          let results = rulelist.rules.iter().map(|rule| expected.matches(actual, rule)).collect::<Vec<Result<(), String>>>();
          if results.iter().all(|result| result.is_ok()) {
            Ok(())
          } else {
            Err(results.iter().filter(|result| result.is_err()).map(|result| result.clone().unwrap_err()).collect())
          }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::select_best_matcher;
    use expectest::prelude::*;

    #[test]
    fn select_best_matcher_selects_most_appropriate_by_weight() {
        let matchers = matchingrules!{
            "body" => {
                "$" => [ MatchingRule::Regex(s!("1")) ],
                "$.item1" => [ MatchingRule::Regex(s!("3")) ],
                "$.item2" => [ MatchingRule::Regex(s!("4")) ],
                "$.item1.level" => [ MatchingRule::Regex(s!("6")) ],
                "$.item1.level[1]" => [ MatchingRule::Regex(s!("7")) ],
                "$.item1.level[1].id" => [ MatchingRule::Regex(s!("8")) ],
                "$.item1.level[1].name" => [ MatchingRule::Regex(s!("9")) ],
                "$.item1.level[2]" => [ MatchingRule::Regex(s!("10")) ],
                "$.item1.level[2].id" => [ MatchingRule::Regex(s!("11")) ],
                "$.item1.level[*].id" => [ MatchingRule::Regex(s!("12")) ],
                "$.*.level[*].id" => [ MatchingRule::Regex(s!("13")) ]
            },
            "header" => {
                "item1" => [ MatchingRule::Regex(s!("5")) ]
            }
        };

        expect!(select_best_matcher("body", &vec![s!("$")], &matchers)).to(
          be_some().value(RuleList::new(MatchingRule::Regex(s!("1")))));
        expect!(select_best_matcher("body", &vec![s!("$"), s!("a")], &matchers)).to(
          be_some().value(RuleList::new(MatchingRule::Regex(s!("1")))));

        expect!(select_best_matcher("body", &vec![s!("$"), s!("item1")], &matchers)).to(
          be_some().value(RuleList::new(MatchingRule::Regex(s!("3")))));
        expect!(select_best_matcher("body", &vec![s!("$"), s!("item2")], &matchers)).to(
          be_some().value(RuleList::new(MatchingRule::Regex(s!("4")))));
        expect!(select_best_matcher("body", &vec![s!("$"), s!("item3")], &matchers)).to(
          be_some().value(RuleList::new(MatchingRule::Regex(s!("1")))));

        expect!(select_best_matcher("header", &vec![s!("item1")], &matchers)).to(
          be_some().value(RuleList::new(MatchingRule::Regex(s!("5")))));

        expect!(select_best_matcher("body", &vec![s!("$"), s!("item1"), s!("level")], &matchers)).to(
          be_some().value(RuleList::new(MatchingRule::Regex(s!("6")))));
        expect!(select_best_matcher("body", &vec![s!("$"), s!("item1"), s!("level"), s!("1")], &matchers)).to(
          be_some().value(RuleList::new(MatchingRule::Regex(s!("7")))));
        expect!(select_best_matcher("body", &vec![s!("$"), s!("item1"), s!("level"), s!("2")], &matchers)).to(
          be_some().value(RuleList::new(MatchingRule::Regex(s!("10")))));
        expect!(select_best_matcher("body", &vec![s!("$"), s!("item1"), s!("level"), s!("1"), s!("id")], &matchers)).to(
          be_some().value(RuleList::new(MatchingRule::Regex(s!("8")))));
        expect!(select_best_matcher("body", &vec![s!("$"), s!("item1"), s!("level"), s!("1"), s!("name")], &matchers)).to(
          be_some().value(RuleList::new(MatchingRule::Regex(s!("9")))));
        expect!(select_best_matcher("body", &vec![s!("$"), s!("item1"), s!("level"), s!("1"), s!("other")], &matchers)).to(
          be_some().value(RuleList::new(MatchingRule::Regex(s!("7")))));
        expect!(select_best_matcher("body", &vec![s!("$"), s!("item1"), s!("level"), s!("2"), s!("id")], &matchers)).to(
          be_some().value(RuleList::new(MatchingRule::Regex(s!("11")))));
        expect!(select_best_matcher("body", &vec![s!("$"), s!("item1"), s!("level"), s!("3"), s!("id")], &matchers)).to(
          be_some().value(RuleList::new(MatchingRule::Regex(s!("12")))));
        expect!(select_best_matcher("body", &vec![s!("$"), s!("item2"), s!("level"), s!("1"), s!("id")], &matchers)).to(
          be_some().value(RuleList::new(MatchingRule::Regex(s!("13")))));
        expect!(select_best_matcher("body", &vec![s!("$"), s!("item2"), s!("level"), s!("3"), s!("id")], &matchers)).to(
          be_some().value(RuleList::new(MatchingRule::Regex(s!("13")))));
    }

    #[test]
    fn select_best_matcher_selects_handles_missing_type_attribute() {
        let matchers = matchingrules!{
            "body" => {
                "$.item1" => [ MatchingRule::Regex(s!("3")) ],
                "$.item2" => [ MatchingRule::MinType(4) ],
                "$.item3" => [ MatchingRule::MaxType(4) ],
                "$.item4" => [ ]
            }
        };

        expect!(select_best_matcher("body", &vec![s!("$"), s!("item1")], &matchers)).to(
          be_some().value(RuleList::new(MatchingRule::Regex(s!("3")))));
        expect!(select_best_matcher("body", &vec![s!("$"), s!("item2")], &matchers)).to(
          be_some().value(RuleList::new(MatchingRule::MinType(4))));
        expect!(select_best_matcher("body", &vec![s!("$"), s!("item3")], &matchers)).to(
          be_some().value(RuleList::new(MatchingRule::MaxType(4))));
        expect!(select_best_matcher("body", &vec![s!("$"), s!("item4")], &matchers)).to(be_none());
    }

    #[test]
    fn equality_matcher_test() {
        let matcher = MatchingRule::Equality;
        expect!(s!("100").matches(&s!("100"), &matcher)).to(be_ok());
        expect!(s!("100").matches(&s!("101"), &matcher)).to(be_err());
        expect!(s!("100").matches(&100, &matcher)).to(be_err());
        expect!(100.matches(&100, &matcher)).to(be_ok());
        expect!(100.matches(&100.0, &matcher)).to(be_err());
        expect!(100.1f64.matches(&100.0, &matcher)).to(be_err());
    }

    #[test]
    fn regex_matcher_test() {
        let matcher = MatchingRule::Regex(s!("^\\d+$"));
        expect!(s!("100").matches(&s!("100"), &matcher)).to(be_ok());
        expect!(s!("100").matches(&s!("10a"), &matcher)).to(be_err());
        expect!(s!("100").matches(&100, &matcher)).to(be_ok());
        expect!(100.matches(&100, &matcher)).to(be_ok());
        expect!(100.matches(&100.01f64, &matcher)).to(be_err());
        expect!(100.1f64.matches(&100.02f64, &matcher)).to(be_err());
    }

    #[test]
    fn type_matcher_test() {
        let matcher = MatchingRule::Type;
        expect!(s!("100").matches(&s!("100"), &matcher)).to(be_ok());
        expect!(s!("100").matches(&s!("10a"), &matcher)).to(be_ok());
        expect!(s!("100").matches(&100, &matcher)).to(be_err());
        expect!(100.matches(&200, &matcher)).to(be_ok());
        expect!(100.matches(&100.1, &matcher)).to(be_err());
        expect!(100.1f64.matches(&100.2, &matcher)).to(be_ok());
    }

    #[test]
    fn min_type_matcher_test() {
        let matcher = MatchingRule::MinType(3);
        expect!(s!("100").matches(&s!("100"), &matcher)).to(be_ok());
        expect!(s!("100").matches(&s!("10a"), &matcher)).to(be_ok());
        expect!(s!("100").matches(&s!("10"), &matcher)).to(be_ok());
        expect!(s!("100").matches(&100, &matcher)).to(be_err());
        expect!(100.matches(&200, &matcher)).to(be_ok());
        expect!(100.matches(&100.1, &matcher)).to(be_err());
        expect!(100.1f64.matches(&100.2, &matcher)).to(be_ok());
    }

    #[test]
    fn max_type_matcher_test() {
        let matcher = MatchingRule::MaxType(3);
        expect!(s!("100").matches(&s!("100"), &matcher)).to(be_ok());
        expect!(s!("100").matches(&s!("10a"), &matcher)).to(be_ok());
        expect!(s!("100").matches(&s!("1000"), &matcher)).to(be_ok());
        expect!(s!("100").matches(&100, &matcher)).to(be_err());
        expect!(100.matches(&200, &matcher)).to(be_ok());
        expect!(100.matches(&100.1, &matcher)).to(be_err());
        expect!(100.1f64.matches(&100.2, &matcher)).to(be_ok());
    }
}
