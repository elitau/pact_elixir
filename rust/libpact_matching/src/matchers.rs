use models::Matchers;
use path_exp::*;

fn matches_token(path_fragment: &String, path_token: &PathToken) -> u8 {
    match *path_token {
        PathToken::Root if path_fragment == "$" => 2,
        PathToken::Field(ref name) if *path_fragment == name.clone() => 2,
        PathToken::Index(ref index) => match path_fragment.parse() {
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

fn matches_path(path_exp: String, path: &Vec<String>) -> bool {
    match parse_path_exp(path_exp) {
        Ok(path_tokens) => {
            if path.len() >= path_tokens.len() {
                path_tokens.iter().zip(path.iter())
                    .fold(1, |acc, (token, fragment)| acc * matches_token(fragment, token)) > 0
            } else {
                false
            }
        },
        Err(err) => {
            warn!("Failed to parse path expression - {}", err);
            false
        }
    }
}

fn resolve_matchers(path: &Vec<String>, matchers: &Matchers) -> Matchers {
    matchers.iter().map(|(k, v)| (k.clone(), v.clone()))
        .filter(|kv| matches_path(kv.0.clone(), path)).collect()
}

pub fn matcher_is_defined(path: Vec<String>, matchers: &Option<Matchers>) -> bool {
    match *matchers {
        Some(ref m) => !resolve_matchers(&path, m).is_empty(),
        None => false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::{matches_path, matches_token};
    use expectest::prelude::*;
    use path_exp::*;

    #[test]
    fn matcher_is_defined_returns_false_when_there_are_no_matchers() {
        expect!(matcher_is_defined(vec![s!("$"), s!("a"), s!("b")], &None)).to(be_false());
    }

    #[test]
    fn matcher_is_defined_returns_false_when_the_path_does_not_have_a_matcher_entry() {
        expect!(matcher_is_defined(vec![s!("$"), s!("a"), s!("b")], &Some(hashmap!{}))).to(be_false());
    }

    #[test]
    fn matcher_is_defined_returns_true_when_the_path_does_have_a_matcher_entry() {
        expect!(matcher_is_defined(vec![s!("$"), s!("a"), s!("b")], &Some(hashmap!{
            s!("$.a.b") => hashmap!{}
        }))).to(be_true());
    }

    #[test]
    fn matcher_is_defined_returns_true_when_the_parent_of_the_path_does_have_a_matcher_entry() {
        expect!(matcher_is_defined(vec![s!("$"), s!("a"), s!("b"), s!("c")], &Some(hashmap!{
            s!("$.a.b") => hashmap!{}
        }))).to(be_true());
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
        expect!(matches_path(s!("$"), &vec![s!("$")])).to(be_true());
        expect!(matches_path(s!("$"), &vec![])).to(be_false());
    }

    #[test]
    fn matches_path_matches_field_name() {
        expect!(matches_path(s!("$.name"), &vec![s!("$"), s!("name")])).to(be_true());
        expect!(matches_path(s!("$['name']"), &vec![s!("$"), s!("name")])).to(be_true());
        expect!(matches_path(s!("$.name.other"), &vec![s!("$"), s!("name"), s!("other")])).to(be_true());
        expect!(matches_path(s!("$['name'].other"), &vec![s!("$"), s!("name"), s!("other")])).to(be_true());
        expect!(matches_path(s!("$.name"), &vec![s!("$"), s!("other")])).to(be_false());
        expect!(matches_path(s!("$.name"), &vec![s!("$"), s!("name"), s!("other")])).to(be_true());
        expect!(matches_path(s!("$.other"), &vec![s!("$"), s!("name"), s!("other")])).to(be_false());
        expect!(matches_path(s!("$.name.other"), &vec![s!("$"), s!("name")])).to(be_false());
    }

    #[test]
    fn matches_path_matches_array_indices() {
        expect!(matches_path(s!("$[0]"), &vec![s!("$"), s!("0")])).to(be_true());
        expect!(matches_path(s!("$.name[1]"), &vec![s!("$"), s!("name"), s!("1")])).to(be_true());
        expect!(matches_path(s!("$.name"), &vec![s!("$"), s!("0")])).to(be_false());
        expect!(matches_path(s!("$.name[1]"), &vec![s!("$"), s!("name"), s!("0")])).to(be_false());
        expect!(matches_path(s!("$[1].name"), &vec![s!("$"), s!("name"), s!("1")])).to(be_false());
    }

    #[test]
    fn matches_path_matches_with_wildcard() {
        expect!(matches_path(s!("$[*]"), &vec![s!("$"), s!("0")])).to(be_true());
        expect!(matches_path(s!("$.*"), &vec![s!("$"), s!("name")])).to(be_true());
        expect!(matches_path(s!("$.*.name"), &vec![s!("$"), s!("some"), s!("name")])).to(be_true());
        expect!(matches_path(s!("$.name[*]"), &vec![s!("$"), s!("name"), s!("0")])).to(be_true());
        expect!(matches_path(s!("$.name[*].name"), &vec![s!("$"), s!("name"), s!("1"), s!("name")])).to(be_true());
        expect!(matches_path(s!("$[*]"), &vec![s!("$"), s!("name")])).to(be_false());
    }

}
