extern crate getopts;
extern crate pact_v1_matching;
extern crate pact_v1_mock_server;
#[macro_use] extern crate rustful;
#[macro_use] extern crate p_macro;
#[macro_use] extern crate log;
extern crate env_logger;
extern crate uuid;

use getopts::{Options, Matches};
use std::env;

mod server;

static SPEC_VERSION: &'static str = "1.0.0";
const COMMANDS: [&'static str; 1] = ["start"];

fn print_general_usage(program: String, opts: Options) {
    let brief = format!("Usage: {} [options] command", program);
    println!("{}", opts.usage(&brief));
    println!(r#"Valid commands are:
start - start a new master mock server

To get more details on any command, run '{} <command> -h' replacing <command> with the command you want details on.
"#, program);
}

fn print_command_usage(program: String, opts: Options, matches: Matches, command: String) {
    match command.as_str() {
        "start" => {
            let brief = format!("Usage: {} [options] start", program);
            println!("{}", opts.usage(&brief));
        },
        _ => ()
    }
}

fn print_usage(program: String, opts: Options, matches: Option<Matches>) {
    let command = match matches.clone() {
        None => None,
        Some(m) => {
            let args = m.free;
            if args.is_empty() || !COMMANDS.contains(&&*args[0]) {
                None
            } else {
                Some(args[0].clone())
            }
        }
    };
    if command.is_none() {
        print_general_usage(program, opts);
    } else {
        print_command_usage(program, opts, matches.unwrap(), command.unwrap());
    }
}

fn setup_options() -> Options {
    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("v", "version", "print the mock server version");
    opts
}

fn print_version() {
    const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
    println!("pact mock server version  : v{}", VERSION.unwrap_or("unknown"));
    println!("pact specification version: v{}", SPEC_VERSION);
}

fn display_error(error: String, program: String, opts: Options) -> ! {
    println!("ERROR: {}", error);
    print_usage(program, opts, None);
    println!("");
    panic!("{}", error)
}

fn execute_command(command: &String, matches: &Matches) {
    match command.as_str() {
        "start" => server::start_command(),
        _ => ()
    }
}

fn main() {
    env_logger::init().unwrap();

    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let opts = setup_options();
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => display_error(f.to_string(), program, opts)
    };
    if matches.free.is_empty() || matches.opt_present("h") {
        print_usage(program, opts, Some(matches));
        std::process::exit(1);
    } else if !COMMANDS.contains(&&*matches.free[0]) {
        display_error(format!("{} is not a valid command.", matches.free[0].clone()), program, opts);
    }

    if matches.opt_present("v") {
        print_version();
    } else {
        execute_command(&matches.free[0], &matches);
    }
}
