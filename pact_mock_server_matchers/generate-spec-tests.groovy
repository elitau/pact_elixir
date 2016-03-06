#!/usr/bin/env groovy

import groovy.io.FileType
import groovy.json.JsonSlurper

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
      pw.println('use rustc_serialize::json::Json;')
    } else if (requestResponsePath == 'response') {
      pw.println('use pact_mock_server_matchers::model::Response;')
      pw.println('use pact_mock_server_matchers::match_response;')
      pw.println('use rustc_serialize::json::Json;')
    }

    dir.eachDir {
      pw.println "mod $it.name;"
    }

    dir.eachFileMatch(~/.*\.json/) {
      def json = new JsonSlurper().parse(it)
      def testBody = """
        |#[test]
        |fn ${it.name.replaceAll(' ', '_').replaceAll('\\.json', '')}() {
        |    let pact = Json::from_str(r#"
      """
      it.text.eachLine { line ->
        testBody += '|      ' + line + '\n'
      }
      testBody += '|    "#).unwrap();' + '\n'
      if (requestResponsePath == 'request') {
        testBody += """
        |    let expected = Request::from_json(&pact.find("expected").unwrap());
        |    println!("{:?}", expected);
        |    let actual = Request::from_json(&pact.find("actual").unwrap());
        |    println!("{:?}", actual);
        |    let pact_match = pact.find("match").unwrap();
        |    if pact_match.as_boolean().unwrap() {
        |       assert!(match_request(expected, actual).is_empty(), "${json.comment}");
        |    } else {
        |       assert!(!match_request(expected, actual).is_empty(), "${json.comment}");
        |    }
        """
      } else if (requestResponsePath == 'response') {
        testBody += """
        |    let expected = Response::from_json(&pact.find("expected").unwrap());
        |    println!("{:?}", expected);
        |    let actual = Response::from_json(&pact.find("actual").unwrap());
        |    println!("{:?}", actual);
        |    let pact_match = pact.find("match").unwrap();
        |    if pact_match.as_boolean().unwrap() {
        |       assert!(match_response(expected, actual).is_empty(), "${json.comment}");
        |    } else {
        |       assert!(!match_response(expected, actual).is_empty(), "${json.comment}");
        |    }
        """
      }
      testBody += '|}'
      pw.println testBody.stripMargin('|')
    }
  }
}
