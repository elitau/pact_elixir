To generate the log, run `git log --pretty='* %h - %s (%an, %ad)' TAGNAME..HEAD .` replacing TAGNAME and HEAD as appropriate.

# 0.0.2 - Fixes required for verifing pacts

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
