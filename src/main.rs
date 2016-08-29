extern crate fern;
#[macro_use(error, info, log, warn)]
extern crate log;
extern crate rustc_serialize;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use rustc_serialize::json;

fn main() {
    let logger_config = fern::DispatchConfig {
        format: Box::new(|msg: &str, level: &log::LogLevel, _location: &log::LogLocation| {
            format!("[{:>5}] {}", level, msg)
        }),
        output: vec![fern::OutputConfig::stdout()],
        level: log::LogLevelFilter::Info,
    };
    let log = fern::init_global_logger(logger_config, log::LogLevelFilter::Info);
    if let Err(e) = log {
        panic!("error initializing log: {}", e);
    }

    let mut cache = match load_cache("collatz.cache") {
        Ok(c) => c,
        Err(e) => {
            warn!("unable to load cache: {:?}", e.description());
            info!("using default cache: {{1}}");
            let mut c = BTreeSet::new();
            c.insert(1);

            c
        }
    };

    for i in 1..100 {
        if !converges(i, &mut cache) {
            error!("found a divergent sequence for {}", i);
            break;
        }
    }

    match store_cache(&cache, "collatz.cache") {
        Ok(_) => info!("stored cache in \"collatz.cache\""),
        Err(e) => error!("unable to store cache: {}", e.description()),
    }
}

fn collatz(number: u64) -> u64 {
    match number % 2 == 0 {
        false => 3 * number + 1,
        true => number / 2,
    }
}

fn converges(number: u64, cache: &mut BTreeSet<u64>) -> bool {
    let mut seen = Vec::new();
    let mut current = number;
    loop {
        if cache.contains(&current) {
            seen.push(current);
            info!("  {}: {:?} (converged)", number, seen);
            cache.extend(seen);
            return true;
        }

        if seen.contains(&current) {
            seen.push(current);
            error!("! {}: {:?} (diverged)", number, seen);
            return false;
        }

        seen.push(current);
        current = collatz(current);
    }
}

// ---------------------------------------------------------------------

fn load_cache<P>(path: P) -> Result<BTreeSet<u64>, CacheError> where P: AsRef<Path> {
    let mut data = String::new();
    let mut file = try!(File::open(path));
    try!(file.read_to_string(&mut data));

    let cache = try!(json::decode(&data));

    Ok(cache)
}

fn store_cache<P>(cache: &BTreeSet<u64>, path: P) -> Result<(), CacheError> where P: AsRef<Path> {
    let data = try!(json::encode(cache));

    let mut file = try!(File::create(path));
    try!(file.write(data.as_bytes()));

    Ok(())
}

// ---------------------------------------------------------------------

#[derive(Debug)]
enum CacheError {
    Io(io::Error),
    Decode(json::DecoderError),
    Encode(json::EncoderError),
}

impl Display for CacheError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match *self {
            CacheError::Io(ref err) => write!(f, "I/O error: {}", err),
            CacheError::Decode(ref err) => write!(f, "decode error: {}", err),
            CacheError::Encode(ref err) => write!(f, "encode error: {}", err),
        }
    }
}

impl Error for CacheError {
    fn description(&self) -> &str {
        match *self {
            CacheError::Io(ref err) => err.description(),
            CacheError::Decode(ref err) => err.description(),
            CacheError::Encode(ref err) => err.description(),
        }
    }
}

impl From<io::Error> for CacheError {
    fn from(err: io::Error) -> CacheError {
        CacheError::Io(err)
    }
}

impl From<json::EncoderError> for CacheError {
    fn from(err: json::EncoderError) -> CacheError {
        CacheError::Encode(err)
    }
}

impl From<json::DecoderError> for CacheError {
    fn from(err: json::DecoderError) -> CacheError {
        CacheError::Decode(err)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn collatz_step() {
        assert_eq!(collatz(7), 22);
        assert_eq!(collatz(22), 11);
        assert_eq!(collatz(11), 34);
        assert_eq!(collatz(34), 17);
        assert_eq!(collatz(17), 52);
        assert_eq!(collatz(52), 26);
        assert_eq!(collatz(26), 13);
        assert_eq!(collatz(13), 40);
        assert_eq!(collatz(40), 20);
        assert_eq!(collatz(20), 10);
        assert_eq!(collatz(10), 5);
        assert_eq!(collatz(5), 16);
        assert_eq!(collatz(16), 8);
        assert_eq!(collatz(8), 4);
        assert_eq!(collatz(4), 2);
        assert_eq!(collatz(2), 1);
    }
}
