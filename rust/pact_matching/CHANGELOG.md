To generate the log, run `git log --pretty='* %h - %s (%an, %ad)' TAGNAME..HEAD .` replacing TAGNAME and HEAD as appropriate.

# 0.2.2 - Bugfix Release

* 3399f7c - Merge pull request #13 from adamtsis/remove-deprecated-json-lib (Ronald Holshausen, Tue May 16 15:56:22 2017 +1000)
* efe4ca7 - Cleanup unused imports and unreachable pattern warning messages (Anthony Damtsis, Tue May 16 10:31:29 2017 +1000)
* a59fb98 - Migrate remaining pact modules over to serde (Anthony Damtsis, Mon May 15 16:59:04 2017 +1000)
* 142d550 - Merge pull request #12 from adamtsis/remove-deprecated-json-lib (Ronald Holshausen, Mon May 8 16:13:30 2017 +1000)
* ff1b676 - Change spec test generator template to use serde library (Anthony Damtsis, Mon May 8 12:23:28 2017 +1000)
* cdecc71 - Simplify json handling logic when running comparisons (Anthony Damtsis, Fri May 5 15:48:17 2017 +1000)
* f725ddc - Remove commented crate (Anthony Damtsis, Fri May 5 15:39:27 2017 +1000)
* bd6fa9b - Fixed remaining serialisation bugs with writing pact files (Anthony Damtsis, Fri May 5 15:27:59 2017 +1000)
* d1bd5ef - Changed type parameter to be a Hashmap when deserialising payload body (Anthony Damtsis, Fri May 5 14:09:54 2017 +1000)
* 83a8b7e - Fix incorrectly deserialised json objects in tests (Anthony Damtsis, Fri May 5 13:23:03 2017 +1000)
* 21cb633 - Compiles now - lots of test failures to work through (Anthony Damtsis, Fri May 5 13:27:36 2017 +1000)
* 1e8531b - Begun work to replace rustc_serialize - work in progress (Anthony Damtsis, Mon May 1 14:52:08 2017 +1000)
* 7982137 - Merge pull request #11 from adamtsis/camel-case-specification (Ronald Holshausen, Mon May 1 13:49:09 2017 +1000)
* 9a29085 - Supports camel case format pact specifications (Anthony Damtsis, Thu Apr 27 15:03:15 2017 +1000)
* 4dabb31 - Simplify call to HeaderMismatch destructure (Anthony Damtsis, Wed Apr 26 20:48:32 2017 +1000)
* 381a85e - Explicitly clone the borrowed header reference (Anthony Damtsis, Wed Apr 26 18:25:04 2017 +1000)
* 755ada8 - Fixed compiler warning messages (Anthony Damtsis, Wed Apr 26 18:12:55 2017 +1000)
* a2847c6 - Replace .to_string() refs with s!() macro (Anthony Damtsis, Wed Apr 26 17:28:19 2017 +1000)
* c9eff21 - Support optional header parameters for 'accept' and 'content-type' (Anthony Damtsis, Wed Apr 26 15:36:08 2017 +1000)
* 26f91a5 - Ensure mismatch for HeaderMismatch is consistent for EqualityMatcher (Anthony Damtsis, Wed Apr 26 15:32:30 2017 +1000)
* a501309 - bump version to 0.2.2 (Ronald Holshausen, Sun Oct 9 16:14:45 2016 +1100)
* 227b61b - correct the doc URL in the cargo manifest (Ronald Holshausen, Sun Oct 9 16:13:27 2016 +1100)
* 5233cfa - correct updating the documentation URL in the release script (Ronald Holshausen, Sun Oct 9 16:08:33 2016 +1100)

# 0.2.1 - Changes required for verifying V2 pacts

* 574e072 - upadte versions for V2 branch and fix an issue with loading JSON bodies encoded as a string (Ronald Holshausen, Sun Oct 9 15:31:57 2016 +1100)
* a21973a - Get the build passing after merge from V1.1 branch (Ronald Holshausen, Sun Oct 9 13:47:09 2016 +1100)
* 341607c - Merge branch 'v1.1-spec' into v2-spec (Ronald Holshausen, Sun Oct 9 12:10:12 2016 +1100)
* 797c9b9 - correct the URLs to the repos (Ronald Holshausen, Sat Oct 8 17:10:56 2016 +1100)
* b7e038e - bump version to 0.1.2 (Ronald Holshausen, Sat Oct 8 16:54:52 2016 +1100)

# 0.1.1 - Changes required for verifying V1.1 pacts

* 373f82d - regenerated the specification tests (Ronald Holshausen, Sat Oct 8 16:50:38 2016 +1100)
* 388a19f - update references (Ronald Holshausen, Sat Oct 8 16:46:11 2016 +1100)
* a46dabb - update all references to V1 spec after merge (Ronald Holshausen, Sat Oct 8 16:20:51 2016 +1100)
* 63ae7e4 - get project compiling after merge from V1 branch (Ronald Holshausen, Sat Oct 8 15:53:22 2016 +1100)
* 1d6d4f8 - Merge branch 'v1-spec' into v1.1-spec (Ronald Holshausen, Sat Oct 8 15:44:25 2016 +1100)
* 04d9e5f - update the docs for the pact consumer library (Ronald Holshausen, Mon Sep 26 23:06:19 2016 +1000)
* 7dd04e6 - update the release scripts to point the docs to docs.rs (Ronald Holshausen, Mon Sep 26 21:49:35 2016 +1000)
* d7c859c - bump version to 0.0.3 (Ronald Holshausen, Mon Sep 26 20:55:12 2016 +1000)
* 02421d5 - exclude IntelliJ files from packaging (Ronald Holshausen, Mon Sep 26 20:46:47 2016 +1000)

# 0.1.0 - V1.1 Specification Implementation

* 140526d - Implement V1.1 matching (Ronald Holshausen, Tue Jun 28 15:58:35 2016 +1000)
* 4224875 - update readmes and bump versions for V1.1 implementation (Ronald Holshausen, Tue Jun 28 15:05:39 2016 +1000)
* b5dc6d2 - added some additional pact loading tests (Ronald Holshausen, Tue Jun 28 14:35:48 2016 +1000)
* 44ec659 - in prep for supporting other spec versions, take the version into account when parsing a pact file (Ronald Holshausen, Tue Jun 28 11:40:07 2016 +1000)
* 91d6d62 - removed the v1 from the project path, will use a git branch instead (Ronald Holshausen, Mon Jun 27 22:09:32 2016 +1000)

# 0.0.2 - Fixes required for verifying pacts

* 429ef78 - Implemented handling state change requests in the pact verifier (Ronald Holshausen, Sun Sep 25 15:55:18 2016 +1000)
* cc1e359 - implemented rudimentary diff output on json bodies (Ronald Holshausen, Sun Sep 25 13:43:45 2016 +1000)
* cd367e6 - Use a regex to detect the content type to handle extended types (e.g application/hal+json) (Ronald Holshausen, Sat Sep 24 17:14:16 2016 +1000)
* 0d69675 - Implemented pact test where there are no pacts in the pact broker (Ronald Holshausen, Sun Sep 18 17:41:51 2016 +1000)
* bc3405c - implemented handling templated HAL URLs (Ronald Holshausen, Sun Sep 18 13:58:54 2016 +1000)
* c3a8a30 - renamed the pact_matching and pact_mock_server directories (Ronald Holshausen, Sun Sep 18 11:07:32 2016 +1000)

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
