extern crate getopts;
use getopts::Options;
use std::env;

static SPEC_VERSION: &'static str = "1.0.0";

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
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

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let opts = setup_options();
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => {
            println!("ERROR: {}", f);
            print_usage(&program, opts);
            println!("");
            panic!(f.to_string())
        }
    };
    if args.len() == 1 || matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    if matches.opt_present("v") {
        print_version();
        return;
    }
}
