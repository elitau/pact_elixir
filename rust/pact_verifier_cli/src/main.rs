//! The `pact_verifier_cli` crate provides the CLI for verification of service providers based on
//! pact files. It implements the V1 Pact specification
//! (https://github.com/pact-foundation/pact-specification/tree/version-1).

#![warn(missing_docs)]

#[macro_use] extern crate clap;
#[macro_use] extern crate p_macro;
#[macro_use] extern crate log;
#[macro_use] extern crate maplit;
#[macro_use] extern crate pact_matching;
extern crate pact_verifier;
extern crate simplelog;
extern crate rand;

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

fn main() {
    match handle_command_args() {
        Ok(_) => (),
        Err(err) => std::process::exit(err)
    }
}

fn print_version() {
    println!("\npact verifier version     : v{}", crate_version!());
    println!("pact specification version: v{}", PactSpecification::V1.version_str());
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
    sources
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
            .required_unless_one(&["dir"])
            .takes_value(true)
            .use_delimiter(false)
            .multiple(true)
            .number_of_values(1)
            .empty_values(false)
            .help("Pact file to verify (can be repeated)"))
        .arg(Arg::with_name("dir")
            .short("d")
            .long("dir")
            .required_unless_one(&["file"])
            .takes_value(true)
            .use_delimiter(false)
            .multiple(true)
            .number_of_values(1)
            .empty_values(false)
            .help("Directory of pact files to verify (can be repeated)"))
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
                .. ProviderInfo::default()
            };
            let source = pact_source(matches);
            if verify_provider(&provider, source) {
                Ok(())
            } else {
                Err(2)
            }
        },
        Err(ref err) => {
            match err.kind {
                ErrorKind::HelpDisplayed => {
                    println!("");
                    Ok(())
                },
                ErrorKind::VersionDisplayed => {
                    print_version();
                    println!("");
                    Ok(())
                },
                _ => {
                    println!("");
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
