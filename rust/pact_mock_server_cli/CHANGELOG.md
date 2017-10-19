To generate the log, run `git log --pretty='* %h - %s (%an, %ad)' TAGNAME..HEAD .` replacing TAGNAME and HEAD as appropriate.

# 0.3.0 - Backported the matching rules from the V3 branch

* c8595cc - Correct the paths in the release scripts for pact_mock_server_cli (Ronald Holshausen, Fri Oct 20 10:48:03 2017 +1100)
* ac94388 - Tests are now all passing #20 (Ronald Holshausen, Thu Oct 19 15:14:25 2017 +1100)
* c983c63 - Bump versions to 0.3.0 (Ronald Holshausen, Wed Oct 18 13:54:46 2017 +1100)
* 06e92e5 - Refer to local libs using version+paths (Eric Kidd, Tue Oct 3 06:22:23 2017 -0400)
* 7afd258 - Update all the cargo manifest versions and commit the cargo lock files (Ronald Holshausen, Wed May 17 10:37:44 2017 +1000)
* be8c299 - Cleanup unused BTreeMap usages and use remote pact dependencies (Anthony Damtsis, Mon May 15 17:09:14 2017 +1000)
* a59fb98 - Migrate remaining pact modules over to serde (Anthony Damtsis, Mon May 15 16:59:04 2017 +1000)
* c5f9c27 - bump version to 0.2.4 (Ronald Holshausen, Sun Apr 23 17:39:49 2017 +1000)

# 0.2.3 - Bugfix Release

* 224ad98 - Change no-console-log to no-term-log and use a simple logger if it is set #6 (Ronald Holshausen, Sun Apr 23 17:19:53 2017 +1000)
* cec2358 - bump version to 0.2.3 (Ronald Holshausen, Fri Apr 21 14:33:13 2017 +1000)

# 0.2.2 - Bugfix Release

* 53074cf - Merge branch 'v1.1-spec' into v2-spec (Ronald Holshausen, Fri Apr 21 14:17:05 2017 +1000)
* 01fa713 - bump version to 0.1.3 (Ronald Holshausen, Fri Apr 21 14:08:58 2017 +1000)
* e4b59e5 - update changelog for release 0.1.2 (Ronald Holshausen, Fri Apr 21 14:05:14 2017 +1000)
* 07b1827 - Merge branch 'v1-spec' into v1.1-spec (Ronald Holshausen, Fri Apr 21 13:39:50 2017 +1000)
* da4e32f - bump version to 0.0.3 (Ronald Holshausen, Fri Apr 21 13:31:18 2017 +1000)
* 9b4b5fb - update changelog for release 0.0.2 (Ronald Holshausen, Fri Apr 21 13:27:54 2017 +1000)
* 2276cd0 - upgraded simple log crate and added cli options to disable file or console logging #6 (Ronald Holshausen, Fri Apr 21 13:15:27 2017 +1000)
* ea5cec8 - bump version to 0.2.2 (Ronald Holshausen, Sun Oct 9 16:43:59 2016 +1100)
* 0b83b06 - correct the displayed help for the pact_mock_server_cli (Ronald Holshausen, Sat Oct 8 17:29:19 2016 +1100)

# 0.1.2 - Bugfix Release

* 07b1827 - Merge branch 'v1-spec' into v1.1-spec (Ronald Holshausen, Fri Apr 21 13:39:50 2017 +1000)
* da4e32f - bump version to 0.0.3 (Ronald Holshausen, Fri Apr 21 13:31:18 2017 +1000)
* 9b4b5fb - update changelog for release 0.0.2 (Ronald Holshausen, Fri Apr 21 13:27:54 2017 +1000)
* 2276cd0 - upgraded simple log crate and added cli options to disable file or console logging #6 (Ronald Holshausen, Fri Apr 21 13:15:27 2017 +1000)
* 91d1216 - bump version to 0.1.2 (Ronald Holshausen, Sat Oct 8 17:49:20 2016 +1100)
* 0b83b06 - correct the displayed help for the pact_mock_server_cli (Ronald Holshausen, Sat Oct 8 17:29:19 2016 +1100)

# 0.0.2 - Bugfix Release

* 2276cd0 - upgraded simple log crate and added cli options to disable file or console logging #6 (Ronald Holshausen, Fri Apr 21 13:15:27 2017 +1000)
* 0b83b06 - correct the displayed help for the pact_mock_server_cli (Ronald Holshausen, Sat Oct 8 17:29:19 2016 +1100)
* 04d9e5f - update the docs for the pact consumer library (Ronald Holshausen, Mon Sep 26 23:06:19 2016 +1000)
* 40c9e02 - exclude IntelliJ files from publishing (Ronald Holshausen, Mon Sep 26 21:22:35 2016 +1000)
* c1d97a0 - correct the repository paths in the cargo manifests (Ronald Holshausen, Tue Jun 28 14:52:46 2016 +1000)
* 91d6d62 - removed the v1 from the project path, will use a git branch instead (Ronald Holshausen, Mon Jun 27 22:09:32 2016 +1000)

# 0.2.1 - Changes required for verifying V2 pacts

* e3eebbd -  update projects to use the published pact mock server library (Ronald Holshausen, Sun Oct 9 16:36:25 2016 +1100)
* 770010a - update projects to use the published pact matching lib (Ronald Holshausen, Sun Oct 9 16:25:15 2016 +1100)
* a21973a - Get the build passing after merge from V1.1 branch (Ronald Holshausen, Sun Oct 9 13:47:09 2016 +1100)
* 341607c - Merge branch 'v1.1-spec' into v2-spec (Ronald Holshausen, Sun Oct 9 12:10:12 2016 +1100)
* 91d1216 - bump version to 0.1.2 (Ronald Holshausen, Sat Oct 8 17:49:20 2016 +1100)
* 0d324a8 - bump version to 0.2.1 (Ronald Holshausen, Wed Jul 13 14:26:40 2016 +1000)
* 377b372 - update changelog for release 0.2.0 (Ronald Holshausen, Wed Jul 13 14:22:15 2016 +1000)
* 7ed156e - updated project for the V2 spec release (Ronald Holshausen, Wed Jul 13 14:19:12 2016 +1000)
* 22b0bb9 - fix for failing build (Ronald Holshausen, Tue Jul 12 16:59:56 2016 +1000)
* 534e7a1 - updated readmes and bump versions for the V2 implementation (Ronald Holshausen, Wed Jun 29 10:38:32 2016 +1000)

# 0.1.1 - Changes required for verifying V1.1 pacts

* 28928ef - correct the displayed help for the pact_mock_server_cli (Ronald Holshausen, Sat Oct 8 17:29:19 2016 +1100)
* 3ca2df8 - update dependencies (Ronald Holshausen, Sat Oct 8 17:22:48 2016 +1100)
* a46dabb - update all references to V1 spec after merge (Ronald Holshausen, Sat Oct 8 16:20:51 2016 +1100)
* 1d6d4f8 - Merge branch 'v1-spec' into v1.1-spec (Ronald Holshausen, Sat Oct 8 15:44:25 2016 +1100)
* 04d9e5f - update the docs for the pact consumer library (Ronald Holshausen, Mon Sep 26 23:06:19 2016 +1000)
* 40c9e02 - exclude IntelliJ files from publishing (Ronald Holshausen, Mon Sep 26 21:22:35 2016 +1000)
* efe036c - bump version to 0.1.1 (Ronald Holshausen, Tue Jun 28 21:54:59 2016 +1000)
* c1d97a0 - correct the repository paths in the cargo manifests (Ronald Holshausen, Tue Jun 28 14:52:46 2016 +1000)

# 0.2.0 - V2 Specification Implementation

* 7ed156e - updated project for the V2 spec release (Ronald Holshausen, Wed Jul 13 14:19:12 2016 +1000)
* 22b0bb9 - fix for failing build (Ronald Holshausen, Tue Jul 12 16:59:56 2016 +1000)
* 534e7a1 - updated readmes and bump versions for the V2 implementation (Ronald Holshausen, Wed Jun 29 10:38:32 2016 +1000)
* efe036c - bump version to 0.1.1 (Ronald Holshausen, Tue Jun 28 21:54:59 2016 +1000)

# 0.1.0 - V1.1 Specification Implementation

* f91bb6e - use the published versions of the matching and mock server libraries (Ronald Holshausen, Tue Jun 28 21:38:21 2016 +1000)
* 140526d - Implement V1.1 matching (Ronald Holshausen, Tue Jun 28 15:58:35 2016 +1000)
* 4224875 - update readmes and bump versions for V1.1 implementation (Ronald Holshausen, Tue Jun 28 15:05:39 2016 +1000)
* 91d6d62 - removed the v1 from the project path, will use a git branch instead (Ronald Holshausen, Mon Jun 27 22:09:32 2016 +1000)

# 0.0.1 - Feature Release

* 18c009b - added changelog (Ronald Holshausen, Mon Jun 27 19:42:26 2016 +1000)
* 78126ab - no point publishing the rust docs as pact_mock_server_cli is not a library (Ronald Holshausen, Mon Jun 27 19:38:56 2016 +1000)
* 8867836 - correct the release script (Ronald Holshausen, Mon Jun 27 19:36:46 2016 +1000)
* aa2d2dd - added release script for pact_mock_server_cli (Ronald Holshausen, Mon Jun 27 17:20:38 2016 +1000)
* 2a78f40 - updated the README for the pact_mock_server_cli (Ronald Holshausen, Mon Jun 27 17:01:16 2016 +1000)
* 3f77f3f - update pact_mock_server_cli to depend on libpact_mock_server from crates.io (Ronald Holshausen, Mon Jun 27 15:50:15 2016 +1000)
* 3b6bf66 - fix the project deps for the travis build (Ronald Holshausen, Mon Jun 27 14:46:19 2016 +1000)
* f7d9960 - implemented the shutdown mock server command (Ronald Holshausen, Sun Jun 26 15:05:40 2016 +1000)
* f91b9fd - compile against the published webmachine crate (Ronald Holshausen, Sun Jun 26 13:14:34 2016 +1000)
* b7635b8 - correctly handle the status codes from the master mock server (Ronald Holshausen, Sun Jun 26 10:49:47 2016 +1000)
* 6234bbd - implemented delete on the master server to shut a mock server down (Ronald Holshausen, Sat Jun 25 16:59:39 2016 +1000)
* ec23a8b - use a Hyper Handler instead of a closure as it is easier to be thread safe (Ronald Holshausen, Fri Jun 24 16:30:28 2016 +1000)
* dd850bc - Got POST to main resource working with webmachine (Ronald Holshausen, Thu Jun 23 13:01:25 2016 +1000)
* b5b41ee - got GET to main resource working with webmachine (Ronald Holshausen, Thu Jun 23 11:30:10 2016 +1000)
* 079fdd4 - correct the webmachine-rust reference (Ronald Holshausen, Thu Jun 16 19:35:39 2016 +1000)
* 4c60f07 - replace rustful with webmachine (Ronald Holshausen, Thu Jun 16 17:31:11 2016 +1000)
* 44daccc - add an optional port number to start the mock server with (Ronald Holshausen, Wed Jun 15 12:40:51 2016 +1000)
* 0cfc690 - add the webmachine project as a dependency (Ronald Holshausen, Thu Jun 9 22:26:16 2016 +1000)
* 7dc4b52 - implemented merging of pact files when writing (Ronald Holshausen, Thu Jun 9 17:34:02 2016 +1000)
* 34fd827 - implement a write_pact exported function to the mock server library (Ronald Holshausen, Thu Jun 9 12:15:01 2016 +1000)
* dcde5dc - add a newline at the end of the help for people with crazy terminal settings (Ronald Holshausen, Thu Jun 9 11:12:16 2016 +1000)
* 511d7a1 - bump version of pact mock server cli (Ronald Holshausen, Wed Jun 8 20:27:53 2016 +1000)
* 5157386 - add rustdoc comment to the cli main file (Ronald Holshausen, Wed Jun 8 20:01:12 2016 +1000)


# 0.0.0 - First Release
