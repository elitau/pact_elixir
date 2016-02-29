#!/usr/bin/env groovy

import groovy.io.FileType

def tests = new File('tests')
def specs = new File(tests, 'spec_testcases')
specs.eachFileRecurse(FileType.DIRECTORIES) { dir ->
  def testFile = new File(dir, 'mod.rs')
  testFile.withPrintWriter { pw ->
    dir.eachDir {
      pw.println "mod $it.name;"
    }

    dir.eachFileMatch(~/.*\.json/) {
      pw.println '// ' + it.name
      pw.println """
        |#[test]
        |fn ${it.name.replaceAll(' ', '_').replaceAll('\\.json', '')}() {
        |    assert!(true);
        |}
      """.stripMargin('|')
    }
  }
}
