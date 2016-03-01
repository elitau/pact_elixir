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
      pw.println('use rustc_serialize::json;')
    } else if (requestResponsePath == 'response') {
      pw.println('use pact_mock_server_matchers::model::Response;')
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
        """
      } else if (requestResponsePath == 'response') {
        testBody += """
        |    let expected = Response::from_json(&pact.find("expected").unwrap());
        |    println!("{:?}", expected);
        |    let actual = Response::from_json(&pact.find("actual").unwrap());
        |    println!("{:?}", actual);
        """
      }
      testBody +=
      """|    let pact_match = pact.find("match").unwrap();
         |    assert!(pact_match.as_boolean().unwrap());
         |}
      """
      pw.println testBody.stripMargin('|')
    }
  }
}
