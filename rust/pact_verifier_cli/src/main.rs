//! # Standalone Pact Verifier
//!
//! This project provides a command line interface to verify pact files against a running provider. It is a single executable binary. It implements the [V2 Pact specification](https://github.com/pact-foundation/pact-specification/tree/version-2).
//!
//! [Online rust docs](https://docs.rs/pact_verifier_cli/)
//!
//! The Pact Verifier works by taking all the interactions (requests and responses) from a number of pact files. For each interaction, it will make the request defined in the pact to a running service provider and check the response received back against the one defined in the pact file. All mismatches will then be reported.
//!
//! ## Command line interface
//!
//! The pact verifier is bundled as a single binary executable `pact_verifier_cli`. Running this with out any options displays the standard help.
//!
//! ```console,ignore
//! pact_verifier_cli v0.2.0
//! Standalone Pact verifier
//!
//! USAGE:
//!     pact_verifier_cli [FLAGS] [OPTIONS] --file <file> --dir <dir> --url <url> --broker-url <broker-url> --provider-name <provider-name>
//!
//! FLAGS:
//!         --filter-no-state          Only validate interactions that have no defined provider state
//!         --help                     Prints help information
//!         --state-change-as-query    State change request data will be sent as query parameters instead of in the request body
//!         --state-change-teardown    State change teardown requests are to be made after each interaction
//!     -v, --version                  Prints version information
//!
//! OPTIONS:
//!     -b, --broker-url <broker-url>                    URL of the pact broker to fetch pacts from to verify (requires the provider name parameter)
//!     -d, --dir <dir>                                  Directory of pact files to verify (can be repeated)
//!     -f, --file <file>                                Pact file to verify (can be repeated)
//!     -c, --filter-consumer <filter-consumer>       Consumer name to filter the pacts to be verified (can be repeated)
//!         --filter-description <filter-description>    Only validate interactions whose descriptions match this filter
//!         --filter-state <filter-state>                Only validate interactions whose provider states match this filter
//!     -h, --hostname <hostname>                        Provider hostname (defaults to localhost)
//!     -l, --loglevel <loglevel>                        Log level (defaults to warn) [values: error, warn, info, debug, trace, none]
//!     -p, --port <port>                                Provider port (defaults to 8080)
//!     -n, --provider-name <provider-name>              Provider name (defaults to provider)
//!     -s, --state-change-url <state-change-url>        URL to post state change requests to
//!     -u, --url <url>                                  URL of pact file to verify (can be repeated)
//! ```
//!
//! ## Options
//!
//! ### Log Level
//!
//! You can control the log level with the `-l, --loglevel <loglevel>` option. It defaults to warn, and the options that you can specify are: error, warn, info, debug, trace, none.
//!
//! ### Pact File Sources
//!
//! You can specify the pacts to verify with the following options. They can be repeated to set multiple sources.
//!
//! | Option | Type | Description |
//! |--------|------|-------------|
//! | `-f, --file <file>` | File | Loads a pact from the given file |
//! | `-u, --url <url>` | URL | Loads a pact from a URL resource |
//! | `-d, --dir <dir>` | Directory | Loads all the pacts from the given directory |
//! | `-b, --broker-url <broker-url>` | Pact Broker | Loads all the pacts for the provider from the pact broker. Requires the `-n, --provider-name <provider-name>` option |
//!
//! ### Provider Options
//!
//! The running provider can be specified with the following options:
//!
//! | Option | Description |
//! |--------|-------------|
//! | `-h, --hostname <hostname>` | The provider hostname, defaults to `localhost` |
//! | `-p, --port <port>` | The provider port (defaults to 8080) |
//! | `-n, --provider-name <provider-name>` | The name of the provider. Required if you are loading pacts from a pact broker |
//!
//! ### Filtering the interactions
//!
//! The interactions that are verified can be filtered by the following options:
//!
//! #### `-c, --filter-consumer <filter-consumer>`
//!
//! This will only verify the interactions of matching consumers. You can specify multiple consumers by either seperating the names with a comma, or repeating the option.
//!
//! #### `--filter-description <filter-description>`
//!
//! This option will filter the interactions that are verified that match by desciption. You can use a regular expression to match.
//!
//! #### `--filter-state <filter-state>`
//!
//! This option will filter the interactions that are verified that match by provider state. You can use a regular expression to match. Can't be used with the `--filter-no-state` option.
//!
//! #### `--filter-no-state`
//!
//! This option will filter the interactions that are verified that don't have a defined provider state. Can't be used with the `--filter-state` option.
//!
//! ### State change requests
//!
//! Provider states are a mechanism to define the state that the provider needs to be in to be able to verify a particular request. This is achieved by setting a state change URL that will receive a POST request with the provider state before the actual request is made.
//!
//! #### `-s, --state-change-url <state-change-url>`
//!
//! This sets the URL that the POST requests will be made to before each actual request.
//!
//! #### `--state-change-as-query`
//!
//! By default, the state for the state change request will be sent as a JSON document in the body of the request. This option forces it to be sent as a query parameter instead.
//! #### `--state-change-teardown`
//!
//!
//! This option will cause the verifier to also make a tear down request after the main request is made. It will receive a second field in the body or a query parameter named `action` with the value `teardown`.
//!
//! ## Example run
//!
//! This will verify all the pacts for the `happy_provider` found in the pact broker (running on localhost) against the provider running on localhost port 5050. Only the pacts for the consumers `Consumer` and `Consumer2` will be verified.
//!
//! ```console,ignore
//! $ pact_verifier_cli -b http://localhost -n 'happy_provider' -p 5050 --filter-consumer Consumer --filter-consumer Consumer2
//! 21:59:28 [WARN] pact_matching::models: No metadata found in pact file "http://localhost/pacts/provider/happy_provider/consumer/Consumer/version/1.0.0", assuming V1.1 specification
//! 21:59:28 [WARN] pact_matching::models: No metadata found in pact file "http://localhost/pacts/provider/happy_provider/consumer/Consumer2/version/1.0.0", assuming V1.1 specification
//!
//! Verifying a pact between Consumer and happy_provider
//!   Given I am friends with Fred
//!     WARNING: State Change ignored as there is no state change URL
//!   Given I have no friends
//!     WARNING: State Change ignored as there is no state change URL
//!   a request to unfriend but no friends
//!     returns a response which
//!       has status code 200 (OK)
//!       includes headers
//!       has a matching body (OK)
//!   a request friends
//!     returns a response which
//!       has status code 200 (FAILED)
//!       includes headers
//!         "Content-Type" with value "application/json" (FAILED)
//!       has a matching body (FAILED)
//!   a request to unfriend
//!     returns a response which
//!       has status code 200 (OK)
//!       includes headers
//!         "Content-Type" with value "application/json" (OK)
//!       has a matching body (FAILED)
//!
//!
//! Verifying a pact between Consumer2 and happy_provider
//!   Given I am friends with Fred
//!     WARNING: State Change ignored as there is no state change URL
//!   Given I have no friends
//!     WARNING: State Change ignored as there is no state change URL
//!   a request to unfriend but no friends
//!     returns a response which
//!       has status code 200 (OK)
//!       includes headers
//!       has a matching body (OK)
//!   a request friends
//!     returns a response which
//!       has status code 200 (FAILED)
//!       includes headers
//!         "Content-Type" with value "application/json" (FAILED)
//!       has a matching body (FAILED)
//!   a request to unfriend
//!     returns a response which
//!       has status code 200 (OK)
//!       includes headers
//!         "Content-Type" with value "application/json" (OK)
//!       has a matching body (FAILED)
//!
//!
//! Failures:
//!
//! 0) Verifying a pact between Consumer and happy_provider - a request friends returns a response which has a matching body
//!     expected "application/json" body but was "text/plain"
//!
//! 1) Verifying a pact between Consumer and happy_provider - a request friends returns a response which has status code 200
//!     expected 200 but was 404
//!
//! 2) Verifying a pact between Consumer and happy_provider - a request friends returns a response which includes header "Content-Type" with value "application/json"
//!     Expected header "Content-Type" to have value "application/json" but was "text/plain"
//!
//! 3) Verifying a pact between Consumer and happy_provider Given I am friends with Fred - a request to unfriend returns a response which has a matching body
//!     $.body -> Type mismatch: Expected Map {"reply":"Bye"} but received  "Ok"
//!
//!
//! 4) Verifying a pact between Consumer2 and happy_provider - a request friends returns a response which has a matching body
//!     expected "application/json" body but was "text/plain"
//!
//! 5) Verifying a pact between Consumer2 and happy_provider - a request friends returns a response which has status code 200
//!     expected 200 but was 404
//!
//! 6) Verifying a pact between Consumer2 and happy_provider - a request friends returns a response which includes header "Content-Type" with value "application/json"
//!     Expected header "Content-Type" to have value "application/json" but was "text/plain"
//!
//! 7) Verifying a pact between Consumer2 and happy_provider Given I am friends with Fred - a request to unfriend returns a response which has a matching body
//!     $.body -> Type mismatch: Expected Map {"reply":"Bye"} but received  "Ok"
//!
//!
//!
//! There were 8 pact failures
//!
//! ```

#![warn(missing_docs)]

#[macro_use] extern crate clap;
#[macro_use] extern crate log;
#[macro_use] extern crate pact_matching;
extern crate pact_verifier;
extern crate simplelog;
extern crate rand;
extern crate regex;

#[cfg(test)]
#[macro_use(expect)]
extern crate expectest;

#[cfg(test)]
extern crate quickcheck;

use std::env;
use clap::{Arg, App, AppSettings, ErrorKind, ArgMatches};
use pact_matching::models::PactSpecification;
use pact_verifier::*;
use log::LogLevelFilter;
use simplelog::TermLogger;
use std::str::FromStr;
use std::error::Error;
use regex::Regex;

fn main() {
    match handle_command_args() {
        Ok(_) => (),
        Err(err) => std::process::exit(err)
    }
}

fn print_version() {
    println!("\npact verifier version     : v{}", crate_version!());
    println!("pact specification version: v{}", PactSpecification::V2.version_str());
}

fn integer_value(v: String) -> Result<(), String> {
    v.parse::<u16>().map(|_| ()).map_err(|e| format!("'{}' is not a valid port value: {}", v, e) )
}

fn pact_source(matches: &ArgMatches) -> Vec<PactSource> {
    let mut sources = vec![];
    match matches.values_of("file") {
        Some(values) => sources.extend(values.map(|v| PactSource::File(s!(v))).collect::<Vec<PactSource>>()),
        None => ()
    };
    match matches.values_of("dir") {
        Some(values) => sources.extend(values.map(|v| PactSource::Dir(s!(v))).collect::<Vec<PactSource>>()),
        None => ()
    };
    match matches.values_of("url") {
        Some(values) => sources.extend(values.map(|v| PactSource::URL(s!(v))).collect::<Vec<PactSource>>()),
        None => ()
    };
    match matches.values_of("broker-url") {
        Some(values) => sources.extend(values.map(|v| PactSource::BrokerUrl(s!(matches.value_of("provider-name").unwrap()),
            s!(v))).collect::<Vec<PactSource>>()),
        None => ()
    };
    sources
}

fn interaction_filter(matches: &ArgMatches) -> FilterInfo {
    if matches.is_present("filter-description") &&
        (matches.is_present("filter-state") || matches.is_present("filter-no-state")) {
        if matches.is_present("filter-state") {
            FilterInfo::DescriptionAndState(s!(matches.value_of("filter-description").unwrap()),
                s!(matches.value_of("filter-state").unwrap()))
        } else {
            FilterInfo::DescriptionAndState(s!(matches.value_of("filter-description").unwrap()),
                s!(""))
        }
    } else if matches.is_present("filter-description") {
        FilterInfo::Description(s!(matches.value_of("filter-description").unwrap()))
    } else if matches.is_present("filter-state") {
        FilterInfo::State(s!(matches.value_of("filter-state").unwrap()))
    } else if matches.is_present("filter-no-state") {
        FilterInfo::State(s!(""))
    } else {
        FilterInfo::None
    }
}

fn handle_command_args() -> Result<(), i32> {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let version = format!("v{}", crate_version!());
    let app = App::new(program)
        .version(version.as_str())
        .about("Standalone Pact verifier")
        .version_short("v")
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::ColoredHelp)
        .arg(Arg::with_name("loglevel")
            .short("l")
            .long("loglevel")
            .takes_value(true)
            .use_delimiter(false)
            .possible_values(&["error", "warn", "info", "debug", "trace", "none"])
            .help("Log level (defaults to warn)"))
        .arg(Arg::with_name("file")
            .short("f")
            .long("file")
            .required_unless_one(&["dir", "url", "broker-url"])
            .takes_value(true)
            .use_delimiter(false)
            .multiple(true)
            .number_of_values(1)
            .empty_values(false)
            .help("Pact file to verify (can be repeated)"))
        .arg(Arg::with_name("dir")
            .short("d")
            .long("dir")
            .required_unless_one(&["file", "url", "broker-url"])
            .takes_value(true)
            .use_delimiter(false)
            .multiple(true)
            .number_of_values(1)
            .empty_values(false)
            .help("Directory of pact files to verify (can be repeated)"))
        .arg(Arg::with_name("url")
            .short("u")
            .long("url")
            .required_unless_one(&["file", "dir", "broker-url"])
            .takes_value(true)
            .use_delimiter(false)
            .multiple(true)
            .number_of_values(1)
            .empty_values(false)
            .help("URL of pact file to verify (can be repeated)"))
        .arg(Arg::with_name("broker-url")
            .short("b")
            .long("broker-url")
            .required_unless_one(&["file", "dir", "url"])
            .requires("provider-name")
            .takes_value(true)
            .use_delimiter(false)
            .multiple(true)
            .number_of_values(1)
            .empty_values(false)
            .help("URL of the pact broker to fetch pacts from to verify (requires the provider name parameter)"))
        .arg(Arg::with_name("hostname")
            .short("h")
            .long("hostname")
            .takes_value(true)
            .use_delimiter(false)
            .help("Provider hostname (defaults to localhost)"))
        .arg(Arg::with_name("port")
            .short("p")
            .long("port")
            .takes_value(true)
            .use_delimiter(false)
            .help("Provider port (defaults to 8080)")
            .validator(integer_value))
        .arg(Arg::with_name("provider-name")
            .short("n")
            .long("provider-name")
            .takes_value(true)
            .use_delimiter(false)
            .help("Provider name (defaults to provider)"))
        .arg(Arg::with_name("state-change-url")
            .short("s")
            .long("state-change-url")
            .takes_value(true)
            .use_delimiter(false)
            .help("URL to post state change requests to"))
        .arg(Arg::with_name("state-change-as-query")
            .long("state-change-as-query")
            .help("State change request data will be sent as query parameters instead of in the request body"))
        .arg(Arg::with_name("state-change-teardown")
            .long("state-change-teardown")
            .help("State change teardown requests are to be made after each interaction"))
        .arg(Arg::with_name("filter-description")
            .long("filter-description")
            .takes_value(true)
            .use_delimiter(false)
            .validator(|val| Regex::new(&val)
                .map(|_| ())
                .map_err(|err| format!("'{}' is an invalid filter value: {}", val, err.description())))
            .help("Only validate interactions whose descriptions match this filter"))
        .arg(Arg::with_name("filter-state")
            .long("filter-state")
            .takes_value(true)
            .use_delimiter(false)
            .conflicts_with("filter-no-state")
            .validator(|val| Regex::new(&val)
                .map(|_| ())
                .map_err(|err| format!("'{}' is an invalid filter value: {}", val, err.description())))
            .help("Only validate interactions whose provider states match this filter"))
        .arg(Arg::with_name("filter-no-state")
            .long("filter-no-state")
            .conflicts_with("filter-state")
            .help("Only validate interactions that have no defined provider state"))
        .arg(Arg::with_name("filter-consumer")
            .short("c")
            .long("filter-consumer")
            .takes_value(true)
            .multiple(true)
            .empty_values(false)
            .help("Consumer name to filter the pacts to be verified (can be repeated)"))
        ;

    let matches = app.get_matches_safe();
    match matches {
        Ok(ref matches) => {
            let level = matches.value_of("loglevel").unwrap_or("warn");
            let log_level = match level {
                "none" => LogLevelFilter::Off,
                _ => LogLevelFilter::from_str(level).unwrap()
            };
            TermLogger::init(log_level).unwrap();
            let provider = ProviderInfo {
                host: s!(matches.value_of("hostname").unwrap_or("localhost")),
                port: matches.value_of("port").unwrap_or("8080").parse::<u16>().unwrap(),
                state_change_url: matches.value_of("state-change-url").map(|s| s.to_string()),
                state_change_body: !matches.is_present("state-change-as-query"),
                state_change_teardown: matches.is_present("state-change-teardown"),
                .. ProviderInfo::default()
            };
            let source = pact_source(matches);
            let filter = interaction_filter(matches);
            if verify_provider(&provider, source, &filter, &matches.values_of_lossy("filter-consumer").unwrap_or(vec![])) {
                Ok(())
            } else {
                Err(2)
            }
        },
        Err(ref err) => {
            match err.kind {
                ErrorKind::HelpDisplayed => {
                    println!("{}", err.message);
                    Ok(())
                },
                ErrorKind::VersionDisplayed => {
                    print_version();
                    println!("");
                    Ok(())
                },
                _ => {
                    println!("{}", err.message);
                    err.exit()
                }
            }
        }
    }
}

#[cfg(test)]
mod test {

    use quickcheck::{TestResult, quickcheck};
    use rand::Rng;
    use super::integer_value;
    use expectest::prelude::*;

    #[test]
    fn validates_integer_value() {
        fn prop(s: String) -> TestResult {
            let mut rng = ::rand::thread_rng();
            if rng.gen() && s.chars().any(|ch| !ch.is_numeric()) {
                TestResult::discard()
            } else {
                let validation = integer_value(s.clone());
                match validation {
                    Ok(_) => TestResult::from_bool(!s.is_empty() && s.chars().all(|ch| ch.is_numeric() )),
                    Err(_) => TestResult::from_bool(s.is_empty() || s.chars().find(|ch| !ch.is_numeric() ).is_some())
                }
            }
        }
        quickcheck(prop as fn(_) -> _);

        expect!(integer_value(s!("1234"))).to(be_ok());
        expect!(integer_value(s!("1234x"))).to(be_err());
    }
}
