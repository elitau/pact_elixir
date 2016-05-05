#[macro_use] extern crate clap;
#[macro_use] extern crate pact_v1_matching;
extern crate pact_v1_mock_server;
#[macro_use] extern crate rustful;
#[macro_use] extern crate p_macro;
#[macro_use] extern crate log;
#[macro_use] extern crate maplit;
extern crate env_logger;
extern crate uuid;
extern crate rustc_serialize;
#[macro_use] extern crate hyper;

use clap::{Arg, App, SubCommand, AppSettings, ErrorKind, ArgMatches};
use std::env;

mod server;

static SPEC_VERSION: &'static str = "1.0.0";

fn print_version() {
    println!("\npact mock server version  : v{}", crate_version!());
    println!("pact specification version: v{}", SPEC_VERSION);
}

fn display_error(error: String, matches: &ArgMatches) -> ! {
    println!("ERROR: {}", error);
    println!("");
    println!("{}", matches.usage());
    panic!("{}", error)
}

fn main() {
    env_logger::init().unwrap();

    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let version = format!("v{}", crate_version!());
    let app = App::new(program)
        .version(version.as_str())
        .about("Standalone Pact mock server")
        .version_short("v")
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::GlobalVersion)
        .setting(AppSettings::VersionlessSubcommands)
        .setting(AppSettings::ColoredHelp)
        .subcommand(SubCommand::with_name("start")
                  .about("Starts the main mock server")
                  .setting(AppSettings::ColoredHelp)
                  .arg(Arg::with_name("port")
                      .short("p")
                      .long("port")
                      .takes_value(true)
                      .use_delimiter(false)
                      .help("port to run on (defaults to 8080)"))
        );

    let matches = app.get_matches_safe();
    match matches {
        Ok(ref matches) => {
            match matches.subcommand() {
                ("start", Some(sub_matches)) => {
                    let port = sub_matches.value_of("port").unwrap_or("8080");
                    match port.parse::<u16>() {
                        Ok(p) => server::start_server(p),
                        Err(err) => display_error(format!("{} is not a valid port number", port), sub_matches)
                    }
                },
                _ => ()
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
