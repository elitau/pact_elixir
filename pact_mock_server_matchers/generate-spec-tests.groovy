#!/usr/bin/env groovy

import groovy.io.FileType

def tests = new File('tests')
def specs = new File(tests, 'spec_testcases')
specs.eachFileRecurse(FileType.DIRECTORIES) { dir ->
  def path = dir.toPath()
  def testFile = new File(dir, 'mod.rs')
  def requestResponsePath = path.getNameCount() > 3 ? path.getName(3).toString() : ''

  testFile.withPrintWriter { pw ->
    if (requestResponsePath == 'request') {
      pw.println('use pact_mock_server_matchers::model::Request;')
      pw.println('use pact_mock_server_matchers::match_request;')
      pw.println('use rustc_serialize::json;')
    } else if (requestResponsePath == 'response') {
      pw.println('use pact_mock_server_matchers::model::Response;')
      pw.println('use pact_mock_server_matchers::match_response;')
      pw.println('use rustc_serialize::json;')
    }

    dir.eachDir {
      pw.println "mod $it.name;"
    }

    dir.eachFileMatch(~/.*\.json/) {
      def testBody = """
        |#[test]
        |fn ${it.name.replaceAll(' ', '_').replaceAll('\\.json', '')}() {
        |    let pact = json!(
      """
      it.text.eachLine { line ->
        testBody += '|      ' + line + '\n'
      }
      testBody += '|    );' + '\n'
      if (requestResponsePath == 'request') {
        testBody += """
        |    let expected = Request::from_json(&pact.find("expected").unwrap());
        |    println!("{:?}", expected);
        |    let actual = Request::from_json(&pact.find("actual").unwrap());
        |    println!("{:?}", expected);
        |    let comment = "comment"; //pact.find("comment").unwrap().as_string().unwrap();
        |    let pact_match = pact.find("match").unwrap();
        |    if pact_match.as_boolean().unwrap() {
        |       //assert!(match_request(&expected, &actual).is_empty(), comment);
        |    } else {
        |       //assert!(!match_request(&expected, &actual).is_empty(), comment);
        |    }
        """
      } else if (requestResponsePath == 'response') {
        testBody += """
        |    let expected = Response::from_json(&pact.find("expected").unwrap());
        |    println!("{:?}", expected);
        |    let actual = Response::from_json(&pact.find("actual").unwrap());
        |    println!("{:?}", actual);
        |    let comment = "comment"; // pact.find("comment").unwrap().as_string().unwrap();
        |    let pact_match = pact.find("match").unwrap();
        |    if pact_match.as_boolean().unwrap() {
        |       //assert!(match_response(&expected, &actual).is_empty(), comment);
        |    } else {
        |       //assert!(!match_response(&expected, &actual).is_empty(), comment);
        |    }
        """
      }
      testBody += '|}'
      pw.println testBody.stripMargin('|')
    }
  }
}
