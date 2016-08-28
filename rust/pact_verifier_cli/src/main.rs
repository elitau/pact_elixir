//! The `pact_verifier_cli` crate provides the CLI for verification of service providers based on
//! pact files. It implements the V1 Pact specification
//! (https://github.com/pact-foundation/pact-specification/tree/version-1).

#![warn(missing_docs)]

#[macro_use] extern crate clap;
#[macro_use] extern crate p_macro;
#[macro_use] extern crate log;
#[macro_use] extern crate maplit;
#[macro_use] extern crate pact_matching;

#[cfg(test)]
#[macro_use(expect)]
extern crate expectest;

use std::env;
use clap::{Arg, App, SubCommand, AppSettings, ErrorKind, ArgMatches};
use pact_matching::models::PactSpecification;

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

fn handle_command_args() -> Result<(), i32> {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let version = format!("v{}", crate_version!());
    let app = App::new(program)
        .version(version.as_str())
        .about("Standalone Pact verifier")
        .version_short("v")
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::SubcommandRequired)
        .setting(AppSettings::GlobalVersion)
        .setting(AppSettings::VersionlessSubcommands)
        .setting(AppSettings::ColoredHelp)
        // .arg(Arg::with_name("port")
        //     .short("p")
        //     .long("port")
        //     .takes_value(true)
        //     .use_delimiter(false)
        //     .global(true)
        //     .help("port the master mock server runs on (defaults to 8080)"))
        // .arg(Arg::with_name("host")
        //     .short("h")
        //     .long("host")
        //     .takes_value(true)
        //     .use_delimiter(false)
        //     .global(true)
        //     .help("hostname the master mock server runs on (defaults to localhost)"))
        // .arg(Arg::with_name("loglevel")
        //     .short("l")
        //     .long("loglevel")
        //     .takes_value(true)
        //     .use_delimiter(false)
        //     .global(true)
        //     .possible_values(&["error", "warn", "info", "debug", "trace", "none"])
        //     .help("Log level for mock servers to write to the log file (defaults to info)"))
        // .subcommand(SubCommand::with_name("start")
        //         .about("Starts the master mock server")
        //         .setting(AppSettings::ColoredHelp)
        //         .arg(Arg::with_name("output")
        //               .short("o")
        //               .long("output")
        //               .takes_value(true)
        //               .use_delimiter(false)
        //               .help("the directory where to write files to (defaults to current directory)")))
        // .subcommand(SubCommand::with_name("list")
        //         .about("Lists all the running mock servers")
        //         .setting(AppSettings::ColoredHelp))
        // .subcommand(SubCommand::with_name("create")
        //         .about("Creates a new mock server from a pact file")
        //         .arg(Arg::with_name("file")
        //             .short("f")
        //             .long("file")
        //             .takes_value(true)
        //             .use_delimiter(false)
        //             .required(true)
        //             .help("the pact file to define the mock server"))
        //         .setting(AppSettings::ColoredHelp))
        // .subcommand(SubCommand::with_name("verify")
        //         .about("Verify the mock server by id or port number, and generate a pact file if all ok")
        //         .arg(Arg::with_name("mock-server-id")
        //             .short("i")
        //             .long("mock-server-id")
        //             .takes_value(true)
        //             .use_delimiter(false)
        //             .required_unless("mock-server-port")
        //             .conflicts_with("mock-server-port")
        //             .help("the ID of the mock server")
        //             .validator(uuid_value))
        //         .arg(Arg::with_name("mock-server-port")
        //             .short("m")
        //             .long("mock-server-port")
        //             .takes_value(true)
        //             .use_delimiter(false)
        //             .required_unless("mock-server-host")
        //             .help("the port number of the mock server")
        //             .validator(integer_value))
        //         .setting(AppSettings::ColoredHelp))
        // .subcommand(SubCommand::with_name("shutdown")
        //         .about("Shutdown the mock server by id or port number, releasing all its resources")
        //         .arg(Arg::with_name("mock-server-id")
        //             .short("i")
        //             .long("mock-server-id")
        //             .takes_value(true)
        //             .use_delimiter(false)
        //             .required_unless("mock-server-port")
        //             .conflicts_with("mock-server-port")
        //             .help("the ID of the mock server")
        //             .validator(uuid_value))
        //         .arg(Arg::with_name("mock-server-port")
        //             .short("m")
        //             .long("mock-server-port")
        //             .takes_value(true)
        //             .use_delimiter(false)
        //             .required_unless("mock-server-host")
        //             .help("the port number of the mock server")
        //             .validator(integer_value))
        //         .setting(AppSettings::ColoredHelp))
        ;

    let matches = app.get_matches_safe();
    match matches {
        Ok(ref matches) => {
            // let log_level = lookup_global_option("loglevel", matches);
            // if let Err(err) = setup_loggers(log_level.unwrap_or("info"),
            //     matches.subcommand_name().unwrap(),
            //     matches.subcommand().1.unwrap().value_of("output")) {
            //     display_error(format!("Could not setup loggers: {}", err), matches);
            // }
            // let port = lookup_global_option("port", matches).unwrap_or("8080");
            // let host = lookup_global_option("host", matches).unwrap_or("localhost");
            // match port.parse::<u16>() {
            //     Ok(p) => {
            //         match matches.subcommand() {
            //             ("start", Some(sub_matches)) => {
            //                 server::start_server(p, sub_matches.value_of("output").map(|s| s.to_owned()))
            //             },
            //             ("list", Some(sub_matches)) => list::list_mock_servers(host, p, sub_matches),
            //             ("create", Some(sub_matches)) => create_mock::create_mock_server(host, p, sub_matches),
            //             ("verify", Some(sub_matches)) => verify::verify_mock_server(host, p, sub_matches),
            //             ("shutdown", Some(sub_matches)) => shutdown::shutdown_mock_server(host, p, sub_matches),
            //             _ => Err(3)
            //         }
            //     },
            //     Err(_) => display_error(format!("{} is not a valid port number", port), matches)
            // }
            Ok(())
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
