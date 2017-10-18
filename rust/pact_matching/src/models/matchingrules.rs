//! `matchingrules` module includes all the classes to deal with V3 format matchers

use serde_json::{self, Value};
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::str::FromStr;
use path_exp::*;
use super::PactSpecification;

fn json_to_string(value: &Value) -> String {
  match value {
    &Value::String(ref s) => s.clone(),
    _ => value.to_string()
  }
}

fn json_to_num(value: Option<Value>) -> Option<usize> {
  if let Some(value) = value {
    match value {
      Value::Number(n) => if n.is_i64() && n.as_i64().unwrap() > 0 { Some(n.as_i64().unwrap() as usize) }
        else if n.is_f64() { Some(n.as_f64().unwrap() as usize) }
        else if n.is_u64() { Some(n.as_u64().unwrap() as usize) }
        else { None },
      Value::String(ref s) => usize::from_str(&s.clone()).ok(),
      _ => None
    }
  } else {
    None
  }
}

fn matches_token(path_fragment: &String, path_token: &PathToken) -> usize {
  match *path_token {
    PathToken::Root if path_fragment == "$" => 2,
    PathToken::Field(ref name) if *path_fragment == name.clone() => 2,
    PathToken::Index(ref index) => match path_fragment.parse::<usize>() {
      Ok(ref i) if index == i => 2,
      _ => 0
    },
    PathToken::StarIndex => match path_fragment.parse::<usize>() {
      Ok(_) => 1,
      _ => 0
    },
    PathToken::Star => 1,
    _ => 0
  }
}

fn calc_path_weight(path_exp: String, path: &Vec<String>) -> usize {
  let weight = match parse_path_exp(path_exp.clone()) {
    Ok(path_tokens) => {
      debug!("Calculating weight for path tokens '{:?}' and path '{:?}'", path_tokens, path);
      if path.len() >= path_tokens.len() {
        path_tokens.iter().zip(path.iter())
          .fold(1, |acc, (token, fragment)| acc * matches_token(fragment, token))
      } else {
        0
      }
    },
    Err(err) => {
      warn!("Failed to parse path expression - {}", err);
      0
    }
  };
  debug!("Calculated weight {} for path '{}' and '{:?}'", weight, path_exp, path);
  weight
}

fn path_length(path_exp: String) -> usize {
  match parse_path_exp(path_exp.clone()) {
    Ok(path_tokens) => path_tokens.len(),
    Err(err) => {
      warn!("Failed to parse path expression - {}", err);
      0
    }
  }
}

/// Set of all matching rules
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
pub enum MatchingRule {
  /// Matcher using equals
  Equality,
  /// Match using a regular expression
  Regex(String),
  /// Match using the type of the value
  Type,
  /// Match using the type of the value and a minimum length for collections
  MinType(usize),
  /// Match using the type of the value and a maximum length for collections
  MaxType(usize),
  /// Match using the type of the value and a minimum and maximum length for collections
  MinMaxType(usize, usize),
  /// Match the value using a timestamp pattern
  Timestamp(String),
  /// Match the value using a time pattern
  Time(String),
  /// Match the value using a date pattern
  Date(String),
  /// Match if the value includes the given value
  Include(String),
  /// Match if the value is a number
  Number,
  /// Match if the value is an integer number
  Integer,
  /// Match if the value is a decimal number
  Decimal
}

impl MatchingRule {

  /// Builds a `MatchingRule` from a `Value` struct
  pub fn from_json(value: &Value) -> Option<MatchingRule> {
    match value {
      &Value::Object(ref m) => match m.get("match") {
        Some(value) => {
          let val = json_to_string(value);
          match val.as_str() {
            "regex" => match m.get(&val) {
              Some(s) => Some(MatchingRule::Regex(json_to_string(s))),
              None => None
            },
            "equality" => Some(MatchingRule::Equality),
            "include" => match m.get("value") {
              Some(s) => Some(MatchingRule::Include(json_to_string(s))),
              None => None
            },
            "type" => match (json_to_num(m.get("min").cloned()), json_to_num(m.get("max").cloned())) {
              (Some(min), Some(max)) => Some(MatchingRule::MinMaxType(min, max)),
              (Some(min), None) => Some(MatchingRule::MinType(min)),
              (None, Some(max)) => Some(MatchingRule::MaxType(max)),
              _ => Some(MatchingRule::Type)
            },
            "number" => Some(MatchingRule::Number),
            "integer" => Some(MatchingRule::Integer),
            "decimal" => Some(MatchingRule::Decimal),
            "real" => Some(MatchingRule::Decimal),
            "min" => match json_to_num(m.get(&val).cloned()) {
              Some(min) => Some(MatchingRule::MinType(min)),
              None => None
            },
            "max" => match json_to_num(m.get(&val).cloned()) {
              Some(max) => Some(MatchingRule::MaxType(max)),
              None => None
            },
            "timestamp" => match m.get(&val) {
              Some(s) => Some(MatchingRule::Timestamp(json_to_string(s))),
              None => None
            },
            "date" => match m.get(&val) {
              Some(s) => Some(MatchingRule::Date(json_to_string(s))),
              None => None
            },
            "time" => match m.get(&val) {
              Some(s) => Some(MatchingRule::Time(json_to_string(s))),
              None => None
            },
            _ => None
          }
        },
        None => if let Some(val) = m.get("regex") {
            Some(MatchingRule::Regex(json_to_string(val)))
          } else if let Some(val) = json_to_num(m.get("min").cloned()) {
            Some(MatchingRule::MinType(val))
          } else if let Some(val) = json_to_num(m.get("max").cloned()) {
            Some(MatchingRule::MaxType(val))
          } else if let Some(val) = m.get("timestamp") {
            Some(MatchingRule::Timestamp(json_to_string(val)))
          } else if let Some(val) = m.get("time") {
            Some(MatchingRule::Time(json_to_string(val)))
          } else if let Some(val) = m.get("date") {
            Some(MatchingRule::Date(json_to_string(val)))
          } else {
            None
          }
      },
      _ => None
    }
  }

  /// Converts this `MatchingRule` to a `Value` struct
  pub fn to_json(&self) -> Value {
    match self {
      &MatchingRule::Equality => json!({ "match": Value::String(s!("equality")) }),
      &MatchingRule::Regex(ref r) => json!({ "match": Value::String(s!("regex")),
        "regex": Value::String(r.clone()) }),
      &MatchingRule::Type => json!({ "match": Value::String(s!("type")) }),
      &MatchingRule::MinType(min) => json!({ "match": Value::String(s!("type")),
        "min": json!(min as u64) }),
      &MatchingRule::MaxType(max) => json!({ "match": Value::String(s!("type")),
        "max": json!(max as u64) }),
      &MatchingRule::MinMaxType(min, max) => json!({ "match": Value::String(s!("type")),
        "min": json!(min as u64), "max": json!(max as u64) }),
      &MatchingRule::Timestamp(ref t) => json!({ "match": Value::String(s!("timestamp")),
        "timestamp": Value::String(t.clone()) }),
      &MatchingRule::Time(ref t) => json!({ s!("match"): Value::String(s!("time")),
        s!("time"): Value::String(t.clone()) }),
      &MatchingRule::Date(ref d) => json!({ s!("match"): Value::String(s!("date")),
        s!("date"): Value::String(d.clone()) }),
      &MatchingRule::Include(ref s) => json!({ "match": Value::String(s!("include")),
        "value": Value::String(s.clone()) }),
      &MatchingRule::Number => json!({ "match": Value::String(s!("number")) }),
      &MatchingRule::Integer => json!({ "match": Value::String(s!("integer")) }),
      &MatchingRule::Decimal => json!({ "match": Value::String(s!("decimal")) })
    }
  }

}

/// Data structure for representing a list of rules and the logic needed to combine them
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
pub struct RuleList {
  /// List of rules to apply
  pub rules: Vec<MatchingRule>
}

impl RuleList {

  /// Creates a new empty rule list
  pub fn default() -> RuleList {
    RuleList {
      rules: Vec::new()
    }
  }

  /// Creates a new rule list with the single matching rule
  pub fn new(rule: MatchingRule) -> RuleList {
    RuleList {
      rules: vec![ rule ]
    }
  }

  fn to_v2_json(&self) -> Value {
    match self.rules.iter().next() {
      Some(rule) => rule.to_json(),
      None => json!({})
    }
  }

}

/// Data structure for representing a category of matching rules
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq)]
pub struct Category {
    /// Name of the category
    pub name: String,
    /// Matching rules for this category
    pub rules: HashMap<String, RuleList>
}

impl Category {

  /// Creates a default empty category
  pub fn default<S>(name: S) -> Category
    where S: Into<String>
  {
      Category {
          name: name.into(),
          rules: hashmap!{}
      }
  }

  /// If the matching rules in the category are empty
  pub fn is_empty(&self) -> bool {
    self.rules.is_empty()
  }

  /// If the matching rules in the category are not empty
  pub fn is_not_empty(&self) -> bool {
    !self.rules.is_empty()
  }

  /// Adds a rule from the Value representation
  pub fn rule_from_json(&mut self, key: &String, matcher_json: &Value) {
    match MatchingRule::from_json(matcher_json) {
      Some(matching_rule) => {
        let mut rules = self.rules.entry(key.clone()).or_insert(RuleList::default());
        rules.rules.push(matching_rule);
      },
      None => warn!("Could not parse matcher {:?}", matcher_json)
    }
  }

  /// Adds a rule to this category
  pub fn add_rule(&mut self, key: &String, matcher: MatchingRule) {
    let mut rules = self.rules.entry(key.clone()).or_insert(RuleList::default());
    rules.rules.push(matcher);
  }

  /// Filters the matchers in the category by the predicate, and returns a new category
  pub fn filter<F>(&self, predicate: F) -> Category
    where F : Fn(&(&String, &RuleList)) -> bool {
    Category {
      name: self.name.clone(),
      rules: self.rules.iter().filter(predicate)
        .map(|(path, rules)| (path.clone(), rules.clone())).collect()
    }
  }

  fn max_by_path(&self, path: &Vec<String>) -> Option<RuleList> {
    self.rules.iter().map(|(k, v)| (k, v, calc_path_weight(k.clone(), path)))
      .filter(|&(_, _, w)| w > 0)
      .max_by_key(|&(_, _, w)| w)
      .map(|(_, v, _)| v.clone())
  }

  /// Convert the rule category to JSON
  pub fn to_v2_json(&self) -> HashMap<String, Value> {
    let mut map = hashmap!{};

    if self.name == "body" {
      for (k, v) in self.rules.clone() {
        if k.is_empty() {
          map.insert("$.body".to_string(), v.to_v2_json());
        } else {
          map.insert(k.replace("$", "$.body"), v.to_v2_json());
        }
      }
    } else {
      for (k, v) in self.rules.clone() {
        map.insert(format!("$.{}.{}", self.name, k), v.to_v2_json());
      }
    }

    map
  }

}

impl Hash for Category {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.name.hash(state);
    for (k, v) in self.rules.clone() {
      k.hash(state);
      v.hash(state);
    }
  }
}

/// Data structure for representing a collection of matchers
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq)]
pub struct MatchingRules {
    /// Categories of matching rules
    pub rules: HashMap<String, Category>
}

impl MatchingRules {

    /// Create a empty set of matching rules
    pub fn default() -> MatchingRules {
        MatchingRules {
            rules: hashmap!{}
        }
    }

    /// If the matching rules are empty (that is there are no rules assigned to any categories)
    pub fn is_empty(&self) -> bool {
        self.rules.values().all(|category| category.is_empty())
    }

    /// If the matching rules are not empty (that is there is at least one rule assigned to a category)
    pub fn is_not_empty(&self) -> bool {
      self.rules.values().any(|category| category.is_not_empty())
    }

    /// Adds the category to the map of rules
    pub fn add_category(&mut self, category: &String) -> &mut Category {
        if !self.rules.contains_key(category) {
            self.rules.insert(category.clone(), Category::default(category.clone()));
        }
        self.rules.get_mut(category).unwrap()
    }

    /// Returns all the category names in this rule set
    pub fn categories(&self) -> HashSet<String> {
      self.rules.keys().cloned().collect()
    }

    /// Returns the category of rules for a given category name
    pub fn rules_for_category(&self, category: &String) -> Option<Category> {
      self.rules.get(category).cloned()
    }

    /// If there is a matcher defined for the category and path
    pub fn matcher_is_defined(&self, category: &str, path: &Vec<String>) -> bool {
      match self.resolve_matchers(category, path) {
        Some(ref category) => !category.is_empty(),
        None => false
      }
    }

    /// If there is a wildcard matcher defined for the category and path
    pub fn wildcard_matcher_is_defined(&self, category: &str, path: &Vec<String>) -> bool {
      match self.resolve_wildcard_matchers(category, path) {
        Some(ref category) => !category.filter(|&(val, _)| val.ends_with(".*")).is_empty(),
        None => false
      }
    }

    /// Returns a `Category` filtered with all rules that match the given path.
    pub fn resolve_matchers(&self, category: &str, path: &Vec<String>) -> Option<Category> {
      if category == "body" {
        self.rules_for_category(&s!(category)).map(|category| category.filter(|&(val, _)| {
          calc_path_weight(val.clone(), path) > 0
        }))
      } else if category == "header" || category == "query" {
        self.rules_for_category(&s!(category)).map(|category| category.filter(|&(val, _)| {
          path.len() == 1 && path[0] == *val
        }))
      } else {
        self.rules_for_category(&s!(category))
      }
    }

    /// Returns a list of rules from the body category that match the given path
    pub fn resolve_body_matchers_by_path(&self, path: &Vec<String>) -> Option<RuleList> {
      match self.rules_for_category(&s!("body")) {
        Some(category) => category.max_by_path(path),
        None => None
      }
    }

    fn resolve_wildcard_matchers(&self, category: &str, path: &Vec<String>) -> Option<Category> {
      if category == "body" {
        self.rules_for_category(&s!(category)).map(|category| category.filter(|&(val, _)| {
          calc_path_weight(val.clone(), path) > 0 && path_length(val.clone()) == path.len()
        }))
      } else if category == "header" || category == "query" {
        self.rules_for_category(&s!(category)).map(|category| category.filter(|&(val, _)| {
          path.len() == 1 && path[0] == *val
        }))
      } else {
        self.rules_for_category(&s!(category))
      }
    }

    fn load_from_v2_map(&mut self, map: &serde_json::Map<String, Value>) {
      for (key, v) in map {
        let path = key.split('.').map(|p| s!(p)).collect::<Vec<String>>();
        if key.starts_with("$.body") {
          if key == "$.body" {
            self.add_v2_rule(s!("body"), s!("$"), v);
          } else {
            self.add_v2_rule(s!("body"), format!("${}", s!(key[6..])), v);
          }
        } else if key.starts_with("$.headers") {
          self.add_v2_rule(s!("header"), path[2].clone(), v);
        } else {
          self.add_v2_rule(path[1].clone(), if path.len() > 2 { path[2].clone() } else { s!("") }, v);
        }
      }
    }

    fn add_rules(&mut self, category_name: &String, rules: &Value) {
      let mut category = self.add_category(category_name);
      if category_name == "path" {
        match rules.get("matchers") {
          Some(matchers) => match matchers {
            &Value::Array(ref array) => for matcher in array {
              category.rule_from_json(&s!(""), &matcher)
            },
            _ => ()
          },
          None => ()
        }
      } else {
        match rules {
          &Value::Object(ref m) => {
            for (k, v) in m {
              match v.get("matchers") {
                Some(matchers) => match matchers {
                  &Value::Array(ref array) => for matcher in array {
                    category.rule_from_json(k, &matcher)
                  },
                  _ => ()
                },
                None => ()
              }
            }
          },
          _ => ()
        }
      }
    }

  fn add_v2_rule(&mut self, category_name: String, sub_category: String, rule: &Value) {
    let mut category = self.add_category(&category_name);
    category.rule_from_json(&sub_category, rule);
  }

  /// Convert these rules to JSON
  pub fn to_v2_json(&self) -> Value {
    Value::Object(self.rules.iter().fold(serde_json::Map::new(), |mut map, (_, category)| {
      for (key, value) in category.to_v2_json() {
        map.insert(key.clone(), value);
      }
      map
    }))
  }
}

impl Hash for MatchingRules {
  fn hash<H: Hasher>(&self, state: &mut H) {
    for (k, v) in self.rules.clone() {
      k.hash(state);
      v.hash(state);
    }
  }
}

/// Parses the matching rules from the Value structure
pub fn matchers_from_json(value: &Value, deprecated_name: &Option<String>) -> MatchingRules {
  let matchers_json = match (value.get("matchingRules"), deprecated_name.clone().and_then(|name| value.get(&name))) {
    (Some(v), _) => Some(v),
    (None, Some(v)) => Some(v),
    (None, None) => None
  };

  let mut matching_rules = MatchingRules::default();
  match matchers_json {
      Some(value) => match value {
        &Value::Object(ref m) => {
            matching_rules.load_from_v2_map(m)
        },
        _ => ()
      },
      None => ()
  }
  matching_rules
}

/// Generates a Value structure for the provided matching rules
pub fn matchers_to_json(matchers: &MatchingRules, spec_version: &PactSpecification) -> Value {
   match spec_version {
     _ => matchers.to_v2_json()
   }
}

#[macro_export]
macro_rules! matchingrules {
    ( $( $name:expr => {
        $( $subname:expr => [ $( $matcher:expr ), * ] ),*
    }), * ) => {{
        let mut _rules = $crate::models::matchingrules::MatchingRules::default();
        $({
            let mut _category = _rules.add_category(&$name.to_string());
            $({
              $({
                _category.add_rule(&$subname.to_string(), $matcher);
              })*
            })*
        })*
        _rules
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::{json_to_string, json_to_num, calc_path_weight, matches_token};
    use expectest::prelude::*;
    use serde_json::Value;

    #[test]
    fn rules_are_empty_when_there_are_no_categories() {
        expect!(MatchingRules::default().is_empty()).to(be_true());
    }

    #[test]
    fn rules_are_empty_when_there_are_only_empty_categories() {
        expect!(MatchingRules {
            rules: hashmap!{
                s!("body") => Category::default(s!("body")),
                s!("header") => Category::default(s!("header")),
                s!("query") => Category::default(s!("query")),
            }
        }.is_empty()).to(be_true());
    }

    #[test]
    fn rules_are_not_empty_when_there_is_a_nonempty_category() {
        expect!(MatchingRules {
            rules: hashmap!{
                s!("body") => Category::default(s!("body")),
                s!("header") => Category::default(s!("headers")),
                s!("query") => Category {
                    name: s!("query"),
                    rules: hashmap!{
                      s!("") => RuleList {
                        rules: vec![ MatchingRule::Equality ]
                      }
                    }
                },
            }
        }.is_empty()).to(be_false());
    }

    #[test]
    fn matchers_from_json_test() {
        expect!(matchers_from_json(&Value::Null, &None).rules.iter()).to(be_empty());
    }

  #[test]
  fn loads_v2_matching_rules() {
    let matching_rules_json = Value::from_str(r#"{"matchingRules": {
      "$.path": { "match": "regex", "regex": "\\w+" },
      "$.query.Q1": { "match": "regex", "regex": "\\d+" },
      "$.header.HEADERY": {"match": "include", "value": "ValueA"},
      "$.body.animals": {"min": 1, "match": "type"},
      "$.body.animals[*].*": {"match": "type"},
      "$.body.animals[*].children": {"min": 1},
      "$.body.animals[*].children[*].*": {"match": "type"}
    }}"#).unwrap();

    let matching_rules = matchers_from_json(&matching_rules_json, &None);

    expect!(matching_rules.rules.iter()).to_not(be_empty());
    expect!(matching_rules.categories()).to(be_equal_to(hashset!{ s!("path"), s!("query"), s!("header"), s!("body") }));
    expect!(matching_rules.rules_for_category(&s!("path"))).to(be_some().value(Category {
      name: s!("path"),
      rules: hashmap! { s!("") => RuleList { rules: vec![ MatchingRule::Regex(s!("\\w+")) ] } }
    }));
    expect!(matching_rules.rules_for_category(&s!("query"))).to(be_some().value(Category {
      name: s!("query"),
      rules: hashmap!{ s!("Q1") => RuleList { rules: vec![ MatchingRule::Regex(s!("\\d+")) ] } }
    }));
    expect!(matching_rules.rules_for_category(&s!("header"))).to(be_some().value(Category {
      name: s!("header"),
      rules: hashmap!{ s!("HEADERY") => RuleList { rules: vec![
        MatchingRule::Include(s!("ValueA")) ] } }
    }));
    expect!(matching_rules.rules_for_category(&s!("body"))).to(be_some().value(Category {
      name: s!("body"),
      rules: hashmap!{
        s!("$.animals") => RuleList { rules: vec![ MatchingRule::MinType(1) ] },
        s!("$.animals[*].*") => RuleList { rules: vec![ MatchingRule::Type ] },
        s!("$.animals[*].children") => RuleList { rules: vec![ MatchingRule::MinType(1) ] },
        s!("$.animals[*].children[*].*") => RuleList { rules: vec![ MatchingRule::Type ] }
      }
    }));
  }

  #[test]
  fn json_to_string_test() {
    expect!(json_to_string(&Value::from_str("\"test string\"").unwrap())).to(be_equal_to(s!("test string")));
    expect!(json_to_string(&Value::from_str("null").unwrap())).to(be_equal_to(s!("null")));
    expect!(json_to_string(&Value::from_str("100").unwrap())).to(be_equal_to(s!("100")));
    expect!(json_to_string(&Value::from_str("100.10").unwrap())).to(be_equal_to(s!("100.1")));
    expect!(json_to_string(&Value::from_str("{}").unwrap())).to(be_equal_to(s!("{}")));
    expect!(json_to_string(&Value::from_str("[]").unwrap())).to(be_equal_to(s!("[]")));
    expect!(json_to_string(&Value::from_str("true").unwrap())).to(be_equal_to(s!("true")));
    expect!(json_to_string(&Value::from_str("false").unwrap())).to(be_equal_to(s!("false")));
  }

  #[test]
  fn json_to_num_test() {
    expect!(json_to_num(Value::from_str("\"test string\"").ok())).to(be_none());
    expect!(json_to_num(Value::from_str("null").ok())).to(be_none());
    expect!(json_to_num(Value::from_str("{}").ok())).to(be_none());
    expect!(json_to_num(Value::from_str("[]").ok())).to(be_none());
    expect!(json_to_num(Value::from_str("true").ok())).to(be_none());
    expect!(json_to_num(Value::from_str("false").ok())).to(be_none());
    expect!(json_to_num(Value::from_str("100").ok())).to(be_some().value(100));
    expect!(json_to_num(Value::from_str("-100").ok())).to(be_none());
    expect!(json_to_num(Value::from_str("100.10").ok())).to(be_some().value(100));
  }

  #[test]
  fn matching_rule_from_json_test() {
    expect!(MatchingRule::from_json(&Value::from_str("\"test string\"").unwrap())).to(be_none());
    expect!(MatchingRule::from_json(&Value::from_str("null").unwrap())).to(be_none());
    expect!(MatchingRule::from_json(&Value::from_str("{}").unwrap())).to(be_none());
    expect!(MatchingRule::from_json(&Value::from_str("[]").unwrap())).to(be_none());
    expect!(MatchingRule::from_json(&Value::from_str("true").unwrap())).to(be_none());
    expect!(MatchingRule::from_json(&Value::from_str("false").unwrap())).to(be_none());
    expect!(MatchingRule::from_json(&Value::from_str("100").unwrap())).to(be_none());
    expect!(MatchingRule::from_json(&Value::from_str("100.10").unwrap())).to(be_none());
    expect!(MatchingRule::from_json(&Value::from_str("{\"stuff\": 100}").unwrap())).to(be_none());
    expect!(MatchingRule::from_json(&Value::from_str("{\"match\": \"stuff\"}").unwrap())).to(be_none());

    expect!(MatchingRule::from_json(&Value::from_str("{\"regex\": \"[0-9]\"}").unwrap())).to(
      be_some().value(MatchingRule::Regex(s!("[0-9]"))));
    expect!(MatchingRule::from_json(&Value::from_str("{\"min\": 100}").unwrap())).to(
      be_some().value(MatchingRule::MinType(100)));
    expect!(MatchingRule::from_json(&Value::from_str("{\"max\": 100}").unwrap())).to(
      be_some().value(MatchingRule::MaxType(100)));
    expect!(MatchingRule::from_json(&Value::from_str("{\"timestamp\": \"yyyy\"}").unwrap())).to(
      be_some().value(MatchingRule::Timestamp(s!("yyyy"))));
    expect!(MatchingRule::from_json(&Value::from_str("{\"date\": \"yyyy\"}").unwrap())).to(
      be_some().value(MatchingRule::Date(s!("yyyy"))));
    expect!(MatchingRule::from_json(&Value::from_str("{\"time\": \"hh:mm\"}").unwrap())).to(
      be_some().value(MatchingRule::Time(s!("hh:mm"))));

    expect!(MatchingRule::from_json(&Value::from_str("{\"match\": \"regex\", \"regex\": \"[0-9]\"}").unwrap())).to(
      be_some().value(MatchingRule::Regex(s!("[0-9]"))));
    expect!(MatchingRule::from_json(&Value::from_str("{\"match\": \"regex\"}").unwrap())).to(be_none());

    expect!(MatchingRule::from_json(&Value::from_str("{\"match\": \"equality\"}").unwrap())).to(
      be_some().value(MatchingRule::Equality));

    expect!(MatchingRule::from_json(&Value::from_str("{\"match\": \"include\", \"value\": \"A\"}").unwrap())).to(
      be_some().value(MatchingRule::Include(s!("A"))));
    expect!(MatchingRule::from_json(&Value::from_str("{\"match\": \"include\"}").unwrap())).to(be_none());

    expect!(MatchingRule::from_json(&Value::from_str("{\"match\": \"type\", \"min\": 1}").unwrap())).to(
      be_some().value(MatchingRule::MinType(1)));
    expect!(MatchingRule::from_json(&Value::from_str("{\"match\": \"type\", \"max\": \"1\"}").unwrap())).to(
      be_some().value(MatchingRule::MaxType(1)));
    expect!(MatchingRule::from_json(&Value::from_str("{\"match\": \"type\", \"min\": 1, \"max\": \"1\"}").unwrap())).to(
      be_some().value(MatchingRule::MinMaxType(1, 1)));
    expect!(MatchingRule::from_json(&Value::from_str("{\"match\": \"type\"}").unwrap())).to(
      be_some().value(MatchingRule::Type));
    expect!(MatchingRule::from_json(&Value::from_str("{\"match\": \"type\", \"value\": 100}").unwrap())).to(
      be_some().value(MatchingRule::Type));
    expect!(MatchingRule::from_json(&Value::from_str("{\"match\": \"min\", \"min\": 1}").unwrap())).to(
      be_some().value(MatchingRule::MinType(1)));
    expect!(MatchingRule::from_json(&Value::from_str("{\"match\": \"max\", \"max\": \"1\"}").unwrap())).to(
      be_some().value(MatchingRule::MaxType(1)));
    expect!(MatchingRule::from_json(&Value::from_str("{\"match\": \"min\"}").unwrap())).to(be_none());
    expect!(MatchingRule::from_json(&Value::from_str("{\"match\": \"max\"}").unwrap())).to(be_none());

    expect!(MatchingRule::from_json(&Value::from_str("{\"match\": \"number\"}").unwrap())).to(
      be_some().value(MatchingRule::Number));
    expect!(MatchingRule::from_json(&Value::from_str("{\"match\": \"integer\"}").unwrap())).to(
      be_some().value(MatchingRule::Integer));
    expect!(MatchingRule::from_json(&Value::from_str("{\"match\": \"decimal\"}").unwrap())).to(
      be_some().value(MatchingRule::Decimal));
    expect!(MatchingRule::from_json(&Value::from_str("{\"match\": \"real\"}").unwrap())).to(
      be_some().value(MatchingRule::Decimal));

    expect!(MatchingRule::from_json(&Value::from_str("{\"match\": \"timestamp\", \"timestamp\": \"A\"}").unwrap())).to(
      be_some().value(MatchingRule::Timestamp(s!("A"))));
    expect!(MatchingRule::from_json(&Value::from_str("{\"match\": \"timestamp\"}").unwrap())).to(be_none());
    expect!(MatchingRule::from_json(&Value::from_str("{\"match\": \"time\", \"time\": \"A\"}").unwrap())).to(
      be_some().value(MatchingRule::Time(s!("A"))));
    expect!(MatchingRule::from_json(&Value::from_str("{\"match\": \"time\"}").unwrap())).to(be_none());
    expect!(MatchingRule::from_json(&Value::from_str("{\"match\": \"date\", \"date\": \"A\"}").unwrap())).to(
      be_some().value(MatchingRule::Date(s!("A"))));
    expect!(MatchingRule::from_json(&Value::from_str("{\"match\": \"date\"}").unwrap())).to(be_none());
  }

  #[test]
  fn matcher_is_defined_returns_false_when_there_are_no_matchers() {
    let matchers = matchingrules!{};
    expect!(matchers.matcher_is_defined("body", &vec![s!("$"), s!("a"), s!("b")])).to(be_false());
  }

  #[test]
  fn matcher_is_defined_returns_false_when_the_path_does_not_have_a_matcher_entry() {
    let matchers = matchingrules!{
            "body" => {
            }
        };
    expect!(matchers.matcher_is_defined("body", &vec![s!("$"), s!("a"), s!("b")])).to(be_false());
  }

  #[test]
  fn matcher_is_defined_returns_true_when_the_path_does_have_a_matcher_entry() {
    let matchers = matchingrules!{
            "body" => {
                "$.a.b" => [ MatchingRule::Type ]
            }
        };
    expect!(matchers.matcher_is_defined("body", &vec![s!("$"), s!("a"), s!("b")])).to(be_true());
  }

  #[test]
  fn matcher_is_defined_returns_true_when_the_parent_of_the_path_does_have_a_matcher_entry() {
    let matchers = matchingrules!{
            "body" => {
                "$.a.b" => [ MatchingRule::Type ]
            }
        };
    expect!(matchers.matcher_is_defined("body", &vec![s!("$"), s!("a"), s!("b"), s!("c")])).to(be_true());
  }

  #[test]
  fn wildcard_matcher_is_defined_returns_false_when_there_are_no_matchers() {
    let matchers = matchingrules!{};
    expect!(matchers.wildcard_matcher_is_defined("body", &vec![s!("$"), s!("a"), s!("b")])).to(be_false());
  }

  #[test]
  fn wildcard_matcher_is_defined_returns_false_when_the_path_does_not_have_a_matcher_entry() {
    let matchers = matchingrules!{
            "body" => {

            }
        };
    expect!(matchers.wildcard_matcher_is_defined("body", &vec![s!("$"), s!("a"), s!("b")])).to(be_false());
  }

  #[test]
  fn wildcard_matcher_is_defined_returns_false_when_the_path_does_have_a_matcher_entry_and_it_is_not_a_wildcard() {
    let matchers = matchingrules!{
            "body" => {
                "$.a.b" => [ MatchingRule::Type ],
                "$.*" => [ MatchingRule::Type ]
            }
        };
    expect!(matchers.wildcard_matcher_is_defined("body", &vec![s!("$"), s!("a"), s!("b")])).to(be_false());
  }

  #[test]
  fn wildcard_matcher_is_defined_returns_true_when_the_path_does_have_a_matcher_entry_and_it_is_a_widcard() {
    let matchers = matchingrules!{
            "body" => {
                "$.a.*" => [ MatchingRule::Type ]
            }
        };
    expect!(matchers.wildcard_matcher_is_defined("body", &vec![s!("$"), s!("a"), s!("b")])).to(be_true());
  }

  #[test]
  fn wildcard_matcher_is_defined_returns_false_when_the_parent_of_the_path_does_have_a_matcher_entry() {
    let matchers = matchingrules!{
            "body" => {
                "$.a.*" => [ MatchingRule::Type ]
            }
        };
    expect!(matchers.wildcard_matcher_is_defined("body", &vec![s!("$"), s!("a"), s!("b"), s!("c")])).to(be_false());
  }

  #[test]
  fn matches_token_test_with_root() {
    expect!(matches_token(&s!("$"), &PathToken::Root)).to(be_equal_to(2));
    expect!(matches_token(&s!("path"), &PathToken::Root)).to(be_equal_to(0));
    expect!(matches_token(&s!("*"), &PathToken::Root)).to(be_equal_to(0));
  }

  #[test]
  fn matches_token_test_with_field() {
    expect!(matches_token(&s!("$"), &PathToken::Field(s!("path")))).to(be_equal_to(0));
    expect!(matches_token(&s!("path"), &PathToken::Field(s!("path")))).to(be_equal_to(2));
  }

  #[test]
  fn matches_token_test_with_index() {
    expect!(matches_token(&s!("$"), &PathToken::Index(2))).to(be_equal_to(0));
    expect!(matches_token(&s!("path"), &PathToken::Index(2))).to(be_equal_to(0));
    expect!(matches_token(&s!("*"), &PathToken::Index(2))).to(be_equal_to(0));
    expect!(matches_token(&s!("1"), &PathToken::Index(2))).to(be_equal_to(0));
    expect!(matches_token(&s!("2"), &PathToken::Index(2))).to(be_equal_to(2));
  }

  #[test]
  fn matches_token_test_with_index_wildcard() {
    expect!(matches_token(&s!("$"), &PathToken::StarIndex)).to(be_equal_to(0));
    expect!(matches_token(&s!("path"), &PathToken::StarIndex)).to(be_equal_to(0));
    expect!(matches_token(&s!("*"), &PathToken::StarIndex)).to(be_equal_to(0));
    expect!(matches_token(&s!("1"), &PathToken::StarIndex)).to(be_equal_to(1));
  }

  #[test]
  fn matches_token_test_with_wildcard() {
    expect!(matches_token(&s!("$"), &PathToken::Star)).to(be_equal_to(1));
    expect!(matches_token(&s!("path"), &PathToken::Star)).to(be_equal_to(1));
    expect!(matches_token(&s!("*"), &PathToken::Star)).to(be_equal_to(1));
    expect!(matches_token(&s!("1"), &PathToken::Star)).to(be_equal_to(1));
  }

  #[test]
  fn matches_path_matches_root_path_element() {
    expect!(calc_path_weight(s!("$"), &vec![s!("$")]) > 0).to(be_true());
    expect!(calc_path_weight(s!("$"), &vec![]) > 0).to(be_false());
  }

  #[test]
  fn matches_path_matches_field_name() {
    expect!(calc_path_weight(s!("$.name"), &vec![s!("$"), s!("name")]) > 0).to(be_true());
    expect!(calc_path_weight(s!("$['name']"), &vec![s!("$"), s!("name")]) > 0).to(be_true());
    expect!(calc_path_weight(s!("$.name.other"), &vec![s!("$"), s!("name"), s!("other")]) > 0).to(be_true());
    expect!(calc_path_weight(s!("$['name'].other"), &vec![s!("$"), s!("name"), s!("other")]) > 0).to(be_true());
    expect!(calc_path_weight(s!("$.name"), &vec![s!("$"), s!("other")]) > 0).to(be_false());
    expect!(calc_path_weight(s!("$.name"), &vec![s!("$"), s!("name"), s!("other")]) > 0).to(be_true());
    expect!(calc_path_weight(s!("$.other"), &vec![s!("$"), s!("name"), s!("other")]) > 0).to(be_false());
    expect!(calc_path_weight(s!("$.name.other"), &vec![s!("$"), s!("name")]) > 0).to(be_false());
  }

  #[test]
  fn matches_path_matches_array_indices() {
    expect!(calc_path_weight(s!("$[0]"), &vec![s!("$"), s!("0")]) > 0).to(be_true());
    expect!(calc_path_weight(s!("$.name[1]"), &vec![s!("$"), s!("name"), s!("1")]) > 0).to(be_true());
    expect!(calc_path_weight(s!("$.name"), &vec![s!("$"), s!("0")]) > 0).to(be_false());
    expect!(calc_path_weight(s!("$.name[1]"), &vec![s!("$"), s!("name"), s!("0")]) > 0).to(be_false());
    expect!(calc_path_weight(s!("$[1].name"), &vec![s!("$"), s!("name"), s!("1")]) > 0).to(be_false());
  }

  #[test]
  fn matches_path_matches_with_wildcard() {
    expect!(calc_path_weight(s!("$[*]"), &vec![s!("$"), s!("0")]) > 0).to(be_true());
    expect!(calc_path_weight(s!("$.*"), &vec![s!("$"), s!("name")]) > 0).to(be_true());
    expect!(calc_path_weight(s!("$.*.name"), &vec![s!("$"), s!("some"), s!("name")]) > 0).to(be_true());
    expect!(calc_path_weight(s!("$.name[*]"), &vec![s!("$"), s!("name"), s!("0")]) > 0).to(be_true());
    expect!(calc_path_weight(s!("$.name[*].name"), &vec![s!("$"), s!("name"), s!("1"), s!("name")]) > 0).to(be_true());
    expect!(calc_path_weight(s!("$[*]"), &vec![s!("$"), s!("name")]) > 0).to(be_false());
  }

  #[test]
  fn min_and_max_values_get_serialised_to_json_as_numbers() {
    expect!(MatchingRule::MinType(1).to_json().to_string()).to(be_equal_to("{\"match\":\"type\",\"min\":1}"));
    expect!(MatchingRule::MaxType(1).to_json().to_string()).to(be_equal_to("{\"match\":\"type\",\"max\":1}"));
    expect!(MatchingRule::MinMaxType(1, 10).to_json().to_string()).to(be_equal_to("{\"match\":\"type\",\"max\":10,\"min\":1}"));
  }

  #[test]
  fn category_from_path() {
    expect!(Category::from_path("")).to(be_none());
    expect!(Category::from_path("$")).to(be_none());
    expect!(Category::from_path("$.body")).to(be_some().value("body"));
    expect!(Category::from_path("$.body.a.b.c")).to(be_some().value("body"));
  }
}
