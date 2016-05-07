#[macro_use] extern crate clap;
#[macro_use] extern crate pact_v1_matching;
extern crate pact_v1_mock_server;
#[macro_use] extern crate rustful;
#[macro_use] extern crate p_macro;
#[macro_use] extern crate log;
#[macro_use] extern crate maplit;
extern crate simplelog;
extern crate uuid;
extern crate rustc_serialize;
#[macro_use] extern crate hyper;

use clap::{Arg, App, SubCommand, AppSettings, ErrorKind, ArgMatches};
use std::env;
use std::str::FromStr;
use std::fs::{self, File};
use log::{LogLevelFilter};
use simplelog::{CombinedLogger, TermLogger, FileLogger};

fn display_error(error: String, matches: &ArgMatches) -> ! {
    println!("ERROR: {}", error);
    println!("");
    println!("{}", matches.usage());
    panic!("{}", error)
}

mod server;
mod create_mock;
mod list;

static SPEC_VERSION: &'static str = "1.0.0";

fn print_version() {
    println!("\npact mock server version  : v{}", crate_version!());
    println!("pact specification version: v{}", SPEC_VERSION);
}

fn setup_loggers(level: &str, command: &str, output: Option<&str>) {
    let log_level = match level {
        "none" => LogLevelFilter::Off,
        _ => LogLevelFilter::from_str(level).unwrap()
    };
    if command == "start" {
        let log_file = match output {
            Some(path) => {
                fs::create_dir_all(path).unwrap();
                format!("{}/pact_v1_mock_server.log", path)
            },
            None => s!("pact_v1_mock_server.log")
        };
        CombinedLogger::init(
            vec![
                TermLogger::new(log_level),
                FileLogger::new(log_level, File::create(log_file).unwrap())
            ]
        ).unwrap();
    } else {
        TermLogger::init(log_level).unwrap();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let version = format!("v{}", crate_version!());
    let app = App::new(program)
        .version(version.as_str())
        .about("Standalone Pact mock server")
        .version_short("v")
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::SubcommandRequired)
        .setting(AppSettings::GlobalVersion)
        .setting(AppSettings::VersionlessSubcommands)
        .setting(AppSettings::ColoredHelp)
        .arg(Arg::with_name("port")
            .short("p")
            .long("port")
            .takes_value(true)
            .use_delimiter(false)
            .help("port the master mock server runs on (defaults to 8080)"))
        .arg(Arg::with_name("host")
            .short("h")
            .long("host")
            .takes_value(true)
            .use_delimiter(false)
            .help("hostname the master mock server runs on (defaults to localhost)"))
        .arg(Arg::with_name("loglevel")
            .short("l")
            .long("loglevel")
            .takes_value(true)
            .use_delimiter(false)
            .possible_values(&["error", "warn", "info", "debug", "trace", "none"])
            .help("Log level for mock servers to write to the log file (defaults to info)"))
        .subcommand(SubCommand::with_name("start")
                .about("Starts the master mock server")
                .setting(AppSettings::ColoredHelp)
                .arg(Arg::with_name("output")
                      .short("o")
                      .long("output")
                      .takes_value(true)
                      .use_delimiter(false)
                      .help("the directory where to write files to (defaults to current directory)")))
        .subcommand(SubCommand::with_name("list")
                .about("Lists all the running mock servers")
                .setting(AppSettings::ColoredHelp))
        .subcommand(SubCommand::with_name("create")
                .about("Creates a new mock server from a pact file")
                .arg(Arg::with_name("file")
                    .short("f")
                    .long("file")
                    .takes_value(true)
                    .use_delimiter(false)
                    .required(true)
                    .help("the pact file to define the mock server"))
                .setting(AppSettings::ColoredHelp));

    let matches = app.get_matches_safe();
    match matches {
        Ok(ref matches) => {
            setup_loggers(matches.value_of("loglevel").unwrap_or("info"),
                matches.subcommand_name().unwrap(), matches.subcommand().1.unwrap().value_of("output"));
            let port = matches.value_of("port").unwrap_or("8080");
            let host = matches.value_of("host").unwrap_or("localhost");
            match port.parse::<u16>() {
                Ok(p) => {
                    match matches.subcommand() {
                        ("start", Some(_)) => server::start_server(p),
                        ("list", Some(sub_matches)) => list::list_mock_servers(host, p, sub_matches),
                        ("create", Some(sub_matches)) => create_mock::create_mock_server(host, p, sub_matches),
                        _ => ()
                    }
                },
                Err(_) => display_error(format!("{} is not a valid port number", port), matches)
            }
        },
        Err(ref err) => {
            match err.kind {
                ErrorKind::HelpDisplayed => (),
                ErrorKind::VersionDisplayed => print_version(),
                _ => err.exit()
            }
        }
    }
}
