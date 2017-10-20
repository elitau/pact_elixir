To generate the log, run `git log --pretty='* %h - %s (%an, %ad)' TAGNAME..HEAD .` replacing TAGNAME and HEAD as appropriate.

# 0.3.0 - Improved Consumer DSL

* 89bebb3 - Correct the paths in the release scripts for pact_consumer (Ronald Holshausen, Fri Oct 20 11:36:05 2017 +1100)
* ac94388 - Tests are now all passing #20 (Ronald Holshausen, Thu Oct 19 15:14:25 2017 +1100)
* d990729 - Some code cleanup #20 (Ronald Holshausen, Wed Oct 18 16:32:37 2017 +1100)
* db6100e - Updated the consumer DSL to use the matching rules (compiling, but tests are failing) #20 (Ronald Holshausen, Wed Oct 18 15:48:23 2017 +1100)
* c983c63 - Bump versions to 0.3.0 (Ronald Holshausen, Wed Oct 18 13:54:46 2017 +1100)
* 44e2cf6 - Add myself to "authors" list (Eric Kidd, Wed Oct 11 11:31:08 2017 -0400)
* 1029745 - Provide more context in top-level crate docs (Eric Kidd, Wed Oct 11 11:29:30 2017 -0400)
* 28b7742 - Add a `strip_null_fields` helper (Eric Kidd, Wed Oct 11 11:21:22 2017 -0400)
* 3e3e5a7 - Change `json` helper to `json_utf8` (Eric Kidd, Wed Oct 11 10:06:15 2017 -0400)
* d53dc01 - Allow `each_like!({ "a": 1 }, min = 2)` (Eric Kidd, Wed Oct 11 09:02:07 2017 -0400)
* 8f864cb - Confirm that `^` and `$` are required (Eric Kidd, Wed Oct 11 08:50:22 2017 -0400)
* 9de566b - Rename `something_like!` and `array_like!` to match JS (Eric Kidd, Wed Oct 11 08:39:06 2017 -0400)
* 01f09be - [BUG] pact_matching: Parse JSON paths with `_` (Eric Kidd, Tue Oct 10 08:55:44 2017 -0400)
* 76b9cd7 - Add helper methods for building popular properties (Eric Kidd, Tue Oct 10 06:42:01 2017 -0400)
* f0e2522 - Add `MockServer::path` and update examples (Eric Kidd, Mon Oct 9 16:43:53 2017 -0400)
* 6d9bb6a - Add macros for `term!` and other special rules (Eric Kidd, Mon Oct 9 16:19:53 2017 -0400)
* 25ad54b - Convert builders to use `StringPattern` (Eric Kidd, Mon Oct 9 12:00:05 2017 -0400)
* 86efdc0 - Add a `get_defaulting` helper and break out utils (Eric Kidd, Mon Oct 9 11:48:22 2017 -0400)
* 12bd014 - Create a new `StringPattern` type (Eric Kidd, Mon Oct 9 11:16:31 2017 -0400)
* 137e349 - Fix outdated comment (Eric Kidd, Mon Oct 9 08:47:40 2017 -0400)
* da9cfda - Implement new, experimental syntax (API BREAKAGE) (Eric Kidd, Sun Oct 8 13:33:09 2017 -0400)
* eb5fcd6 - Fix warnings by removing unused `p-macro` (Eric Kidd, Fri Oct 6 07:56:44 2017 -0400)
* e6ad973 - Reorganize `matchables` code (Eric Kidd, Fri Oct 6 07:55:24 2017 -0400)
* d6f867b - Replace `Term` with open-ended `Matchable` trait (Eric Kidd, Fri Oct 6 06:56:02 2017 -0400)
* 23f0a26 - Create a Rust version of `Term` (Eric Kidd, Thu Oct 5 07:49:12 2017 -0400)
* 3f42e50 - Implement `JsonPattern` w/o matcher support (Eric Kidd, Wed Oct 4 13:40:09 2017 -0400)
* 182b0a4 - Add a `body_present` function that handles boilerplate (Eric Kidd, Tue Oct 3 10:42:55 2017 -0400)
* 0bd43a3 - Get rid of `hashmap!` in public APIs (Eric Kidd, Tue Oct 3 09:19:53 2017 -0400)
* 4e9f6a6 - Replace `s!` with `Into<String>` (Eric Kidd, Tue Oct 3 07:18:02 2017 -0400)
* 359f1f5 - Re-export OptionalBody (Eric Kidd, Tue Oct 3 07:17:01 2017 -0400)
* 487a0bd - pact_consumer: Move doctest to tests.rs temporarily (Eric Kidd, Tue Oct 3 06:33:54 2017 -0400)
* 06e92e5 - Refer to local libs using version+paths (Eric Kidd, Tue Oct 3 06:22:23 2017 -0400)
* 4c7c66a - Missed updating the crate versions for pact_consumer (Ronald Holshausen, Wed May 17 12:45:06 2017 +1000)
* 7afd258 - Update all the cargo manifest versions and commit the cargo lock files (Ronald Holshausen, Wed May 17 10:37:44 2017 +1000)
* be8c299 - Cleanup unused BTreeMap usages and use remote pact dependencies (Anthony Damtsis, Mon May 15 17:09:14 2017 +1000)
* a59fb98 - Migrate remaining pact modules over to serde (Anthony Damtsis, Mon May 15 16:59:04 2017 +1000)
* c988180 - bump version to 0.2.1 (Ronald Holshausen, Sun Oct 9 16:55:35 2016 +1100)

# 0.2.0 - V2 implementation

* 2eb38fc - update the consumer library versions for the V2 branch (Ronald Holshausen, Sun Oct 9 16:50:03 2016 +1100)
* e3eebbd -  update projects to use the published pact mock server library (Ronald Holshausen, Sun Oct 9 16:36:25 2016 +1100)
* 770010a - update projects to use the published pact matching lib (Ronald Holshausen, Sun Oct 9 16:25:15 2016 +1100)
* 574e072 - upadte versions for V2 branch and fix an issue with loading JSON bodies encoded as a string (Ronald Holshausen, Sun Oct 9 15:31:57 2016 +1100)
* 6d581d5 - bump version to 0.1.1 (Ronald Holshausen, Sat Oct 8 17:59:33 2016 +1100)

# 0.1.0 - V1.1 specification implementation

* dae5d42 - correct the doc link (Ronald Holshausen, Sat Oct 8 17:55:15 2016 +1100)
* 16b99b5 - V1.1 spec changes (Ronald Holshausen, Sat Oct 8 17:53:53 2016 +1100)
* 1f3f3f1 - correct the versions of the inter-dependent projects as they were causing the build to fail (Ronald Holshausen, Sat Oct 8 17:41:57 2016 +1100)
* a46dabb - update all references to V1 spec after merge (Ronald Holshausen, Sat Oct 8 16:20:51 2016 +1100)
* 548c5aa - bump version to 0.0.1 (Ronald Holshausen, Mon Sep 26 23:16:50 2016 +1000)
* d80e899 - release script needs to be executable (Ronald Holshausen, Mon Sep 26 23:14:15 2016 +1000)

# 0.0.0 - First Release
