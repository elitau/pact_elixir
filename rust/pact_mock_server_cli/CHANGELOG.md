To generate the log, run `git log --pretty='* %h - %s (%an, %ad)' TAGNAME..HEAD .` replacing TAGNAME and HEAD as appropriate.

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
