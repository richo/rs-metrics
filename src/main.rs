#![feature(plugin)]
#![plugin(regex_macros)]
extern crate regex;
extern crate getopts;

use getopts::Options;
use std::env;
use regex::Regex;


static STATSRE: Regex = regex!("^(?P<name>[^:]+):(?P<value>[^|]+)\\|(?P<type>.*)$");

struct Metric<'a> {
    name: &'a str,
    value: f64,
}

struct Config {
    listen: String,
    send: String,
    relay: String,
    key: String,
}

fn print_usage(program: &str, opts: Options) {
        let brief = format!("Usage: {} [options]", program);
        print!("{}", opts.usage(&brief));
}

macro_rules! extract {
    ($opt:expr, $matches:expr, $ret:expr) => (
        if let Some(o) = $matches.opt_str($opt) {
            o
        } else {
            return Err($ret);
        }
    )
}

impl Config {
    fn parse(args: Vec<String>) -> Result<Config, Options> {
        let mut opts = Options::new();
        opts.optflag("h", "help", "print this help menu");
        opts.optopt("l", "listen", "address to listen on", "LISTEN");
        opts.optopt("s", "send", "address to send to", "SEND");
        opts.optopt("r", "relay", "address to relay on", "RELAY");
        opts.optopt("k", "key", "api key", "KEY");

        let matches = match opts.parse(&args[1..]) {
            Ok(m) => { m }
            Err(f) => { panic!(f.to_string()) }
        };

        if matches.opt_present("h") {
            return Err(opts);
        }

        let listen = extract!("l", matches, opts);
        let send = extract!("s", matches, opts);
        let relay = extract!("r", matches, opts);
        let key = extract!("k", matches, opts);

        Ok(Config {
            listen: listen,
            send: send,
            relay: relay,
            key: key,
        })
    }
}



fn main() {
    match Config::parse(env::args().collect()) {
        Ok(opts) => {
        },
        Err(opts) => {
            print_usage("rs-metrics", opts);
        }
    }
}
