use super::Mismatch;
use super::DiffConfig;
use sxd_document::*;
use sxd_document::dom::*;
use std::collections::btree_map::BTreeMap;

pub fn match_xml(expected: &String, actual: &String, config: DiffConfig, mismatches: &mut Vec<super::Mismatch>) {
    let expected_result = parser::parse(expected);
    let actual_result = parser::parse(actual);

    if expected_result.is_err() || actual_result.is_err() {
        match expected_result {
            Err(e) => {
                mismatches.push(Mismatch::BodyMismatch { path: s!("$.body"), expected: Some(expected.clone()),
                    actual: Some(actual.clone()),
                    mismatch: format!("Failed to parse the expected body: '{:?}'", e)});
            },
            _ => ()
        }
        match actual_result {
            Err(e) => {
                mismatches.push(Mismatch::BodyMismatch { path: s!("$.body"), expected: Some(expected.clone()),
                    actual: Some(actual.clone()),
                    mismatch: format!("Failed to parse the actual body: '{:?}'", e)});
            },
            _ => ()
        }
    } else {
        let expected_package = expected_result.unwrap();
        let expected_root = expected_package.as_document().root();
        let expected_root_node = expected_root.children().iter().cloned().find(|n| n.element().is_some());
        let actual_package = actual_result.unwrap();
        let actual_root = actual_package.as_document().root();
        let actual_root_node = actual_root.children().iter().cloned().find(|n| n.element().is_some());
        compare_element(vec!["$", "body"], &expected_root_node.unwrap().element().unwrap(),
            &actual_root_node.unwrap().element().unwrap(), config, mismatches);
    }
}

fn compare_element(path: Vec<&str>, expected: &Element, actual: &Element, config: DiffConfig,
    mismatches: &mut Vec<super::Mismatch>) {
    if actual.name() != expected.name() {
      mismatches.push(Mismatch::BodyMismatch { path: path.join("."), expected: Some(s!(expected.name().local_part())),
          actual: Some(s!(actual.name().local_part())),
          mismatch: format!("Expected element '{}' but received '{}'", expected.name().local_part(), actual.name().local_part())});
    } else {
        let mut new_path = path.to_vec();
        new_path.push(actual.name().local_part());
        compare_attributes(new_path, expected, actual, config, mismatches);
    //   compareAttributes(newPath,expected,actual,config,matchers) ++ compareChildren(newPath,expected,actual,config,matchers)
    }
}

fn compare_attributes(path: Vec<&str>, expected: &Element, actual: &Element, config: DiffConfig,
    mismatches: &mut Vec<super::Mismatch>) {
    let expected_attributes: BTreeMap<String, String> = expected.attributes()
        .iter().map(|attr| (s!(attr.name().local_part()), s!(attr.value()))).collect();
    let actual_attributes: BTreeMap<String, String> = actual.attributes()
        .iter().map(|attr| (s!(attr.name().local_part()), s!(attr.value()))).collect();
    if expected_attributes.is_empty() && !actual_attributes.is_empty() {
      mismatches.push(Mismatch::BodyMismatch { path: path.join("."),
          expected: Some(format!("{:?}", expected_attributes)),
          actual: Some(format!("{:?}", actual_attributes)),
          mismatch: format!("Did not expect any attributes but received {:?}", actual_attributes)});
    } else {
        match config {
            DiffConfig::AllowUnexpectedKeys if expected_attributes.len() > actual_attributes.len() => {
                mismatches.push(Mismatch::BodyMismatch { path: path.join("."),
                    expected: Some(format!("{:?}", expected_attributes)),
                    actual: Some(format!("{:?}", actual_attributes)),
                    mismatch: format!("Expected at least {} attribute(s) but received {} attribute(s)",
                    expected_attributes.len(), actual_attributes.len())});
            },
            DiffConfig::NoUnexpectedKeys if expected_attributes.len() != actual_attributes.len() => {
                mismatches.push(Mismatch::BodyMismatch { path: path.join("."),
                    expected: Some(format!("{:?}", expected_attributes)),
                    actual: Some(format!("{:?}", actual_attributes)),
                    mismatch: format!("Expected {} attribute(s) but received {} attribute(s)",
                    expected_attributes.len(), actual_attributes.len())});
            },
            _ => ()
        }

        for (key, value) in expected_attributes.iter() {
            if actual_attributes.contains_key(key) {
                let mut p = path.to_vec();
                p.push(&key);
                compare_value(p, value, &actual_attributes[key], mismatches);
            } else {
                mismatches.push(Mismatch::BodyMismatch { path: path.join("."),
                    expected: Some(format!("{:?}", expected_attributes)),
                    actual: Some(format!("{:?}", actual_attributes)),
                    mismatch: format!("Expected attribute '{}'='{}' but was missing", key, value)});
            }
        }
    }
}

fn compare_value(path: Vec<&str>, expected: &String, actual: &String, mismatches: &mut Vec<super::Mismatch>) {
    if expected != actual {
        mismatches.push(Mismatch::BodyMismatch { path: path.join("."),
            expected: Some(expected.clone()),
            actual: Some(actual.clone()),
            mismatch: format!("Expected '{}' but received '{}'", expected, actual)});
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use expectest::prelude::*;
    use Mismatch;
    use DiffConfig;

    #[test]
    fn match_xml_handles_empty_strings() {
        let mut mismatches = vec![];
        let expected = s!("");
        let actual = s!("");
        match_xml(&expected, &actual, DiffConfig::AllowUnexpectedKeys, &mut mismatches);
        expect!(mismatches.iter()).to(have_count(2));
        let mismatch = mismatches[0].clone();
        expect!(&mismatch).to(be_equal_to(&Mismatch::BodyMismatch { path: s!("$.body"), expected: Some(expected),
            actual: Some(actual), mismatch: s!("")}));
    }

    #[test]
    fn match_xml_handles_invalid_expected_xml() {
        let mut mismatches = vec![];
        let expected = s!(r#"<xml-is-bad"#);
        let actual = s!(r#"<?xml version="1.0" encoding="UTF-8"?> <blah/>"#);
        match_xml(&expected, &actual, DiffConfig::AllowUnexpectedKeys, &mut mismatches);
        expect!(mismatches.iter()).to(have_count(1));
        let mismatch = mismatches[0].clone();
        expect!(&mismatch).to(be_equal_to(&Mismatch::BodyMismatch { path: s!("$.body"), expected: Some(expected),
            actual: Some(actual), mismatch: s!("")}));
    }

    #[test]
    fn match_xml_handles_invalid_actual_json() {
        let mut mismatches = vec![];
        let expected = s!(r#"<?xml version="1.0" encoding="UTF-8"?> <blah/>"#);
        let actual = s!(r#"{json: "is bad"}"#);
        match_xml(&expected, &actual, DiffConfig::AllowUnexpectedKeys, &mut mismatches);
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
    fn match_xml_with_equal_bodies() {
        let mut mismatches = vec![];
        let expected = s!(r#"<?xml version="1.0" encoding="UTF-8"?> <blah/>"#);
        let actual = s!(r#"<?xml version="1.0" encoding="UTF-8"?> <blah/>"#);
        match_xml(&expected, &actual, DiffConfig::AllowUnexpectedKeys, &mut mismatches);
        expect!(mismatches).to(be_empty());
    }

    #[test]
    fn match_xml_when_bodies_differ_only_in_whitespace() {
        let mut mismatches = vec![];
        let expected = s!(r#"<?xml version="1.0" encoding="UTF-8"?>
        <foo>
            <bar></bar>
        </foo>
        "#);
        let actual = s!(r#"<?xml version="1.0" encoding="UTF-8"?>
        <foo><bar></bar></foo>
        "#);
        match_xml(&expected, &actual, DiffConfig::AllowUnexpectedKeys, &mut mismatches);
        expect!(mismatches).to(be_empty());
    }

    #[test]
    fn match_xml_when_actual_has_different_root() {
        let mut mismatches = vec![];
        let expected = s!(r#"<?xml version="1.0" encoding="UTF-8"?>
        <foo/>
        "#);
        let actual = s!(r#"<?xml version="1.0" encoding="UTF-8"?>
        <bar/>
        "#);
        match_xml(&expected, &actual, DiffConfig::AllowUnexpectedKeys, &mut mismatches);
        expect!(mismatches.iter()).to(have_count(1));
        let mismatch = mismatches[0].clone();
        expect!(&mismatch).to(be_equal_to(&Mismatch::BodyMismatch { path: s!("$.body"), expected: Some(s!("foo")),
            actual: Some(s!("bar")), mismatch: s!("")}));
        expect!(mismatch_message(&mismatch)).to(be_equal_to(s!("Expected element 'foo' but received 'bar'")));
    }

    #[test]
    fn match_xml_with_equal_attributes() {
        let mut mismatches = vec![];
        let expected = s!(r#"<?xml version="1.0" encoding="UTF-8"?>
        <blah a="b" c="d"/>
        "#);
        let actual = s!(r#"<?xml version="1.0" encoding="UTF-8"?>
        <blah a="b" c="d"/>
        "#);
        match_xml(&expected, &actual, DiffConfig::AllowUnexpectedKeys, &mut mismatches);
        expect!(mismatches).to(be_empty());
    }

    #[test]
    fn match_xml_with_nonequal_attributes() {
        let mut mismatches = vec![];
        let expected = s!(r#"<?xml version="1.0" encoding="UTF-8"?>
        <blah a="c" c="b"/>
        "#);
        let actual = s!(r#"<?xml version="1.0" encoding="UTF-8"?>
        <blah a="b"/>
        "#);
        match_xml(&expected, &actual, DiffConfig::AllowUnexpectedKeys, &mut mismatches);
        expect!(mismatches.iter()).to(have_count(3));
        let mismatch = mismatches[0].clone();
        expect!(&mismatch).to(be_equal_to(&Mismatch::BodyMismatch { path: s!("$.body.blah"),
            expected: Some(s!("{\"a\": \"c\", \"c\": \"b\"}")),
            actual: Some(s!("{\"a\": \"b\"}")), mismatch: s!("")}));
        expect!(mismatch_message(&mismatch)).to(be_equal_to(s!("Expected at least 2 attribute(s) but received 1 attribute(s)")));
        let mismatch = mismatches[1].clone();
        expect!(&mismatch).to(be_equal_to(&Mismatch::BodyMismatch { path: s!("$.body.blah.a"), expected: Some(s!("c")),
            actual: Some(s!("b")), mismatch: s!("")}));
        expect!(mismatch_message(&mismatch)).to(be_equal_to(s!("Expected 'c' but received 'b'")));
        let mismatch = mismatches[2].clone();
        expect!(&mismatch).to(be_equal_to(&Mismatch::BodyMismatch { path: s!("$.body.blah"), expected: Some(s!("{\"a\": \"c\", \"c\": \"b\"}")),
            actual: Some(s!("{\"a\": \"b\"}")), mismatch: s!("")}));
        expect!(mismatch_message(&mismatch)).to(be_equal_to(s!("Expected attribute \'c\'=\'b\' but was missing")));
    }

    #[test]
    fn match_xml_with_when_not_expecting_attributes() {
        let mut mismatches = vec![];
        let expected = s!(r#"<?xml version="1.0" encoding="UTF-8"?>
        <blah/>
        "#);
        let actual = s!(r#"<?xml version="1.0" encoding="UTF-8"?>
        <blah a="b" c="d"/>
        "#);
        match_xml(&expected, &actual, DiffConfig::AllowUnexpectedKeys, &mut mismatches);
        expect!(mismatches.iter()).to(have_count(1));
        let mismatch = mismatches[0].clone();
        expect!(&mismatch).to(be_equal_to(&Mismatch::BodyMismatch { path: s!("$.body.blah"), expected: Some(s!("{}")),
            actual: Some(s!("{\"a\": \"b\", \"c\": \"d\"}")), mismatch: s!("")}));
        expect!(mismatch_message(&mismatch)).to(be_equal_to(s!("Did not expect any attributes but received {\"a\": \"b\", \"c\": \"d\"}")));
    }

    // #[test]
    // fn match_xml_when_actual_is_non_empty_and_we_do_not_allow_extra_keys() {
    //     let mut mismatches = vec![];
    //     let expected = s!(r#"<?xml version="1.0" encoding="UTF-8"?>
    //     <!-- TEST -->
    //     "#);
    //     let actual = s!(r#"<?xml version="1.0" encoding="UTF-8"?>
    //     <foo><bar></bar></foo>
    //     "#);
    //     match_xml(&expected, &actual, DiffConfig::NoUnexpectedKeys, &mut mismatches);
    //     expect!(mismatches.clone()).to_not(be_empty());
    //     let mismatch = mismatches[0].clone();
    //     expect!(&mismatch).to(be_equal_to(&Mismatch::BodyMismatch { path: s!("$.body"), expected: Some(expected),
    //         actual: Some(actual), mismatch: s!("")}));
    //     expect!(mismatch_message(&mismatch)).to(be_equal_to(s!("Type mismatch: Expected List [{}] but received Map {}")));
    // }

    /*

     "when allowUnexpectedKeys is true" in {

       val allowUnexpectedKeys = DiffConfig(structural = true, allowUnexpectedKeys = true)

       "and comparing an empty list to a non-empty one" in {
         expectedBody = OptionalBody.body("<foo></foo>")
         actualBody = OptionalBody.body("<foo><item/></foo>")
         matcher.matchBody(expected(), actual(), allowUnexpectedKeys) must beEmpty
       }

       "and comparing a list to a super-set" in {
         expectedBody = OptionalBody.body("<foo><item1/></foo>")
         actualBody = OptionalBody.body("<foo><item1/><item2/></foo>")
         matcher.matchBody(expected(), actual(), allowUnexpectedKeys) must beEmpty
       }

       "and comparing a tags attributes to one with more entries" in {
         expectedBody = OptionalBody.body("<foo something=\"100\"/>")
         actualBody = OptionalBody.body("<foo something=\"100\" somethingElse=\"101\"/>")
         matcher.matchBody(expected(), actual(), allowUnexpectedKeys) must beEmpty
       }

     }

   }

   "returns a mismatch" should {

     def containMessage(s: String) = (a: List[BodyMismatch]) => (
         a.exists((m: BodyMismatch) => m.mismatch.get == s),
         s"$a does not contain '$s'"
       )

     def havePath(p: String) = (a: List[BodyMismatch]) => (
       a.forall((m: BodyMismatch) => m.path == p),
       s"$a does not have path '$p', paths are: ${a.map(m => m.path).mkString(",")}"
       )


     "when comparing anything to an empty body" in {
       expectedBody = OptionalBody.body(<blah/>.toString())
       matcher.matchBody(expected(), actual(), diffconfig) must not(beEmpty)
     }

     "when the root elements do not match" in {
       expectedBody = OptionalBody.body("<foo/>")
       actualBody = OptionalBody.body("<bar></bar>")
       val mismatches: List[BodyMismatch] = matcher.matchBody(expected(), actual(), diffconfig)
       mismatches must not(beEmpty)
       mismatches must containMessage("Expected element foo but received bar")
       mismatches must havePath("$.body")
     }

     "when comparing an empty list to a non-empty one" in {
       expectedBody = OptionalBody.body("<foo></foo>")
       actualBody = OptionalBody.body("<foo><item/></foo>")
       val mismatches: List[BodyMismatch] = matcher.matchBody(expected(), actual(), diffconfig)
       mismatches must not(beEmpty)
       mismatches must containMessage("Expected an empty List but received <item/>")
       mismatches must havePath("$.body.foo")
     }

     "when comparing a list to one with with different size" in {
       expectedBody = OptionalBody.body("<foo><one/><two/><three/><four/></foo>")
       actualBody = OptionalBody.body("<foo><one/><two/><three/></foo>")
       val mismatches = matcher.matchBody(expected(), actual(), diffconfig)
       mismatches must not(beEmpty)
       mismatches must have size 2
       mismatches must containMessage("Expected a List with 4 elements but received 3 elements")
       mismatches must containMessage("Expected <four/> but was missing")
       mismatches must havePath("$.body.foo")
     }

     "when comparing a list to one with with the same size but different children" in {
       expectedBody = OptionalBody.body("<foo><one/><two/><three/></foo>")
       actualBody = OptionalBody.body("<foo><one/><two/><four/></foo>")
       val mismatches = matcher.matchBody(expected(), actual(), diffconfig)

       mismatches must containMessage("Expected element three but received four")
       mismatches must havePath("$.body.foo[2]")
     }

     "when comparing a list to one where the items are in the wrong order" in {
       expectedBody = OptionalBody.body("<foo><one/><two/><three/></foo>")
       actualBody = OptionalBody.body("<foo><one/><three/><two/></foo>")
       val mismatches = matcher.matchBody(expected(), actual(), diffconfig)

       mismatches must containMessage("Expected element two but received three")
       mismatches must containMessage("Expected element three but received two")
     }

     "when comparing a tags attributes to one with less entries" in {
       expectedBody = OptionalBody.body("<foo something=\"100\" somethingElse=\"101\"/>")
       actualBody = OptionalBody.body("<foo something=\"100\"/>")
       val mismatches = matcher.matchBody(expected(), actual(), diffconfig)
       mismatches must not(beEmpty)
       mismatches must containMessage("Expected a Tag with at least 2 attributes but received 1 attributes")
     }

     "when comparing a tags attributes to one with more entries" in {
       expectedBody = OptionalBody.body("<foo something=\"100\"/>")
       actualBody = OptionalBody.body("<foo something=\"100\" somethingElse=\"101\"/>")
       val mismatches = matcher.matchBody(expected(), actual(), diffconfig)
       mismatches must not(beEmpty)
       mismatches must containMessage("Expected a Tag with at least 1 attributes but received 2 attributes")
     }

     "when a tag is missing an attribute" in {
       expectedBody = OptionalBody.body("<foo something=\"100\" somethingElse=\"100\"/>")
       actualBody = OptionalBody.body("<foo something=\"100\"/>")
       val mismatches = matcher.matchBody(expected(), actual(), diffconfig)
       mismatches must not(beEmpty)
       mismatches must containMessage("Expected somethingElse=100 but was missing")
     }

     "when a tag has the same number of attributes but different keys" in {
       expectedBody = OptionalBody.body("<foo something=\"100\" somethingElse=\"100\"/>")
       actualBody = OptionalBody.body("<foo something=\"100\" somethingDifferent=\"100\"/>")
       val mismatches = matcher.matchBody(expected(), actual(), diffconfig)
       mismatches must not(beEmpty)
       mismatches must containMessage("Expected somethingElse=100 but was missing")
       mismatches must havePath("$.body.foo.somethingElse")
     }

     "when a tag has an invalid value" in {
       expectedBody = OptionalBody.body("<foo something=\"100\"/>")
       actualBody = OptionalBody.body("<foo something=\"101\"/>")
       val mismatches = matcher.matchBody(expected(), actual(), diffconfig)
       mismatches must not(beEmpty)
       mismatches must containMessage("Expected something=100 but received 101")
       mismatches must havePath("$.body.foo.something")
     }

     "when the content of an element does not match" in {
       expectedBody = OptionalBody.body("<foo>hello world</foo>")
       actualBody = OptionalBody.body("<foo>hello my friend</foo>")
       val mismatches = matcher.matchBody(expected(), actual(), diffconfig)
       mismatches must not(beEmpty)
       mismatches must containMessage("Expected value 'hello world' but received 'hello my friend'")
       mismatches must havePath("$.body.foo[0]")
     }
   }

   "with a matcher defined" should {

     "delegate to the matcher" in {
       expectedBody = OptionalBody.body("<foo something=\"100\"/>")
       actualBody = OptionalBody.body("<foo something=\"101\"/>")
       matchers = Map("$.body.foo.something" -> Map("regex" -> "\\d+"))
       matcher.matchBody(expected(), actual(), diffconfig) must beEmpty
     }
    */

}
