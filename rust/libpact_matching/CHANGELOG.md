To generate the log, run `git log --pretty='* %h - %s (%an, %ad)' TAGNAME..HEAD .` replacing TAGNAME and HEAD as appropriate.

# 0.2.0 - V2 specification implementation

* 831c771 - remove the example code use as rustdoc chokes on it (Ronald Holshausen, Tue Jul 12 11:20:44 2016 +1000)
* 925eae8 - update the crate docs from the readme (Ronald Holshausen, Tue Jul 12 10:53:23 2016 +1000)
* ea51435 - correct the format of the matcher table in the readme (Ronald Holshausen, Tue Jul 12 10:42:52 2016 +1000)
* 2267844 - update readme with matchers (Ronald Holshausen, Tue Jul 12 10:39:52 2016 +1000)
* f2df5d0 - update the readme for the matching library (Ronald Holshausen, Tue Jul 12 10:27:10 2016 +1000)
* 4841121 - Completed the V2 matching on XML. Now I have a headache (Ronald Holshausen, Mon Jul 11 17:10:36 2016 +1000)
* f1a2448 - added all the remaining V2 XML testcases without matchers (Ronald Holshausen, Mon Jul 11 13:21:11 2016 +1000)
* 068e8f8 - V2 XML testcases fir missing values (Ronald Holshausen, Mon Jul 11 12:06:08 2016 +1000)
* 30d692b - more basic V2 XML testcases (Ronald Holshausen, Mon Jul 11 11:28:54 2016 +1000)
* 59a7598 - V2 XML testcases testing tag order (Ronald Holshausen, Mon Jul 11 10:50:18 2016 +1000)
* 2f0b166 - added XML V2 testcase for missing key and index (Ronald Holshausen, Mon Jul 11 10:25:13 2016 +1000)
* 1501215 - added basic xml V2 testcase (Ronald Holshausen, Mon Jul 11 10:14:11 2016 +1000)
* e3e3de0 - Have sensible defaults if the matcher type attribute is missing (Ronald Holshausen, Fri Jul 8 17:08:31 2016 +1000)
* f7a714e - Implemented V2 min and max type matchers on JSON bodies (Ronald Holshausen, Fri Jul 8 16:23:31 2016 +1000)
* bfabbde - Implemented V2 type and regex matchers on JSON bodies (Ronald Holshausen, Fri Jul 8 11:35:29 2016 +1000)
* 6f6e082 - Implemented V2 type matcher (Ronald Holshausen, Tue Jul 5 16:52:51 2016 +1000)
* abb529a - Implemented V2 regex matcher on query strings and headers (Ronald Holshausen, Tue Jul 5 16:31:06 2016 +1000)
* c7e40f8 - Implemented V2 regex matcher for request paths (Ronald Holshausen, Tue Jul 5 15:17:29 2016 +1000)
* dcefc9d - Implemented the matcher selection algorithm (Ronald Holshausen, Fri Jul 1 16:28:57 2016 +1000)
* 25f9e48 - completed the XML matching (Ronald Holshausen, Wed Jun 29 22:30:21 2016 +1000)
* ab6dca8 - Implemented matching on XML elements (Ronald Holshausen, Wed Jun 29 22:03:34 2016 +1000)
* fe7d358 - completed the tests for comparing a tags attributes (Ronald Holshausen, Wed Jun 29 20:16:23 2016 +1000)
* 089a5e3 - started port of XML matching from JVM version (Ronald Holshausen, Wed Jun 29 17:26:01 2016 +1000)
* 14fdd01 - must be able to handle extended content types like application/thrift+json (Ronald Holshausen, Wed Jun 29 13:47:18 2016 +1000)
* ab43348 - Implemented reading and writing V2 pacts (Ronald Holshausen, Wed Jun 29 13:18:24 2016 +1000)
* b9a1a73 - enable all the V2 spec test cases that pass with the V1.1 implementation (Ronald Holshausen, Wed Jun 29 11:56:59 2016 +1000)
* 3502002 - introduce V2 spec version enum, set it as the default (Ronald Holshausen, Wed Jun 29 11:39:12 2016 +1000)
* 87596c4 - generate tests for all the V2 test cases (marked as ignored for now) (Ronald Holshausen, Wed Jun 29 11:20:11 2016 +1000)
* 3bab4e8 - add all the V2 test cases (Ronald Holshausen, Wed Jun 29 10:54:15 2016 +1000)
* 534e7a1 - updated readmes and bump versions for the V2 implementation (Ronald Holshausen, Wed Jun 29 10:38:32 2016 +1000)
* e3b9497 - bump version to 0.1.1 (Ronald Holshausen, Tue Jun 28 20:47:34 2016 +1000)

# 0.1.0 - V1.1 Specification Implementation

* 140526d - Implement V1.1 matching (Ronald Holshausen, Tue Jun 28 15:58:35 2016 +1000)
* 4224875 - update readmes and bump versions for V1.1 implementation (Ronald Holshausen, Tue Jun 28 15:05:39 2016 +1000)
* b5dc6d2 - added some additional pact loading tests (Ronald Holshausen, Tue Jun 28 14:35:48 2016 +1000)
* 44ec659 - in prep for supporting other spec versions, take the version into account when parsing a pact file (Ronald Holshausen, Tue Jun 28 11:40:07 2016 +1000)
* 91d6d62 - removed the v1 from the project path, will use a git branch instead (Ronald Holshausen, Mon Jun 27 22:09:32 2016 +1000)

# 0.0.1 - Second Feature Release

* 25bf4d0 - added changelog (Ronald Holshausen, Sun Jun 26 15:20:23 2016 +1000)
* 4c60f07 - replace rustful with webmachine (Ronald Holshausen, Thu Jun 16 17:31:11 2016 +1000)
* 7dc4b52 - implemented merging of pact files when writing (Ronald Holshausen, Thu Jun 9 17:34:02 2016 +1000)
* 801f24c - update the github readmes to point to the published rust docs (Ronald Holshausen, Wed Jun 8 10:42:30 2016 +1000)
* ecc4018 - add example pact files for testing (Ronald Holshausen, Wed Jun 8 09:36:35 2016 +1000)
* bbf6fbb - make test not be dependent on the library version (Ronald Holshausen, Wed Jun 1 17:23:02 2016 +1000)
* 937360d - Oops, test generates a pact with the version in the metadata (Ronald Holshausen, Wed Jun 1 17:07:29 2016 +1000)
* e957983 - bump libpact_matching version (Ronald Holshausen, Wed Jun 1 17:00:41 2016 +1000)

# 0.0.0 - First Release
