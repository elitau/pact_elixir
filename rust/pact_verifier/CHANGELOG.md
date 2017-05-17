To generate the log, run `git log --pretty='* %h - %s (%an, %ad)' TAGNAME..HEAD .` replacing TAGNAME and HEAD as appropriate.

# 0.2.1 - Replace rustc_serialize with serde_json

* a1f78f9 - Move linux specific bits out of the release script (Ronald Holshausen, Wed May 17 10:18:37 2017 +1000)
* efe4ca7 - Cleanup unused imports and unreachable pattern warning messages (Anthony Damtsis, Tue May 16 10:31:29 2017 +1000)
* be8c299 - Cleanup unused BTreeMap usages and use remote pact dependencies (Anthony Damtsis, Mon May 15 17:09:14 2017 +1000)
* a59fb98 - Migrate remaining pact modules over to serde (Anthony Damtsis, Mon May 15 16:59:04 2017 +1000)
* 3ca29d6 - bump version to 0.2.1 (Ronald Holshausen, Sun Oct 9 17:06:35 2016 +1100)

# 0.2.0 - V2 specification implementation

* 91f5315 - update the references to the spec in the verifier library to V2 (Ronald Holshausen, Sun Oct 9 16:59:45 2016 +1100)
* e2f88b8 - update the verifier library to use the published consumer library (Ronald Holshausen, Sun Oct 9 16:57:34 2016 +1100)
* 770010a - update projects to use the published pact matching lib (Ronald Holshausen, Sun Oct 9 16:25:15 2016 +1100)
* 574e072 - upadte versions for V2 branch and fix an issue with loading JSON bodies encoded as a string (Ronald Holshausen, Sun Oct 9 15:31:57 2016 +1100)
* dabe425 - bump version to 0.1.1 (Ronald Holshausen, Sun Oct 9 10:40:39 2016 +1100)

# 0.1.0 - V1.1 specification implementation

* 7b66941 - Update the deps for pact verifier library (Ronald Holshausen, Sun Oct 9 10:32:47 2016 +1100)
* 1f3f3f1 - correct the versions of the inter-dependent projects as they were causing the build to fail (Ronald Holshausen, Sat Oct 8 17:41:57 2016 +1100)
* a46dabb - update all references to V1 spec after merge (Ronald Holshausen, Sat Oct 8 16:20:51 2016 +1100)
* 1246784 - correct the verifier library release script (Ronald Holshausen, Tue Sep 27 20:57:13 2016 +1000)
* f0ce08a - bump version to 0.0.1 (Ronald Holshausen, Tue Sep 27 20:43:34 2016 +1000)

# 0.0.0 - First Release
