To generate the log, run `git log --pretty='* %h - %s (%an, %ad)' TAGNAME..HEAD .` replacing TAGNAME and HEAD as appropriate.

# 0.2.2 - Bugfix Release

* be8c299 - Cleanup unused BTreeMap usages and use remote pact dependencies (Anthony Damtsis, Mon May 15 17:09:14 2017 +1000)
* a59fb98 - Migrate remaining pact modules over to serde (Anthony Damtsis, Mon May 15 16:59:04 2017 +1000)
* 84867ac - bump version to 0.2.2 (Ronald Holshausen, Sun Oct 9 16:31:07 2016 +1100)

# 0.2.1 - Changes required for verifying V2 pacts

* 770010a - update projects to use the published pact matching lib (Ronald Holshausen, Sun Oct 9 16:25:15 2016 +1100)
* 574e072 - upadte versions for V2 branch and fix an issue with loading JSON bodies encoded as a string (Ronald Holshausen, Sun Oct 9 15:31:57 2016 +1100)
* a21973a - Get the build passing after merge from V1.1 branch (Ronald Holshausen, Sun Oct 9 13:47:09 2016 +1100)
* 341607c - Merge branch 'v1.1-spec' into v2-spec (Ronald Holshausen, Sun Oct 9 12:10:12 2016 +1100)
* 797c9b9 - correct the URLs to the repos (Ronald Holshausen, Sat Oct 8 17:10:56 2016 +1100)
* ca29349 - bump version to 0.1.2 (Ronald Holshausen, Sat Oct 8 17:09:57 2016 +1100)

# 0.1.1 - Changes required for verifying V1.1 pacts

* a54abd7 - update the dependencies (Ronald Holshausen, Sat Oct 8 17:01:35 2016 +1100)
* a46dabb - update all references to V1 spec after merge (Ronald Holshausen, Sat Oct 8 16:20:51 2016 +1100)
* 63ae7e4 - get project compiling after merge from V1 branch (Ronald Holshausen, Sat Oct 8 15:53:22 2016 +1100)
* 1d6d4f8 - Merge branch 'v1-spec' into v1.1-spec (Ronald Holshausen, Sat Oct 8 15:44:25 2016 +1100)
* 04d9e5f - update the docs for the pact consumer library (Ronald Holshausen, Mon Sep 26 23:06:19 2016 +1000)
* 7dd04e6 - update the release scripts to point the docs to docs.rs (Ronald Holshausen, Mon Sep 26 21:49:35 2016 +1000)
* d8ef338 - bump version to 0.0.3 (Ronald Holshausen, Mon Sep 26 21:48:37 2016 +1000)

# 0.2.0 - V2 specification implementation

* ea9644d - added some V2 matcher tests (Ronald Holshausen, Wed Jul 13 13:35:24 2016 +1000)
* 0e75490 - link to 0.2.0 of the matching library and updated the rust docs (Ronald Holshausen, Tue Jul 12 14:10:02 2016 +1000)
* 534e7a1 - updated readmes and bump versions for the V2 implementation (Ronald Holshausen, Wed Jun 29 10:38:32 2016 +1000)
* f235684 - bump version to 0.1.1 (Ronald Holshausen, Tue Jun 28 21:25:58 2016 +1000)

# 0.1.0 - V1.1 Specification Implementation

* 1e7ab5a - use the V1.1 matching library (Ronald Holshausen, Tue Jun 28 21:17:01 2016 +1000)
* 140526d - Implement V1.1 matching (Ronald Holshausen, Tue Jun 28 15:58:35 2016 +1000)
* 4224875 - update readmes and bump versions for V1.1 implementation (Ronald Holshausen, Tue Jun 28 15:05:39 2016 +1000)
* 91d6d62 - removed the v1 from the project path, will use a git branch instead (Ronald Holshausen, Mon Jun 27 22:09:32 2016 +1000)

# 0.0.2 - Fixes required for verifing pacts

* a0954f9 - prepare for release (Ronald Holshausen, Mon Sep 26 21:32:05 2016 +1000)
* 40c9e02 - exclude IntelliJ files from publishing (Ronald Holshausen, Mon Sep 26 21:22:35 2016 +1000)
* c3a8a30 - renamed the pact_matching and pact_mock_server directories (Ronald Holshausen, Sun Sep 18 11:07:32 2016 +1000)

# 0.0.1 - Feature Release

* 21ca473 - add changelog to libpact_mock_server (Ronald Holshausen, Mon Jun 27 14:59:49 2016 +1000)
* 60077b4 - release script needs to be executable (Ronald Holshausen, Mon Jun 27 14:54:14 2016 +1000)
* 6712635 - added release script for libpact_mock_server (Ronald Holshausen, Mon Jun 27 14:10:20 2016 +1000)
* 0f7965a - updated README for libpact_mock_server (Ronald Holshausen, Mon Jun 27 13:36:37 2016 +1000)
* 518e14a - If the mock server has been shutdown, return a 401 Not Implemented (Ronald Holshausen, Sun Jun 26 11:04:58 2016 +1000)
* 6234bbd - implemented delete on the master server to shut a mock server down (Ronald Holshausen, Sat Jun 25 16:59:39 2016 +1000)
* 4c60f07 - replace rustful with webmachine (Ronald Holshausen, Thu Jun 16 17:31:11 2016 +1000)
* 44daccc - add an optional port number to start the mock server with (Ronald Holshausen, Wed Jun 15 12:40:51 2016 +1000)
* 60bbae5 - handle the result from setting up the logger framework (Ronald Holshausen, Fri Jun 10 11:21:10 2016 +1000)
* 4b8a98a - upgrade hyper to latest version in the mock server library (Ronald Holshausen, Thu Jun 9 21:50:22 2016 +1000)
* b769277 - also add static library as an artifact (Ronald Holshausen, Thu Jun 9 21:22:26 2016 +1000)
* 1c0c7cd - remove rustful from the mock server library (Ronald Holshausen, Thu Jun 9 21:09:32 2016 +1000)
* 7dc4b52 - implemented merging of pact files when writing (Ronald Holshausen, Thu Jun 9 17:34:02 2016 +1000)
* 34fd827 - implement a write_pact exported function to the mock server library (Ronald Holshausen, Thu Jun 9 12:15:01 2016 +1000)
* 769f840 - update the mock server cli readme (Ronald Holshausen, Wed Jun 8 16:05:56 2016 +1000)
* 5f99bb3 - links in readmes are relative to the file they are in (Ronald Holshausen, Wed Jun 8 11:58:05 2016 +1000)
* 0178f8b - change the link to the javascript examples (Ronald Holshausen, Wed Jun 8 11:55:32 2016 +1000)
* 2ba2a08 - correct the link to the javascript examples (Ronald Holshausen, Wed Jun 8 11:46:32 2016 +1000)
* e0130c5 - small tweaks to the libpact_mock_server library readme (Ronald Holshausen, Wed Jun 8 10:46:08 2016 +1000)
* 801f24c - update the github readmes to point to the published rust docs (Ronald Holshausen, Wed Jun 8 10:42:30 2016 +1000)
* 1577eeb - bump the version of libpact_mock_server (Ronald Holshausen, Wed Jun 1 21:59:48 2016 +1000)

# 0.0.0 - First Release
